import React from "react";
import { useTranslation } from "react-i18next";
import { SettingContainer } from "../ui/SettingContainer";
import { Dropdown, type DropdownOption } from "../ui/Dropdown";
import { useSettings } from "../../hooks/useSettings";
import { commands, type DictationMode } from "@/bindings";
import { toast } from "sonner";

const DICTATION_MODE_OPTIONS: DropdownOption[] = [
  { value: "literal", label: "Literal (paste exactly what was said)" },
  { value: "natural_corrected", label: "Natural / corrected (AI cleanup)" },
  { value: "ai_prompt", label: "AI prompt (for chat boxes, e.g. Claude Code)" },
  { value: "code", label: "Code (spoken punctuation -> symbols, for editors)" },
  {
    value: "terminal_command",
    label: "Terminal command (code mode + destructive-command confirmation)",
  },
];

interface DictationModeSelectorProps {
  descriptionMode?: "tooltip" | "inline";
  grouped?: boolean;
}

/**
 * Global default dictation mode. Per-application overrides (Cursor, a
 * terminal, etc.) live in `AppProfilesSettings` and take precedence over
 * this when the focused window matches one of their patterns — see
 * `app_profile::resolve_dictation_settings` on the Rust side.
 */
export const DictationModeSelector: React.FC<DictationModeSelectorProps> = ({
  descriptionMode = "tooltip",
  grouped = false,
}) => {
  const { t } = useTranslation();
  const { getSetting, isUpdating, refreshSettings } = useSettings();
  const currentMode = getSetting("dictation_mode") ?? "literal";

  const handleSelect = async (value: string) => {
    if (value === currentMode) return;
    try {
      await commands.changeDictationModeSetting(value as DictationMode);
      await refreshSettings();
    } catch (error) {
      console.error("Failed to update dictation mode:", error);
      toast.error(String(error));
    }
  };

  return (
    <SettingContainer
      title={t("settings.dictation.mode.title", "Dictation mode")}
      description={t(
        "settings.dictation.mode.description",
        "How Vozora treats a transcription before pasting it. Per-app overrides in App Profiles below take precedence.",
      )}
      descriptionMode={descriptionMode}
      grouped={grouped}
      layout="horizontal"
    >
      <Dropdown
        options={DICTATION_MODE_OPTIONS}
        selectedValue={currentMode}
        onSelect={handleSelect}
        disabled={isUpdating("dictation_mode")}
      />
    </SettingContainer>
  );
};
