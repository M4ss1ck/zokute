import type { CSSProperties, ComponentType } from "react";
import useStats, { type Stats } from "./useStats";
import { CpuWidget } from "./widgets/Cpu";
import { DiskWidget } from "./widgets/Disk";
import { MemoryWidget } from "./widgets/Memory";
import { NetworkWidget } from "./widgets/Network";
import { SystemWidget } from "./widgets/System";
import { TemperaturesWidget } from "./widgets/Temperatures";

type WidgetId = "system" | "cpu" | "memory" | "disk" | "network" | "temperatures";
type WidgetEntry = { id: WidgetId; Component: ComponentType<{ stats: Stats }> };

const widgets: readonly WidgetEntry[] = [
  { id: "system", Component: SystemWidget },
  { id: "cpu", Component: CpuWidget },
  { id: "memory", Component: MemoryWidget },
  { id: "disk", Component: DiskWidget },
  { id: "network", Component: NetworkWidget },
  { id: "temperatures", Component: TemperaturesWidget },
];

export default function App() {
  const stats = useStats();
  if (!stats) return <main className="dashboard" aria-label="Zokute dashboard" />;
  const dashboardStyle = {
    ["--dashboard-opacity" as "--dashboard-opacity"]: String(stats.config.opacity),
  } as CSSProperties;
  const orderedWidgets = stats.config.widgets
    .map((id) => widgets.find((widget) => widget.id === id))
    .filter((widget): widget is WidgetEntry => widget !== undefined);
  return (
    <main className="dashboard" aria-label="Zokute dashboard" style={dashboardStyle}>
      {orderedWidgets.map(({ id, Component }) => (
        <Component key={id} stats={stats} />
      ))}
    </main>
  );
}
