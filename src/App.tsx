import type { CSSProperties, ComponentType } from "react";
import useStats, { type SectionConfig, type Stats, type StatsHistory } from "./useStats";
import { CpuWidget } from "./widgets/Cpu";
import { DiskWidget } from "./widgets/Disk";
import { MemoryWidget } from "./widgets/Memory";
import { NetworkWidget } from "./widgets/Network";
import { SystemWidget } from "./widgets/System";

type WidgetId = "system" | "cpu" | "memory" | "disk" | "network";
type WidgetEntry = { id: WidgetId; Component: ComponentType<{ stats: Stats; history: StatsHistory }> };

const widgets: readonly WidgetEntry[] = [
  { id: "system", Component: SystemWidget },
  { id: "cpu", Component: CpuWidget },
  { id: "memory", Component: MemoryWidget },
  { id: "disk", Component: DiskWidget },
  { id: "network", Component: NetworkWidget },
];

function isWidgetId(id: string): id is WidgetId {
  return widgets.some((widget) => widget.id === id);
}

export default function App() {
  const { stats, history } = useStats();
  if (!stats) return <main className="dashboard" aria-label="Zokute dashboard" />;
  const dashboardStyle = {
    ["--dashboard-opacity" as "--dashboard-opacity"]: String(stats.config.opacity),
  } as CSSProperties;
  const orderedSections = stats.config.sections.filter((section: SectionConfig) => section.enabled && isWidgetId(section.id));
  return (
    <main className="dashboard" aria-label="Zokute dashboard" style={dashboardStyle}>
      {orderedSections.map((section) => {
        const Component = widgets.find((widget) => widget.id === section.id as WidgetId)!.Component;
        return <Component key={section.id} stats={stats} history={history} />;
      })}
    </main>
  );
}
