import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

const HISTORY_LENGTH = 60;

export interface StatsConfig {
  opacity: number;
  text_opacity?: number;
  sections: SectionConfig[];
  system_fields: string[];
  show_cpu_cores: boolean;
  disks: DiskPreference[];
}

// Mirrors `src-tauri/src/collect.rs` so the single stats event stays field-for-field.
export interface Stats {
  cpu: { aggregate_percent: number; core_percents: number[] };
  memory: {
    used_bytes: number;
    total_bytes: number;
    swap_used_bytes: number;
    swap_total_bytes: number;
  };
  disks: Array<{
    id: string;
    name: string;
    mount: string;
    used_bytes: number;
    total_bytes: number;
    temperature_celsius: number | null;
    display_label: string | null;
  }>;
  network: { down_bytes_per_second: number; up_bytes_per_second: number };
  cpu_temperature: { label: string; celsius: number } | null;
  uptime: number;
  edit_mode: boolean;
  system_fields: SystemField[];
  config: StatsConfig;
}

export interface SystemField {
  id: string;
  label: string;
  value: string;
}

export interface SectionConfig {
  id: string;
  enabled: boolean;
  monitor: number;
  x: number;
  y: number;
  width: number;
  scale?: number;
}

export interface DiskPreference {
  id: string;
  enabled: boolean;
  label: string | null;
}

// A 60-sample rolling buffer per sparkline metric, derived from `stats` on
// each tick. Lives here so it survives widget re-mounts and stays the sole
// place that touches the `stats` event.
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
