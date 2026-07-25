import type { StatsConfig } from "../useStats";
import { HeaderToggle } from "./Header";
import { SettingSwitch } from "./SettingSwitch";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function CpuPreferences({ config, onChange }: Props) {
  return (
    <section className="settingsCard" aria-labelledby="cpu-title">
      <header className="settingsCardHeader">
        <h2 id="cpu-title">CPU</h2>
        <p>Choose how processor activity is displayed.</p>
      </header>
      <div className="settingsCardBody">
        <HeaderToggle id="cpu" config={config} onChange={onChange} />
        <SettingSwitch
          isSelected={config.show_cpu_cores}
          onChange={(show_cpu_cores) => onChange({ ...config, show_cpu_cores })}
        >
          Show CPU cores
        </SettingSwitch>
      </div>
    </section>
  );
}
