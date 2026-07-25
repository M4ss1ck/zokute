import type { StatsConfig } from "../useStats";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  id: string;
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function HeaderToggle({ id, config, onChange }: Props) {
  const section = config.sections.find((candidate) => candidate.id === id);
  return (
    <SettingSwitch
      isSelected={section?.show_header ?? true}
      onChange={(show_header) =>
        onChange({
          ...config,
          sections: config.sections.map((candidate) =>
            candidate.id === id ? { ...candidate, show_header } : candidate,
          ),
        })
      }
    >
      Show header
    </SettingSwitch>
  );
}
