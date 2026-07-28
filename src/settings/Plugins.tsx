import type { MergedConfig } from "../useStats";

interface Props {
  config: MergedConfig;
  onChange: (next: MergedConfig) => void;
}

export function PluginPreferences({ config, onChange }: Props) {
  const pluginSections = config.sections.filter((s) => s.id === "plugin");
  return (
    <section className="settingsCard" aria-labelledby="plugins-title">
      <header className="settingsCardHeader">
        <h2 id="plugins-title">Plugins</h2>
        <p>Manage external plugin widgets.</p>
      </header>
      {pluginSections.length === 0 && (
        <p className="settingsEmpty">No plugin widgets configured.</p>
      )}
    </section>
  );
}
