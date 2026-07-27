import { useEffect, useRef, useState, type ComponentType } from "react";
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
import { ClockWidget } from "./widgets/Clock";
import { Settings } from "./Settings";
import { EditOverlay, type ResizeDirection } from "./EditOverlay";
import { dashboardStyle } from "./dashboard-style";
import { targetWindowSize } from "./window-size";
const SETTINGS_LABEL = "settings";
type WidgetId = "system" | "cpu" | "memory" | "disk" | "network" | "spectrum" | "ring" | "clock";
type WidgetProps = { stats: Stats; history: StatsHistory; section: SectionConfig };
const widgets: Record<WidgetId, ComponentType<WidgetProps>> = {
  system: SystemWidget,
  cpu: CpuWidget,
  memory: MemoryWidget,
  disk: DiskWidget,
  network: NetworkWidget,
  spectrum: SpectrumWidget, ring: RingWidget, clock: ClockWidget,
};
function lastSection(label: string, sections: SectionConfig[]) {
  return sections.find((section) => (section.instance ?? section.id) === label);
}
function isWidgetId(id: string): id is WidgetId {
  return id in widgets;
}
function isBare(id: string) { return id === "spectrum" || id === "ring"; }
// Edges resize the box and reflow the content; diagonals zoom it.
function isDiagonal(direction: ResizeDirection) {
  return direction !== "North" && direction !== "South" && direction !== "East" && direction !== "West";
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
  const [viewportWidth, setViewportWidth] = useState(globalThis.innerWidth);
  const [liveScale, setLiveScale] = useState<number | null>(null);
  const zoomRef = useRef<{ baseWidth: number } | null>(null);
  const scale = liveScale ?? renderableSection?.scale ?? 1;
  const scaleRef = useRef(scale);
  scaleRef.current = scale;
  const style = dashboardStyle(stats?.config, scale, renderableSection ? (editing ? viewportWidth : renderableSection.width) : undefined);
  const windowSize = useRef<{ width: number; height: number } | null>(null);
  useEffect(() => {
    if (!editing) {
      zoomRef.current = null;
      setLiveScale(null);
      return;
    }
    const resized = () => {
      setViewportWidth(globalThis.innerWidth);
      const zoom = zoomRef.current;
      if (!zoom) return;
      const next = globalThis.innerWidth / zoom.baseWidth;
      if (!Number.isFinite(next) || next <= 0) return;
      scaleRef.current = next;
      setLiveScale(next);
      void invoke("update_widget_scale", { id: label, scale: next });
    };
    // Keep mouseup as a final write when WebKit receives it after the native drag.
    const finished = () => {
      const zoom = zoomRef.current;
      zoomRef.current = null;
      if (zoom) void invoke("update_widget_scale", { id: label, scale: scaleRef.current });
    };
    globalThis.addEventListener("resize", resized);
    globalThis.addEventListener("mouseup", finished);
    return () => {
      globalThis.removeEventListener("resize", resized);
      globalThis.removeEventListener("mouseup", finished);
    };
  }, [editing, label]);
  useEffect(() => {
    if (!renderableSection || !dashboardRef.current || !panelRef.current) return;
    const window = getCurrentWindow();
    const dashboard = dashboardRef.current;
    const element = panelRef.current;
    const observer = new ResizeObserver(([entry]) => {
      const padding = getComputedStyle(dashboard);
      const paddingY =
        Number.parseFloat(padding.paddingTop || "0") + Number.parseFloat(padding.paddingBottom || "0");
      const next = targetWindowSize(entry, element, paddingY, {
        width: editing ? globalThis.innerWidth : renderableSection.width,
        height: zoomRef.current ? 0 : editing ? globalThis.innerHeight : renderableSection.height ?? 0,
        scale,
      });
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
  }, [renderableSection?.instance, renderableSection?.width, renderableSection?.height, renderableSection?.enabled, editing, scale]);
  if (label === SETTINGS_LABEL) return <Settings stats={stats} />;
  return (
    <>
      <main className={["dashboard", stats?.config.show_background === false ? "dashboard--background-hidden" : "", renderableSection && isBare(renderableSection.id) ? "dashboard--bare" : ""].filter(Boolean).join(" ")} aria-label="Zokute dashboard" ref={dashboardRef} style={style}>
        {stats && Widget && renderableSection ? (
          <div ref={panelRef}>
            <Widget stats={stats} history={history} section={renderableSection} />
          </div>
        ) : null}
      </main>
      {/* Outside the dashboard on purpose: its `transform` would become the
          containing block for the overlay's `position: fixed`, pinning the
          handles to the content box instead of the window. */}
      {editing && renderableSection ? (
        <EditOverlay
          label={label}
          bare={stats?.config.show_background === false}
          scale={scale}
          onReset={() => {
            setLiveScale(1);
            void invoke("update_widget_scale", { id: label, scale: 1 });
          }}
          onResizeStart={(direction) => {
            zoomRef.current = isDiagonal(direction) ? { baseWidth: globalThis.innerWidth / scaleRef.current } : null;
          }}
        />
      ) : null}
    </>
  );
}
