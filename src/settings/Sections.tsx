import type { SectionConfig, StatsConfig } from "../useStats";

interface Props {
  config: StatsConfig;
  monitorCount: number;
  onChange: (next: StatsConfig) => void;
}

export function SectionToggles({ config, monitorCount, onChange }: Props) {
  function replace(id: string, changes: Partial<SectionConfig>) {
    onChange({
      ...config,
      sections: config.sections.map((section) =>
        section.id === id ? { ...section, ...changes } : section,
      ),
    });
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
            onChange={(event) => replace(section.id, { enabled: event.currentTarget.checked })}
          />
          <select
            aria-label={`Monitor for ${section.id}`}
            value={section.monitor}
            onChange={(event) => replace(section.id, { monitor: Number(event.currentTarget.value) })}
          >
            {Array.from({ length: monitorCount }, (_, index) => (
              <option key={index} value={index}>
                Monitor {index + 1}
              </option>
            ))}
          </select>
        </div>
      ))}
    </fieldset>
  );
}
