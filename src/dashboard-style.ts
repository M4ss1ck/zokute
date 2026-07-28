import type { CSSProperties } from "react";
import type { StatsConfig } from "./useStats";

const DENSITY_GAPS = { compact: "var(--space-xs)", comfortable: "var(--space-sm)" } as const;

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
  "--dashboard-gap"?: string;
  "--dashboard-font-scale"?: number;
};

export function dashboardStyle(config: StatsConfig | undefined, scale: number, width: number | undefined): DashboardStyle {
  const text = config?.text_color ?? "#292824";
  const density = config?.density ?? "compact";
  return {
    "--dashboard-opacity": config?.opacity ?? 1,
    "--dashboard-text-opacity": config?.text_opacity ?? 1,
    "--panel-title-color": config?.accent_color ?? text,
    "--panel-label-color": text,
    "--panel-value-color": text,
    "--panel-value-secondary-color": text,
    "--viz-stroke-color": config?.graph_color ?? "#494137",
    "--panel-icon-color": config?.icon_color ?? (config?.accent_color ?? "#c07100"),
    "--dashboard-scale": scale,
    "--dashboard-gap": DENSITY_GAPS[density as keyof typeof DENSITY_GAPS] ?? DENSITY_GAPS.compact,
    "--dashboard-font-scale": config?.font_scale ?? 1,
    width: width === undefined ? undefined : `${width / scale}px`,
  };
}
