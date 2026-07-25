import type { Stats, StatsConfig } from "../useStats";

interface Props {
  detected: Stats["disks"];
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function DiskPreferences({ detected, config, onChange }: Props) {
  function replace(id: string, changes: { enabled?: boolean; label?: string | null }) {
    onChange({
      ...config,
      disks: config.disks.map((disk) => (disk.id === id ? { ...disk, ...changes } : disk)),
    });
  }
  return (
    <fieldset className="settingsGroup">
      <legend className="settingsGroupTitle">Disks</legend>
      {config.disks.map((disk) => {
        const mounted = detected.find((candidate) => candidate.id === disk.id);
        const name = mounted?.mount ?? disk.label ?? disk.id;
        return (
          <div className="settingsRow" key={disk.id}>
            <label className="settingsRowLabel" htmlFor={`disk-${disk.id}`}>
              Show {name}
            </label>
            <input
              id={`disk-${disk.id}`}
              type="checkbox"
              checked={disk.enabled}
              onChange={(event) => replace(disk.id, { enabled: event.currentTarget.checked })}
            />
            <input
              className="settingsTextInput"
              type="text"
              aria-label={`Label for ${name}`}
              placeholder={mounted?.name ?? ""}
              value={disk.label ?? ""}
              onChange={(event) =>
                replace(disk.id, { label: event.currentTarget.value.trim() || null })
              }
            />
          </div>
        );
      })}
    </fieldset>
  );
}
