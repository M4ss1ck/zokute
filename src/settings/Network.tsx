import type { MergedConfig } from "../useStats";
import { HeaderToggle } from "./Header";

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

export function NetworkPreferences({ config, onChange }: Props) {
  return (
    <section className="settingsCard" aria-labelledby="network-title">
      <header className="settingsCardHeader">
        <h2 id="network-title">Network</h2>
        <p>Choose how network activity is displayed.</p>
      </header>
      <div className="settingsCardBody">
        <HeaderToggle id="network" config={config} onChange={onChange} />
      </div>
    </section>
  );
}
