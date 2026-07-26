import type { CSSProperties } from "react";
import type { StatsConfig } from "./useStats";

type DashboardStyle = CSSProperties & {
  "--dashboard-opacity": number;
  "--dashboard-text-opacity": number;
  "--panel-title-color": string;
  "--panel-label-color": string;
  "--panel-value-color": string;
  "--panel-value-secondary-color": string;
  "--viz-stroke-color": string;
  "--panel-icon-color": string;
  "--dashboard-scale": number;
};

// Width is divided by the zoom factor because `transform: scale()` does not
// participate in layout: the dashboard lays out small and is drawn large, so
// the drawn result matches the window.
export function dashboardStyle(config: StatsConfig | undefined, scale: number, width: number | undefined): DashboardStyle {
  const text = config?.text_color ?? "#292824";
  return {
    "--dashboard-opacity": config?.opacity ?? 1,
    "--dashboard-text-opacity": config?.text_opacity ?? 1,
    "--panel-title-color": text,
    "--panel-label-color": text,
    "--panel-value-color": text,
    "--panel-value-secondary-color": text,
    "--viz-stroke-color": config?.graph_color ?? "#494137",
    "--panel-icon-color": config?.icon_color ?? "#c07100",
    "--dashboard-scale": scale,
    width: width === undefined ? undefined : `${width / scale}px`,
  };
}
