import type { SectionConfig, StatsConfig } from "../useStats";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function SectionToggles({ config, onChange }: Props) {
  function replaced(id: string, changes: Partial<SectionConfig>) {
    return {
      ...config,
      sections: config.sections.map((section) =>
        section.id === id ? { ...section, ...changes } : section,
      ),
    };
  }
  return (
    <section className="settingsCard" aria-labelledby="widgets-title">
      <header className="settingsCardHeader">
        <h2 id="widgets-title">Widgets</h2>
        <p>Choose which monitors stay on your desktop.</p>
      </header>
      <div className="settingsCardBody">
        {config.sections.map((section) => (
          <SettingSwitch
            key={section.id}
            isSelected={section.enabled}
            onChange={(enabled) => onChange(replaced(section.id, { enabled }))}
          >
            {`Show ${section.id}`}
          </SettingSwitch>
        ))}
      </div>
    </section>
  );
}
