import type { StatsConfig, SystemField } from "../useStats";

interface Props {
  available: SystemField[];
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function FieldToggles({ available, config, onChange }: Props) {
  // Selection is re-derived from the backend's order, so the widget's field
  // order stays stable no matter what order the boxes were clicked in.
  function toggle(id: string, checked: boolean) {
    const selected = new Set(config.system_fields);
    if (checked) {
      selected.add(id);
    } else {
      selected.delete(id);
    }
    onChange({
      ...config,
      system_fields: available.filter((field) => selected.has(field.id)).map((field) => field.id),
    });
  }
  return (
    <fieldset className="settingsGroup">
      <legend className="settingsGroupTitle">System</legend>
      {available.map((field) => (
        <div className="settingsRow" key={field.id}>
          <label className="settingsRowLabel" htmlFor={`field-${field.id}`}>
            {field.label}
          </label>
          <input
            id={`field-${field.id}`}
            type="checkbox"
            checked={config.system_fields.includes(field.id)}
            onChange={(event) => toggle(field.id, event.currentTarget.checked)}
          />
        </div>
      ))}
      <div className="settingsRow">
        <label className="settingsRowLabel" htmlFor="field-cpu-cores">
          Show CPU cores
        </label>
        <input
          id="field-cpu-cores"
          type="checkbox"
          checked={config.show_cpu_cores}
          onChange={(event) =>
            onChange({ ...config, show_cpu_cores: event.currentTarget.checked })
          }
        />
      </div>
    </fieldset>
  );
}
