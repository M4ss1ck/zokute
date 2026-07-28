import { Input, TextField } from "react-aria-components";
import type { Stats, MergedConfig } from "../useStats";
import { HeaderToggle } from "./Header";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  detected: Stats["disks"];
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

export function DiskPreferences({ detected, config, onChange }: Props) {
  function replace(id: string, changes: { enabled?: boolean; label?: string | null }) {
    onChange({
      ...config,
      disks: config.disks.map((disk) => (disk.id === id ? { ...disk, ...changes } : disk)),
    });
  }
  return (
    <section className="settingsCard" aria-labelledby="disks-title">
      <header className="settingsCardHeader">
        <h2 id="disks-title">Disks</h2>
        <p>Select volumes and give them compact display names.</p>
      </header>
      <div className="settingsCardBody">
        <HeaderToggle id="disk" config={config} onChange={onChange} />
        {config.disks.map((disk) => {
          const mounted = detected.find((candidate) => candidate.id === disk.id);
          const name = mounted?.mount ?? disk.label ?? disk.id;
          return (
            <div className="settingsDisk" key={disk.id}>
              <SettingSwitch
                isSelected={disk.enabled}
                onChange={(enabled) => replace(disk.id, { enabled })}
              >
                {`Show ${name}`}
              </SettingSwitch>
              <TextField
                className="settingsTextField"
                aria-label={`Label for ${name}`}
                value={disk.label ?? ""}
                onChange={(label) => replace(disk.id, { label: label.trim() || null })}
              >
                <Input
                  className="settingsTextInput"
                  placeholder={mounted?.name ?? "Display label"}
                />
              </TextField>
            </div>
          );
        })}
      </div>
    </section>
  );
}
