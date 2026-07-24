import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

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
    name: string;
    mount: string;
    used_bytes: number;
    total_bytes: number;
  }>;
  network: { down_bytes_per_second: number; up_bytes_per_second: number };
  cpu_temperature: { label: string; celsius: number } | null;
  gpu_temperatures: Array<{ label: string; celsius: number }>;
  uptime: number;
  hostname: string;
  config: {
    monitor: number;
    x: number;
    y: number;
    width: number;
    height: number;
    opacity: number;
    widgets: string[];
  };
}

export default function useStats(): Stats | null {
  const [stats, setStats] = useState<Stats | null>(null);
  useEffect(() => {
    let active = true;
    let unlisten = () => {};
    void listen<Stats>("stats", ({ payload }) => {
      if (active) setStats(payload);
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
  return stats;
}
