import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import { SystemWidget } from "./System";
import type { Stats } from "../useStats";

afterEach(cleanup);

const stats = {
  cpu: { aggregate_percent: 0, core_percents: [] },
  memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
  disks: [],
  network: [],
  cpu_temperature: null,
  uptime: 3661,
  now_ms: 0,
  edit_mode: false,
  edit_touched: false,
  fullscreen: false,
  system_fields: [
    { id: "terminal", label: "Terminal", value: "WezTerm" },
    { id: "host", label: "Host", value: "zokute" },
    { id: "locale", label: "Locale", value: "en_US.UTF-8" },
    { id: "uptime", label: "Uptime", value: "1 hour, 1 min" },
  ],
  config: {
    opacity: 0.92,
  },
  profile: {
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

it("removes the complete card header when configured", () => {
  const hiddenHeader = {
    ...stats,
    profile: {
      ...stats.profile,
      sections: [
        { id: "system", enabled: true, show_header: false, monitor: 0, x: 0, y: 0, width: 360 },
      ],
    },
  };
  const { container } = render(<SystemWidget stats={hiddenHeader} />);

  expect(container.querySelector(".panelHeader")).toBeNull();
  expect(container.querySelector(".panelTitle")).toBeNull();
  expect(container.querySelector(".panelIcon")).toBeNull();
});

it("renders every row sharing one configured field id", () => {
  const multiRow = {
    ...stats,
    system_fields: [
      { id: "display", label: "Display (A)", value: "1920x1080" },
      { id: "display", label: "Display (B)", value: "2560x1440" },
    ],
    profile: { ...stats.profile, system_fields: ["display"] },
  };
  const { container } = render(<SystemWidget stats={multiRow} />);

  expect(
    Array.from(container.querySelectorAll(".metricLabel")).map(
      (node) => node.textContent,
    ),
  ).toEqual(["Display (A)", "Display (B)"]);
});
