import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { aggregateNetworkRates } from "./network-rates";

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
  motion?: string;
}

export interface StatsProfile {
  profile_schema_version?: number;
  sections: SectionConfig[];
  system_fields: string[];
  show_cpu_cores: boolean;
  disks: DiskPreference[];
  collect_interval_ms?: number;
}

export type MergedConfig = StatsConfig & StatsProfile;

export interface Stats {
  cpu: { aggregate_percent: number; core_percents: number[] };
  memory: { used_bytes: number; total_bytes: number; swap_used_bytes: number; swap_total_bytes: number };
  disks: Array<{ id: string; name: string; mount: string; used_bytes: number; total_bytes: number; temperature_celsius: number | null; display_label: string | null }>;
  network: import("./network-rates").NetworkReading[];
  cpu_temperature: { label: string; celsius: number } | null;
  uptime: number;
  now_ms: number;
  edit_mode: boolean;
  fullscreen: boolean;
  fullscreen_dim?: number | null;
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
  x: number; y: number; position?: import("./section-position").SectionPosition;  width: number;
  height?: number;
  scale?: number;
  color_mode?: "solid" | "gradient";
  color_a?: string; color_b?: string;
  gradient_direction?: "horizontal" | "vertical";
  clock_font?: "mono" | "sans";
  clock_color?: string;
  clock_seconds?: boolean; clock_24h?: boolean;
  clock_ampm?: boolean; clock_pad?: boolean;
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
  viz_bar_count?: number | null;
  viz_min_hz?: number | null; viz_max_hz?: number | null;
  viz_gain?: number | null; viz_smoothing?: number | null; viz_decay?: number | null;
  viz_mirror?: boolean | null; viz_gap?: number | null;
  viz_rounded_caps?: boolean | null; viz_fps?: number | null;
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

interface UseStatResult { stats: Stats | null; history: StatsHistory; }

export default function useStats(): UseStatResult {
  const [stats, setStats] = useState<Stats | null>(null);
  const [history, setHistory] = useState<StatsHistory>({ cpuAggregate: [], networkDown: [], networkUp: [] });
  useEffect(() => {
    let active = true;
    let unlisten = () => {};
    void listen<Stats>("stats", ({ payload }) => {
      if (!active) return;
      const network = aggregateNetworkRates(payload.network);
      setStats(payload);
      setHistory((previous) => ({
        cpuAggregate: pushSample(previous.cpuAggregate, payload.cpu.aggregate_percent),
        networkDown: pushSample(previous.networkDown, network.down_bytes_per_second),
        networkUp: pushSample(previous.networkUp, network.up_bytes_per_second),
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
