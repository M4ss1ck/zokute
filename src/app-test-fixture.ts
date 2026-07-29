/// Stats payload the App tests render against. Widths differ per section so a
/// test can tell which section drove the window sizing.
export function makeStats() {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: [],
    cpu_temperature: null,
    uptime: 0,
    edit_mode: false,
    system_fields: [],
    config: { opacity: 0.42 },
    profile: {
      sections: [
        { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 401 },
        { id: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 402 },
        { id: "memory", enabled: true, monitor: 0, x: 0, y: 0, width: 403 },
        { id: "disk", enabled: true, monitor: 0, x: 0, y: 0, width: 404 },
        { id: "network", enabled: true, monitor: 0, x: 0, y: 0, width: 405 },
      ],
      system_fields: [],
      show_cpu_cores: true,
      disks: [],
    },
  };
}

export function makeHistory() {
  return { cpuAggregate: [], networkDown: [], networkUp: [] };
}
