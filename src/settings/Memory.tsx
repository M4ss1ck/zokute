import type { StatsConfig } from "../useStats";
import { HeaderToggle } from "./Header";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function MemoryPreferences({ config, onChange }: Props) {
  return (
    <section className="settingsCard" aria-labelledby="memory-title">
      <header className="settingsCardHeader">
        <h2 id="memory-title">Memory</h2>
        <p>Choose how memory usage is displayed.</p>
      </header>
      <div className="settingsCardBody">
        <HeaderToggle id="memory" config={config} onChange={onChange} />
      </div>
    </section>
  );
}
