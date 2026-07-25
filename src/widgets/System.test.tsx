import { render } from "@testing-library/react";
import { expect, it } from "vitest";
import { SystemWidget } from "./System";
import type { Stats } from "../useStats";

const stats = {
  cpu: { aggregate_percent: 0, core_percents: [] },
  memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
  disks: [],
  network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
  cpu_temperature: null,
  uptime: 3661,
  system_fields: [
    { id: "terminal", label: "Terminal", value: "WezTerm" },
    { id: "host", label: "Host", value: "zokute" },
    { id: "locale", label: "Locale", value: "en_US.UTF-8" },
  ],
  config: {
    opacity: 0.92,
    sections: [],
    system_fields: ["host", "display", "locale", "terminal", "uptime"],
    show_cpu_cores: true,
    disks: [],
  },
} satisfies Stats;

it("renders configured system fields in configured order and omits unavailable ones", () => {
  const { container, queryByText } = render(<SystemWidget stats={stats} />);

  expect(Array.from(container.querySelectorAll(".metricLabel")).map((node) => node.textContent)).toEqual([
    "Host",
    "Locale",
    "Terminal",
    "Uptime",
  ]);
  expect(queryByText("Display")).toBeNull();
});
