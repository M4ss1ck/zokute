import { useEffect, useRef, useState, type CSSProperties, type ComponentType } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/dpi";
import useStats, { type SectionConfig, type Stats, type StatsHistory } from "./useStats";
import { CpuWidget } from "./widgets/Cpu";
import { DiskWidget } from "./widgets/Disk";
import { MemoryWidget } from "./widgets/Memory";
import { NetworkWidget } from "./widgets/Network";
import { SpectrumWidget } from "./widgets/Spectrum";
import { RingWidget } from "./widgets/Ring";
import { SystemWidget } from "./widgets/System";
import { Settings } from "./Settings";
import { EditOverlay, type ResizeDirection } from "./EditOverlay";
const SETTINGS_LABEL = "settings";
type WidgetId = "system" | "cpu" | "memory" | "disk" | "network" | "spectrum" | "ring";
type WidgetProps = { stats: Stats; history: StatsHistory; section: SectionConfig };
type DashboardStyle = CSSProperties & { "--dashboard-opacity": number; "--dashboard-text-opacity": number; "--panel-title-color": string; "--panel-label-color": string; "--panel-value-color": string; "--panel-value-secondary-color": string; "--viz-stroke-color": string; "--panel-icon-color": string; "--dashboard-scale": number };
const widgets: Record<WidgetId, ComponentType<WidgetProps>> = {
  system: SystemWidget,
  cpu: CpuWidget,
  memory: MemoryWidget,
  disk: DiskWidget,
  network: NetworkWidget,
  spectrum: SpectrumWidget, ring: RingWidget,
};
function getBorderBoxHeight(entry: ResizeObserverEntry, element: HTMLElement) {
  const borderBoxSize = Array.isArray(entry.borderBoxSize) ? entry.borderBoxSize[0] : entry.borderBoxSize;
  return borderBoxSize ? borderBoxSize.blockSize : element.getBoundingClientRect().height;
}
function lastSection(label: string, sections: SectionConfig[]) {
  return sections.find((section) => (section.instance ?? section.id) === label);
}
function isWidgetId(id: string): id is WidgetId {
  return id in widgets;
}
function isBare(id: string) { return id === "spectrum" || id === "ring"; }
function isCorner(direction: ResizeDirection) {
  return direction.length > 5;
}
export default function App() {
  const { stats, history } = useStats();
  const dashboardRef = useRef<HTMLElement | null>(null);
  const panelRef = useRef<HTMLElement | null>(null);
  const resizeChain = useRef(Promise.resolve());
  const label = getCurrentWindow().label as WidgetId | string;
  const section = stats ? lastSection(label, stats.config.sections) : undefined;
  const renderableSection = section && section.enabled && isWidgetId(section.id) ? section : null;
  const Widget = renderableSection ? widgets[renderableSection.id] : null;
  const editing = stats?.edit_mode ?? false;
  const configuredScale = renderableSection?.scale ?? 1;
  const [viewportWidth, setViewportWidth] = useState(globalThis.innerWidth);
  const [liveScale, setLiveScale] = useState<number | null>(null);
  const scaleRef = useRef(configuredScale);
  const resizeRef = useRef<{ direction: ResizeDirection; baseWidth: number } | null>(null);
  const scale = liveScale ?? configuredScale;
  scaleRef.current = scale;
  const textColor = stats?.config.text_color ?? "#292824";
  const dashboardStyle: DashboardStyle = {
    "--dashboard-opacity": stats?.config.opacity ?? 1,
    "--dashboard-text-opacity": stats?.config.text_opacity ?? 1,
    "--panel-title-color": textColor, "--panel-label-color": textColor,
    "--panel-value-color": textColor,
    "--panel-value-secondary-color": textColor,
    "--viz-stroke-color": stats?.config.graph_color ?? "#494137",
    "--panel-icon-color": stats?.config.icon_color ?? "#c07100",
    "--dashboard-scale": scale,
    width: renderableSection ? `${(editing ? viewportWidth : renderableSection.width) / scale}px` : undefined,
  };
  const windowSize = useRef<{ width: number; height: number } | null>(null);
  useEffect(() => {
    if (!editing || !renderableSection) {
      resizeRef.current = null;
      setLiveScale(null);
      return;
    }
    const resized = () => {
      setViewportWidth(globalThis.innerWidth);
      const active = resizeRef.current;
      if (!active || !isCorner(active.direction)) return;
      const next = Math.min(3, Math.max(0.5, globalThis.innerWidth / active.baseWidth));
      scaleRef.current = next;
      setLiveScale(next);
    };
    const finished = () => {
      const active = resizeRef.current;
      resizeRef.current = null;
      if (active && isCorner(active.direction)) {
        void invoke("update_widget_scale", { id: label, scale: scaleRef.current });
      }
    };
    globalThis.addEventListener("resize", resized);
    globalThis.addEventListener("mouseup", finished);
    return () => {
      globalThis.removeEventListener("resize", resized);
      globalThis.removeEventListener("mouseup", finished);
    };
  }, [editing, label, renderableSection?.instance]);
  useEffect(() => {
    if (!renderableSection || !dashboardRef.current || !panelRef.current) return;
    const window = getCurrentWindow();
    const dashboard = dashboardRef.current;
    const element = panelRef.current;
    const observer = new ResizeObserver(([entry]) => {
      const padding = getComputedStyle(dashboard);
      const paddingY =
        Number.parseFloat(padding.paddingTop || "0") + Number.parseFloat(padding.paddingBottom || "0");
      const width = editing ? globalThis.innerWidth : renderableSection.width;
      const next = {
        width,
        height: Math.ceil((getBorderBoxHeight(entry, element) + paddingY) * scale),
      };
      if (windowSize.current && windowSize.current.width === next.width && windowSize.current.height === next.height) return;
      resizeChain.current = resizeChain.current
        .then(async () => {
          try {
            if (!editing) await window.setResizable(true);
            await window.setSize(new LogicalSize(next.width, next.height));
          } finally {
            if (!editing) await window.setResizable(false);
          }
          windowSize.current = next;
        })
        .catch(() => {});
    });
    observer.observe(element);
    return () => {
      observer.unobserve(element);
      observer.disconnect();
    };
  }, [renderableSection?.instance, renderableSection?.width, renderableSection?.enabled, editing, scale]);
  if (label === SETTINGS_LABEL) return <Settings stats={stats} />;
  return (
    <main className={["dashboard", stats?.config.show_background === false ? "dashboard--background-hidden" : "", renderableSection && isBare(renderableSection.id) ? "dashboard--bare" : ""].filter(Boolean).join(" ")} aria-label="Zokute dashboard" ref={dashboardRef} style={dashboardStyle}>
      {stats && Widget && renderableSection ? (
        <div ref={panelRef}>
          <Widget stats={stats} history={history} section={renderableSection} />
        </div>
      ) : null}
      {editing && renderableSection ? (
        <EditOverlay
          label={label}
          onResizeStart={(direction) => {
            resizeRef.current = { direction, baseWidth: globalThis.innerWidth / scaleRef.current };
          }}
        />
      ) : null}
    </main>
  );
}
