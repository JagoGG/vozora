import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { listen } from "@tauri-apps/api/event";
import { commands } from "@/bindings";
import { toast } from "sonner";

/**
 * Confirmation gate for "terminal-command" dictation mode (see
 * `settings::DictationMode::TerminalCommand` and `coding_mode::looks_destructive`
 * on the Rust side). When a transcription in that mode looks like a
 * destructive shell command, the backend withholds the auto-paste and emits
 * `pending-destructive-paste` with the text instead of pasting it. This
 * dialog shows the exact text and requires an explicit Paste/Discard choice.
 *
 * Vozora never executes shell commands itself in either app or backend code
 * — this only gates whether the transcribed text gets inserted via the
 * paste pipeline, as a safety net against a misheard "rm -rf" ending up in a
 * live terminal.
 */
export const DestructiveCommandDialog: React.FC = () => {
  const { t } = useTranslation();
  const [pendingText, setPendingText] = useState<string | null>(null);
  const [isBusy, setIsBusy] = useState(false);

  useEffect(() => {
    const unlisten = listen<string>("pending-destructive-paste", (event) => {
      setPendingText(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (pendingText === null) return null;

  const handleConfirm = async () => {
    setIsBusy(true);
    try {
      const result = await commands.confirmPendingPaste();
      if (result.status === "error") {
        toast.error(String(result.error));
      }
    } finally {
      setIsBusy(false);
      setPendingText(null);
    }
  };

  const handleCancel = async () => {
    setIsBusy(true);
    try {
      await commands.cancelPendingPaste();
    } finally {
      setIsBusy(false);
      setPendingText(null);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
      <div className="max-w-lg w-full rounded-lg border border-border bg-background p-5 space-y-4 shadow-xl">
        <h2 className="text-base font-semibold text-text">
          {t(
            "dictation.destructiveConfirm.title",
            "This looks like a destructive command",
          )}
        </h2>
        <p className="text-sm text-text/70">
          {t(
            "dictation.destructiveConfirm.description",
            "Vozora paused before inserting this text because it matched a pattern often used in destructive shell commands (e.g. deleting files, force-pushing, dropping a database). Vozora never runs commands itself — review the text below and choose whether to paste it.",
          )}
        </p>
        <pre className="max-h-40 overflow-auto rounded-md bg-mid-gray/10 border border-mid-gray/30 p-3 text-xs font-mono whitespace-pre-wrap break-words">
          {pendingText}
        </pre>
        <div className="flex justify-end gap-2">
          <button
            className="px-3 py-1.5 text-sm rounded border border-border hover:bg-border/50 transition-colors disabled:opacity-50"
            onClick={handleCancel}
            disabled={isBusy}
          >
            {t("dictation.destructiveConfirm.discard", "Discard")}
          </button>
          <button
            className="px-3 py-1.5 text-sm rounded bg-red-600 text-white hover:bg-red-700 transition-colors disabled:opacity-50"
            onClick={handleConfirm}
            disabled={isBusy}
          >
            {t("dictation.destructiveConfirm.paste", "Paste anyway")}
          </button>
        </div>
      </div>
    </div>
  );
};
