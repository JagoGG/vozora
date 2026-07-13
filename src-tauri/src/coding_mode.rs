//! "Coding Mode" dictation post-processing.
//!
//! Converts spoken punctuation/formatting commands ("open paren", "abre
//! paréntesis", "new line", "nueva línea", ...) into literal characters, so a
//! transcription like "call foo open paren bar close paren new line" becomes
//! "call foo(bar)\n".
//!
//! The phrase table is intentionally DATA-DRIVEN (a plain array of
//! `PhraseRule`s) rather than a chain of if/else branches, so it can later be
//! extended with user-defined custom phrases (e.g. loaded from settings/a
//! JSON file) without touching the matching algorithm.
//!
//! Matching works on whitespace-tokenized words: every rule's phrase is also
//! tokenized, and the transcription is scanned left-to-right for the longest
//! matching run of tokens (case-insensitive), which is replaced by the rule's
//! literal output. This lets multi-word phrases like "open paren" /
//! "abre paréntesis" be recognized without false-positiving on the individual
//! words "open" or "paren" appearing naturally in speech elsewhere — a rule
//! only fires when the *whole* phrase appears contiguously.

/// A single phrase -> literal mapping. `phrases` holds every spoken variant
/// (English + Spanish, plus casing/punctuation-free variants) that should
/// produce `output`.
#[derive(Debug, Clone)]
pub struct PhraseRule {
    pub phrases: &'static [&'static str],
    pub output: &'static str,
}

/// Default English + Spanish coding-mode phrase table.
///
/// Ordered so longer / more specific phrases are listed before shorter ones
/// that could be a prefix of them, since matching prefers the longest phrase
/// match at a given position (see [`apply_coding_mode`]).
pub const DEFAULT_PHRASE_TABLE: &[PhraseRule] = &[
    PhraseRule { phrases: &["open paren", "open parenthesis", "abre parentesis", "abre paréntesis"], output: "(" },
    PhraseRule { phrases: &["close paren", "close parenthesis", "cierra parentesis", "cierra paréntesis"], output: ")" },
    PhraseRule { phrases: &["open brace", "open curly brace", "abre llave"], output: "{" },
    PhraseRule { phrases: &["close brace", "close curly brace", "cierra llave"], output: "}" },
    PhraseRule { phrases: &["open bracket", "open square bracket", "abre corchete"], output: "[" },
    PhraseRule { phrases: &["close bracket", "close square bracket", "cierra corchete"], output: "]" },
    PhraseRule { phrases: &["open angle bracket", "less than", "abre angular", "menor que"], output: "<" },
    PhraseRule { phrases: &["close angle bracket", "greater than", "cierra angular", "mayor que"], output: ">" },
    PhraseRule { phrases: &["new line", "newline", "nueva linea", "nueva línea"], output: "\n" },
    PhraseRule { phrases: &["new paragraph", "nuevo parrafo", "nuevo párrafo"], output: "\n\n" },
    PhraseRule { phrases: &["tab", "tab key", "tabulacion", "tabulación"], output: "\t" },
    PhraseRule { phrases: &["semicolon", "punto y coma"], output: ";" },
    PhraseRule { phrases: &["colon", "dos puntos"], output: ":" },
    PhraseRule { phrases: &["comma", "coma"], output: "," },
    PhraseRule { phrases: &["period", "full stop", "punto"], output: "." },
    PhraseRule { phrases: &["dash", "hyphen", "guion", "guión"], output: "-" },
    PhraseRule { phrases: &["underscore", "guion bajo", "guión bajo"], output: "_" },
    PhraseRule { phrases: &["forward slash", "slash", "barra inclinada", "barra"], output: "/" },
    PhraseRule { phrases: &["backslash", "back slash", "barra invertida"], output: "\\" },
    PhraseRule { phrases: &["pipe", "vertical bar", "barra vertical"], output: "|" },
    PhraseRule { phrases: &["ampersand", "and sign", "ampersand comercial"], output: "&" },
    PhraseRule { phrases: &["asterisk", "star", "asterisco"], output: "*" },
    PhraseRule { phrases: &["percent sign", "percent", "signo de porcentaje", "por ciento"], output: "%" },
    PhraseRule { phrases: &["dollar sign", "dollar", "signo de dolar", "signo de dólar"], output: "$" },
    PhraseRule { phrases: &["hash", "pound sign", "hashtag", "almohadilla"], output: "#" },
    PhraseRule { phrases: &["at sign", "at symbol", "arroba"], output: "@" },
    PhraseRule { phrases: &["equals sign", "equals", "signo igual", "igual"], output: "=" },
    PhraseRule { phrases: &["plus sign", "plus", "signo mas", "signo más"], output: "+" },
    PhraseRule { phrases: &["quote", "double quote", "comilla", "comillas"], output: "\"" },
    PhraseRule { phrases: &["single quote", "apostrophe", "comilla simple"], output: "'" },
    PhraseRule { phrases: &["backtick", "acento grave"], output: "`" },
    PhraseRule { phrases: &["exclamation mark", "exclamation point", "signo de exclamacion", "signo de exclamación"], output: "!" },
    PhraseRule { phrases: &["question mark", "signo de interrogacion", "signo de interrogación"], output: "?" },
    PhraseRule { phrases: &["caret", "circumflex", "circunflejo"], output: "^" },
    PhraseRule { phrases: &["tilde", "virgulilla"], output: "~" },
    PhraseRule { phrases: &["space", "espacio"], output: " " },
];

