import type { StatsConfig } from "../useStats";

interface Props {
  config: StatsConfig;
  onChange: (next: StatsConfig) => void;
}

export function PanelPreferences({ config, onChange }: Props) {
  const panels = config.sections.filter((s) => s.id === "panel");
  return (
    <section className="settingsCard" aria-labelledby="panels-title">
      <header className="settingsCardHeader">
        <h2 id="panels-title">Panels</h2>
        <p>Manage panel widgets containing multiple children.</p>
      </header>
      {panels.length === 0 && (
        <p className="settingsEmpty">No panels configured.</p>
      )}
    </section>
  );
}
