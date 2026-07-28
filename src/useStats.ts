import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

const HISTORY_LENGTH = 60;

export interface StatsConfig {
  schema_version?: number;
  active_profile?: string;
  opacity: number;
  text_opacity?: number;
  text_color?: string;
  graph_color?: string | null;
  icon_color?: string | null;
  show_background?: boolean;
  theme?: string;
  accent_color?: string | null;
  density?: string;
  font_scale?: number;
  sans_font?: string | null;
  mono_font?: string | null;
  byte_format?: string;
  temperature_unit?: string;
  locale?: string | null;
}

export interface StatsProfile {
  profile_schema_version?: number;
  sections: SectionConfig[];
  system_fields: string[];
  show_cpu_cores: boolean;
  disks: DiskPreference[];
}

export type MergedConfig = StatsConfig & StatsProfile;

export interface Stats {
  cpu: { aggregate_percent: number; core_percents: number[] };
  memory: { used_bytes: number; total_bytes: number; swap_used_bytes: number; swap_total_bytes: number };
  disks: Array<{ id: string; name: string; mount: string; used_bytes: number; total_bytes: number; temperature_celsius: number | null; display_label: string | null }>;
  network: { down_bytes_per_second: number; up_bytes_per_second: number };
  cpu_temperature: { label: string; celsius: number } | null;
  uptime: number;
  now_ms: number;
  edit_mode: boolean;
  system_fields: SystemField[];
  config: StatsConfig;
  profile: StatsProfile;
}

export interface SystemField {
  id: string;
  label: string;
  value: string;
}

export interface SectionConfig {
  id: string;
  children?: SectionConfig[];
  panel_gap?: number;
  panel_padding?: number;
  instance?: string;
  enabled: boolean;
  show_header?: boolean;
  monitor: number;
  x: number;
  y: number;
  width: number;
  height?: number;
  scale?: number;
  color_mode?: "solid" | "gradient";
  color_a?: string;
  color_b?: string;
  gradient_direction?: "horizontal" | "vertical";
  clock_font?: "mono" | "sans";
  clock_color?: string;
  clock_seconds?: boolean;
  clock_24h?: boolean;
  clock_ampm?: boolean;
  clock_pad?: boolean;
  clock_layout?: "row" | "column";
  clock_align?: "left" | "center" | "right";
  date_weekday?: boolean;
  date_format?: "long" | "short" | "numeric";
  date_color?: string;
  accent_color?: string | null;
  transparent_surface?: boolean | null;
  opacity_override?: number | null;
  border_visible?: boolean | null;
  radius_override?: number | null;
  padding_override?: number | null;
  font_scale?: number | null;
  chart_colors?: string[] | null;
}

export interface DiskPreference {
  id: string;
  enabled: boolean;
  label: string | null;
}

export interface StatsHistory {
  cpuAggregate: number[];
  networkDown: number[];
  networkUp: number[];
}

function pushSample(samples: number[], value: number): number[] {
  return [...samples, value].slice(-HISTORY_LENGTH);
}

interface UseStatsResult {
  stats: Stats | null;
  history: StatsHistory;
}

export default function useStats(): UseStatsResult {
  const [stats, setStats] = useState<Stats | null>(null);
  const [history, setHistory] = useState<StatsHistory>({
    cpuAggregate: [],
    networkDown: [],
    networkUp: [],
  });
  useEffect(() => {
    let active = true;
    let unlisten = () => {};
    void listen<Stats>("stats", ({ payload }) => {
      if (!active) return;
      setStats(payload);
      setHistory((previous) => ({
        cpuAggregate: pushSample(previous.cpuAggregate, payload.cpu.aggregate_percent),
        networkDown: pushSample(previous.networkDown, payload.network.down_bytes_per_second),
        networkUp: pushSample(previous.networkUp, payload.network.up_bytes_per_second),
      }));
    }).then((cleanup) => {
      if (!active) {
        void cleanup();
        return;
      }
      unlisten = () => {
        void cleanup();
      };
    });
    return () => {
      active = false;
      unlisten();
    };
  }, []);
  return { stats, history };
}
