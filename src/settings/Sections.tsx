import type { SectionConfig, StatsConfig } from "../useStats";

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
    <fieldset className="settingsGroup">
      <legend className="settingsGroupTitle">Widgets</legend>
      {config.sections.map((section) => (
        <div className="settingsRow" key={section.id}>
          <label className="settingsRowLabel" htmlFor={`section-${section.id}`}>
            Show {section.id}
          </label>
          <input
            id={`section-${section.id}`}
            type="checkbox"
            checked={section.enabled}
            onChange={(event) => onChange(replaced(section.id, { enabled: event.currentTarget.checked }))}
          />
        </div>
      ))}
    </fieldset>
  );
}
