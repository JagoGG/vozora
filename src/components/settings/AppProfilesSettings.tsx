import React from "react";
import { useTranslation } from "react-i18next";
import { Dropdown, type DropdownOption } from "../ui/Dropdown";
import { useSettings } from "../../hooks/useSettings";
import { commands, type AppProfile, type DictationMode } from "@/bindings";
import { toast } from "sonner";

const DICTATION_MODE_OPTIONS: DropdownOption[] = [
  { value: "literal", label: "Literal" },
  { value: "natural_corrected", label: "Natural / corrected" },
  { value: "ai_prompt", label: "AI prompt" },
  { value: "code", label: "Code" },
  { value: "terminal_command", label: "Terminal command" },
];

interface AppProfilesSettingsProps {
  grouped?: boolean;
}

/**
 * Per-application dictation-mode overrides, matched by a substring of the
 * focused window's title (see `settings::AppProfile` /
 * `app_profile::match_profile` on the Rust side). Ships with defaults for
 * Cursor, Windows Terminal, kitty, and Claude Code running in a terminal;
 * this view lets the user change the mode each profile applies without
 * hand-editing the settings store.
 *
 * Adding/removing profiles or editing window-title patterns isn't exposed in
 * this UI yet — that's a reasonable next increment, tracked in
 * docs/ARCHITECTURE.md.
 */
export const AppProfilesSettings: React.FC<AppProfilesSettingsProps> = ({
  grouped = false,
}) => {
  const { t } = useTranslation();
  const { getSetting, refreshSettings } = useSettings();
  const profiles: AppProfile[] = getSetting("app_profiles") ?? [];

  const handleModeChange = async (index: number, mode: string) => {
    const updated = profiles.map((profile, i) =>
      i === index
        ? { ...profile, dictation_mode: mode as DictationMode }
        : profile,
    );
    try {
      await commands.changeAppProfilesSetting(updated);
      await refreshSettings();
    } catch (error) {
      console.error("Failed to update app profile:", error);
      toast.error(String(error));
    }
  };

  if (profiles.length === 0) {
    return (
      <div className={grouped ? "" : "max-w-3xl w-full mx-auto"}>
        <p className="text-sm text-mid-gray">
          {t(
            "settings.dictation.appProfiles.empty",
            "No app profiles configured.",
          )}
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      {profiles.map((profile, index) => (
        <div
          key={profile.name}
          className="flex items-center justify-between gap-4 rounded-md border border-mid-gray/30 px-3 py-2"
        >
          <div>
            <div className="text-sm font-semibold text-text">
              {profile.name}
            </div>
            <div className="text-xs text-mid-gray font-mono">
              {profile.window_title_patterns.join(", ")}
            </div>
          </div>
          <Dropdown
            options={DICTATION_MODE_OPTIONS}
            selectedValue={profile.dictation_mode}
            onSelect={(value) => handleModeChange(index, value)}
          />
        </div>
      ))}
    </div>
  );
};
