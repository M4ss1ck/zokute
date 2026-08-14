import { type ComponentType } from "react";
import type { SectionConfig, Stats, StatsHistory } from "../useStats";
import { CpuWidget } from "./Cpu";
import { DiskWidget } from "./Disk";
import { MemoryWidget } from "./Memory";
import { NetworkWidget } from "./Network";
import { SystemWidget } from "./System";
import { ClockWidget } from "./Clock";
import { DateWidget } from "./Date";
import { PluginWidget } from "./Plugin";
import { WidgetErrorBoundary } from "./WidgetErrorBoundary";

type WidgetProps = { stats: Stats; history: StatsHistory; section: SectionConfig };

const widgetMap: Record<string, ComponentType<WidgetProps>> = {
  system: SystemWidget, cpu: CpuWidget, memory: MemoryWidget,
  disk: DiskWidget, network: NetworkWidget,
  clock: ClockWidget, date: DateWidget, plugin: PluginWidget,
};

interface Props {
  stats: Stats;
  history: StatsHistory;
  section: SectionConfig;
}

export function PanelWidget({ stats, history, section }: Props) {
  const gap = section.panel_gap ?? 4;
  const padding = section.panel_padding ?? 8;
  const dividers = section.panel_dividers ?? true;

  return (
    <div
      className={["panelDashboard", dividers ? "panelDashboard--dividers" : ""].filter(Boolean).join(" ")}
      style={{ gap: `${gap}px`, padding: `${padding}px` }}
    >
      {section.children!.map((child) => {
        const Widget = widgetMap[child.id];
        if (!Widget) return null;
        return (
          <div key={child.instance ?? child.id} className="panelChild">
            <WidgetErrorBoundary instance={child.instance ?? child.id}>
              <Widget stats={stats} history={history} section={child} />
            </WidgetErrorBoundary>
          </div>
        );
      })}
    </div>
  );
}
