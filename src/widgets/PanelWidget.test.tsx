import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { PanelWidget } from "./PanelWidget";
import type { SectionConfig, Stats, StatsHistory } from "../useStats";

vi.mock("./System", () => ({ SystemWidget: () => <div data-testid="system" /> }));
vi.mock("./Cpu", () => ({ CpuWidget: () => <div data-testid="cpu" /> }));

afterEach(cleanup);

const stats = {
  cpu: { aggregate_percent: 0, core_percents: [] },
  memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
  disks: [],
  network: [],
  cpu_temperature: null,
  uptime: 0,
  now_ms: 0,
  edit_mode: false,
  edit_touched: false,
  fullscreen: false,
  system_fields: [],
  config: { opacity: 0.42 },
  profile: { sections: [], system_fields: [], show_cpu_cores: true, disks: [] },
} satisfies Stats;

const history: StatsHistory = { cpuAggregate: [], networkDown: [], networkUp: [] };

function makeSection(overrides: Partial<SectionConfig> = {}): SectionConfig {
  return {
    id: "panel",
    enabled: true,
    monitor: 0,
    x: 0,
    y: 0,
    width: 500,
    panel_gap: 8,
    panel_padding: 12,
    children: [
      { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 400 },
      { id: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 400 },
    ],
    ...overrides,
  };
}

it("renders the panel container with panelDashboard and the configured gap and padding", () => {
  const { container, getByTestId } = render(
    <PanelWidget stats={stats} history={history} section={makeSection()} />,
  );
  const panel = container.querySelector(".panelDashboard");
  expect(panel).toBeTruthy();
  expect(panel?.getAttribute("style")).toContain("gap: 8px");
  expect(panel?.getAttribute("style")).toContain("padding: 12px");
  getByTestId("system");
  getByTestId("cpu");
});

it("is not itself a dashboard element", () => {
  const { container } = render(
    <PanelWidget stats={stats} history={history} section={makeSection()} />,
  );
  expect(container.querySelector(".dashboard")).toBeNull();
});