/// One token of a phrase, lowercased, along with the literal it maps to when
/// matched contiguously. Built once from [`PhraseRule`]s for fast lookup.
struct CompiledRule {
    tokens: Vec<String>,
    output: String,
}

fn compile_table(table: &[PhraseRule]) -> Vec<CompiledRule> {
    let mut rules: Vec<CompiledRule> = table
        .iter()
        .flat_map(|rule| {
            rule.phrases.iter().map(move |phrase| CompiledRule {
                tokens: phrase.split_whitespace().map(|w| w.to_lowercase()).collect(),
                output: rule.output.to_string(),
            })
        })
        .collect();
    // Longest phrase (in tokens) first, so a longer match always wins over a
    // shorter one that happens to share a prefix.
    rules.sort_by(|a, b| b.tokens.len().cmp(&a.tokens.len()));
    rules
}

/// Outputs that "attach" tight to what comes both before and after them
/// (opening brackets): no space is inserted on either side.
const TIGHT_BOTH: &[&str] = &["(", "{", "[", "<"];

/// Outputs that attach tight to what comes *before* them but still want a
/// normal separating space after (closing brackets and terminal punctuation).
const TIGHT_LEFT: &[&str] = &[")", "}", "]", ">", ";", ",", ".", "!", "?", ":"];

/// Whether a "unit" (a plain word or a matched phrase's literal output)
/// suppresses the space that would otherwise be inserted before/after it.
fn spacing(output: &str) -> (bool /* no_space_before */, bool /* no_space_after */) {
    if output.chars().all(char::is_whitespace) && !output.is_empty() {
        // Newlines/tabs/space itself already are the separator.
        (true, true)
    } else if TIGHT_BOTH.contains(&output) {
        (true, true)
    } else if TIGHT_LEFT.contains(&output) {
        (true, false)
    } else {
        (false, false)
    }
}

/// Apply the given phrase table to `text`, replacing spoken punctuation
/// commands with their literal characters. Word matching is case-insensitive
/// and ignores punctuation the ASR model may have inserted around a token.
pub fn apply_coding_mode_with_table(text: &str, table: &[PhraseRule]) -> String {
    let compiled = compile_table(table);

    let words: Vec<String> = text.split_whitespace().map(|w| w.to_string()).collect();
    let normalized: Vec<String> = words
        .iter()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .collect();

    let mut out = String::new();
    let mut last_no_space_after = true; // nothing before the first unit
    let mut i = 0;
    while i < words.len() {
        let mut matched: Option<(usize, &str)> = None;

        for rule in &compiled {
            let len = rule.tokens.len();
            if i + len <= normalized.len() && normalized[i..i + len] == rule.tokens[..] {
                matched = Some((len, rule.output.as_str()));
                break; // compiled is sorted longest-first
            }
        }

        let (unit_text, advance): (&str, usize) = match matched {
            Some((len, output)) => (output, len),
            None => (words[i].as_str(), 1),
        };

        let (no_space_before, no_space_after) = spacing(unit_text);
        if !out.is_empty()
            && !last_no_space_after
            && !no_space_before
            && !out.ends_with(char::is_whitespace)
        {
            out.push(' ');
        }
        out.push_str(unit_text);
        last_no_space_after = no_space_after || unit_text.ends_with(char::is_whitespace);
        i += advance;
    }
    out
}

