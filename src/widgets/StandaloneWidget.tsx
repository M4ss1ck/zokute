import { useEffect, useRef, useState, type ComponentType } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalSize } from "@tauri-apps/api/dpi";
import type { SectionConfig, Stats, StatsHistory } from "../useStats";
import { EditOverlay, type ResizeDirection } from "../EditOverlay";
import { dashboardStyle } from "../dashboard-style";
import { targetWindowSize } from "../window-size";
import { WidgetErrorBoundary } from "./WidgetErrorBoundary";

type WidgetProps = { stats: Stats; history: StatsHistory; section: SectionConfig };

interface Props {
  Widget: ComponentType<WidgetProps>;
  stats: Stats;
  history: StatsHistory;
  section: SectionConfig;
  label: string;
}

function isDiagonal(direction: ResizeDirection) {
  return direction !== "North" && direction !== "South" && direction !== "East" && direction !== "West";
}

function isBare(id: string) { return id === "spectrum" || id === "ring"; }

export function StandaloneWidget({ Widget, stats, history, section, label }: Props) {
  const editing = stats.edit_mode;
  const [viewportWidth, setViewportWidth] = useState(globalThis.innerWidth);
  const [liveScale, setLiveScale] = useState<number | null>(null);
  const zoomRef = useRef<{ baseWidth: number } | null>(null);
  const scale = liveScale ?? section.scale ?? 1;
  const scaleRef = useRef(scale);
  scaleRef.current = scale;
  const style = dashboardStyle(stats.config, scale, editing ? viewportWidth : section.width, stats.fullscreen_dim);
  const dashboardRef = useRef<HTMLElement | null>(null);
  const panelRef = useRef<HTMLElement | null>(null);
  const resizeChain = useRef(Promise.resolve());
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
    if (!dashboardRef.current || !panelRef.current) return;
    const window = getCurrentWindow();
    const dashboard = dashboardRef.current;
    const element = panelRef.current;
    const observer = new ResizeObserver(([entry]) => {
      const padding = getComputedStyle(dashboard);
      const paddingY =
        Number.parseFloat(padding.paddingTop || "0") + Number.parseFloat(padding.paddingBottom || "0");
      const next = targetWindowSize(entry, element, paddingY, {
        width: editing ? globalThis.innerWidth : section.width,
        height: zoomRef.current ? 0 : editing ? globalThis.innerHeight : section.height ?? 0,
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
  }, [section.instance, section.width, section.height, section.enabled, editing, scale]);

  return (
    <>
      <main
        className={["dashboard", stats.config.show_background === false ? "dashboard--background-hidden" : "", isBare(section.id) ? "dashboard--bare" : ""].filter(Boolean).join(" ")}
        aria-label="Zokute dashboard"
        ref={dashboardRef}
        style={style}
      >
        {stats && section ? (
          <div ref={panelRef}>
            <WidgetErrorBoundary instance={label}>
              <Widget stats={stats} history={history} section={section} />
            </WidgetErrorBoundary>
          </div>
        ) : null}
      </main>
      {editing ? (
        <EditOverlay
          label={label}
          bare={stats.config.show_background === false}
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
