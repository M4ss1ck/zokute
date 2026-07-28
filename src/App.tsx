import { useEffect, type ComponentType } from "react";
import useStats, { type SectionConfig, type Stats, type StatsHistory } from "./useStats";
import { CpuWidget } from "./widgets/Cpu";
import { DiskWidget } from "./widgets/Disk";
import { MemoryWidget } from "./widgets/Memory";
import { NetworkWidget } from "./widgets/Network";
import { SpectrumWidget } from "./widgets/Spectrum";
import { RingWidget } from "./widgets/Ring";
import { SystemWidget } from "./widgets/System";
import { ClockWidget } from "./widgets/Clock";
import { DateWidget } from "./widgets/Date";
import { PluginWidget } from "./widgets/Plugin";
import { PanelWidget } from "./widgets/PanelWidget";
import { Settings } from "./Settings";
import { LayoutEditor } from "./LayoutEditor";
import { StandaloneWidget } from "./widgets/StandaloneWidget";
import { getCurrentWindow } from "@tauri-apps/api/window";

const SETTINGS_LABEL = "settings";
const LAYOUT_EDITOR_LABEL = "layout-editor";
type WidgetId = "system" | "cpu" | "memory" | "disk" | "network" | "spectrum" | "ring" | "clock" | "date";
type WidgetProps = { stats: Stats; history: StatsHistory; section: SectionConfig };
const widgets: Record<string, ComponentType<WidgetProps>> = {
  system: SystemWidget,
  cpu: CpuWidget,
  memory: MemoryWidget,
  disk: DiskWidget,
  network: NetworkWidget,
  spectrum: SpectrumWidget, ring: RingWidget, clock: ClockWidget, date: DateWidget,
  plugin: PluginWidget as unknown as ComponentType<WidgetProps>,
};

function isWidgetId(id: string): id is WidgetId {
  return id in widgets;
}

export default function App() {
  const { stats, history } = useStats();
  const label = getCurrentWindow().label as WidgetId | string;
  useEffect(() => {
    if (!stats) return;
    const theme = stats.config.theme ?? "light";
    document.documentElement.setAttribute("data-theme", theme);
    document.documentElement.setAttribute("data-motion", stats.config.motion ?? "system");
  }, [stats]);
  const section = stats ? stats.profile.sections.find((s) => (s.instance ?? s.id) === label) : undefined;
  const renderable = section && section.enabled && isWidgetId(section.id) ? section : null;
  const Widget = renderable ? widgets[renderable.id] : null;

  if (label === SETTINGS_LABEL) return <Settings stats={stats} />;
  if (label === LAYOUT_EDITOR_LABEL) return <LayoutEditor stats={stats} />;
  const panelSection = stats ? stats.profile.sections.find((s) => (s.instance ?? s.id) === label && s.id === "panel") : undefined;
  if (panelSection && stats) {
    return <PanelWidget stats={stats} history={history} section={panelSection} />;
  }
  if (stats && Widget && renderable) {
    return <StandaloneWidget Widget={Widget} stats={stats} history={history} section={renderable} label={label} />;
  }
  return <main className="dashboard" aria-label="Zokute dashboard" />;
}
