import { useEffect, useRef, type ComponentType } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/dpi";
import useStats, { type SectionConfig, type Stats, type StatsHistory } from "./useStats";
import { CpuWidget } from "./widgets/Cpu";
import { DiskWidget } from "./widgets/Disk";
import { MemoryWidget } from "./widgets/Memory";
import { NetworkWidget } from "./widgets/Network";
import { SystemWidget } from "./widgets/System";

type WidgetId = "system" | "cpu" | "memory" | "disk" | "network";
type WidgetProps = { stats: Stats; history: StatsHistory };
const widgets: Record<WidgetId, ComponentType<WidgetProps>> = {
  system: SystemWidget,
  cpu: CpuWidget,
  memory: MemoryWidget,
  disk: DiskWidget,
  network: NetworkWidget,
};

function isWidgetId(id: string): id is WidgetId {
  return id in widgets;
}

export default function App() {
  const { stats, history } = useStats();
  const dashboardRef = useRef<HTMLElement | null>(null);
  const label = getCurrentWindow().label as WidgetId | string;
  const section = stats?.config.sections.find((candidate: SectionConfig) => candidate.id === label && candidate.enabled);
  const Widget = section && isWidgetId(section.id) ? widgets[section.id] : null;
  const windowSize = useRef<{ width: number; height: number } | null>(null);
  useEffect(() => {
    if (!section || !dashboardRef.current) return;
    const window = getCurrentWindow();
    const element = dashboardRef.current;
    const observer = new ResizeObserver(([entry]) => {
      const next = { width: section.width, height: Math.ceil(entry.contentRect.height) };
      if (windowSize.current && windowSize.current.width === next.width && windowSize.current.height === next.height) return;
      windowSize.current = next;
      void window.setSize(new LogicalSize(next.width, next.height));
    });
    observer.observe(element);
    return () => {
      observer.unobserve(element);
      observer.disconnect();
    };
  }, [section?.id, section?.width]);
  return (
    <main className="dashboard" aria-label="Zokute dashboard" ref={dashboardRef}>
      {stats && Widget ? <Widget stats={stats} history={history} /> : null}
    </main>
  );
}
