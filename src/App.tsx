import { useEffect, useRef, type CSSProperties, type ComponentType } from "react";
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
type DashboardStyle = CSSProperties & { "--dashboard-opacity": number };
const widgets: Record<WidgetId, ComponentType<WidgetProps>> = {
  system: SystemWidget,
  cpu: CpuWidget,
  memory: MemoryWidget,
  disk: DiskWidget,
  network: NetworkWidget,
};

function getBorderBoxHeight(entry: ResizeObserverEntry, element: HTMLElement) {
  const borderBoxSize = Array.isArray(entry.borderBoxSize) ? entry.borderBoxSize[0] : entry.borderBoxSize;
  return borderBoxSize ? borderBoxSize.blockSize : element.getBoundingClientRect().height;
}

function lastSection(label: string, sections: SectionConfig[]) {
  for (let index = sections.length - 1; index >= 0; index--) {
    const section = sections[index];
    if (section.id === label) return section;
  }
  return undefined;
}

function isWidgetId(id: string): id is WidgetId {
  return id in widgets;
}

export default function App() {
  const { stats, history } = useStats();
  const dashboardRef = useRef<HTMLElement | null>(null);
  const panelRef = useRef<HTMLElement | null>(null);
  const label = getCurrentWindow().label as WidgetId | string;
  const section = stats ? lastSection(label, stats.config.sections) : undefined;
  const renderableSection = section && section.enabled && isWidgetId(section.id) ? section : null;
  const Widget = renderableSection ? widgets[renderableSection.id] : null;
  const dashboardStyle: DashboardStyle = {
    "--dashboard-opacity": stats?.config.opacity ?? 1,
    width: renderableSection ? `${renderableSection.width}px` : undefined,
  };
  const windowSize = useRef<{ width: number; height: number } | null>(null);
  useEffect(() => {
    if (!renderableSection || !dashboardRef.current || !panelRef.current) return;
    const window = getCurrentWindow();
    const dashboard = dashboardRef.current;
    const element = panelRef.current;
    const observer = new ResizeObserver(([entry]) => {
      const padding = getComputedStyle(dashboard);
      const paddingY =
        Number.parseFloat(padding.paddingTop || "0") + Number.parseFloat(padding.paddingBottom || "0");
      const next = {
        width: renderableSection.width,
        height: Math.ceil(getBorderBoxHeight(entry, element) + paddingY),
      };
      if (windowSize.current && windowSize.current.width === next.width && windowSize.current.height === next.height) return;
      windowSize.current = next;
      void window.setSize(new LogicalSize(next.width, next.height));
    });
    observer.observe(element);
    return () => {
      observer.unobserve(element);
      observer.disconnect();
    };
  }, [renderableSection?.id, renderableSection?.width, renderableSection?.enabled]);
  return (
    <main className="dashboard" aria-label="Zokute dashboard" ref={dashboardRef} style={dashboardStyle}>
      {stats && Widget ? (
        <div ref={panelRef}>
          <Widget stats={stats} history={history} />
        </div>
      ) : null}
    </main>
  );
}