/// Apply the default built-in phrase table.
pub fn apply_coding_mode(text: &str) -> String {
    apply_coding_mode_with_table(text, DEFAULT_PHRASE_TABLE)
}

/// Regex-ish (hand-rolled, no external dependency needed) check for shell
/// commands that are very likely destructive. This is a UX confirmation gate
/// only — Vozora never executes anything itself, it only inserts text — but
/// surfacing a warning before auto-pasting a `rm -rf /` into a terminal is
/// cheap insurance against a misheard transcription.
///
/// Deliberately conservative (may have false positives) since the cost of an
/// extra confirmation click is far lower than the cost of a silent paste of
/// a destructive command into a live terminal.
pub fn looks_destructive(text: &str) -> bool {
    let lower = text.to_lowercase();

    const SUBSTRING_PATTERNS: &[&str] = &[
        "rm -rf",
        "rm -fr",
        "rm --recursive --force",
        "mkfs",
        "dd if=",
        ":(){:|:&};:",
        ":(){ :|:& };:",
        "drop table",
        "drop database",
        "truncate table",
        "git push --force",
        "git push -f",
        "git reset --hard",
        "> /dev/sda",
        "format c:",
        "del /f /s /q",
        "rd /s /q",
        "shutdown -r",
        "chmod -r 777 /",
        "chown -r",
        "wipefs",
    ];

    SUBSTRING_PATTERNS.iter().any(|pattern| lower.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_open_and_close_paren() {
        assert_eq!(
            apply_coding_mode("call foo open paren bar close paren"),
            "call foo(bar)"
        );
    }

    #[test]
    fn translates_new_line_and_semicolon() {
        assert_eq!(
            apply_coding_mode("let x equals sign 1 semicolon new line let y equals sign 2 semicolon"),
            "let x = 1;\nlet y = 2;"
        );
    }

    #[test]
    fn translates_spanish_phrases() {
        assert_eq!(
            apply_coding_mode("funcion abre parentesis x cierra parentesis abre llave"),
            "funcion(x){"
        );
    }

    #[test]
    fn leaves_unmatched_words_alone() {
        assert_eq!(
            apply_coding_mode("this is just normal speech"),
            "this is just normal speech"
        );
    }

    #[test]
    fn is_case_insensitive() {
        assert_eq!(apply_coding_mode("Open Paren Close Paren"), "()");
    }

    #[test]
    fn custom_phrase_table_can_extend_defaults() {
        let mut table: Vec<PhraseRule> = DEFAULT_PHRASE_TABLE.to_vec();
        table.push(PhraseRule {
            phrases: &["arrow function"],
            output: "=>",
        });
        assert_eq!(
            apply_coding_mode_with_table("const f arrow function", &table),
            "const f =>"
        );
    }

    #[test]
    fn detects_rm_rf() {
        assert!(looks_destructive("please run rm -rf / now"));
    }

    #[test]
    fn detects_fork_bomb() {
        assert!(looks_destructive(":(){:|:&};:"));
    }

    #[test]
    fn detects_drop_table() {
        assert!(looks_destructive("DROP TABLE users;"));
    }

    #[test]
    fn detects_force_push() {
        assert!(looks_destructive("git push --force origin main"));
    }

    #[test]
    fn ordinary_commands_are_not_flagged() {
        assert!(!looks_destructive("ls -la"));
        assert!(!looks_destructive("git status"));
        assert!(!looks_destructive("npm install"));
    }
}
