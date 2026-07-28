import type { SectionConfig } from "../useStats";

interface Props {
  section: SectionConfig;
}

export function PluginWidget({ section }: Props) {
  return (
    <div className="pluginWidget" data-testid="plugin-widget">
      <div className="pluginWidgetHeader">{section.plugin_id ?? "Plugin"}</div>
    </div>
  );
}
