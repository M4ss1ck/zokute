import { expect, it } from "vitest";
import { dashboardStyle } from "./dashboard-style";
import type { StatsConfig } from "./useStats";

const config = { opacity: 0.92, text_opacity: 1 } as StatsConfig;

it("uses the configured opacity when nothing is fullscreen", () => {
  expect(dashboardStyle(config, 1, undefined, null)["--dashboard-opacity"]).toBe(0.92);
});

it("dims to the fullscreen opacity while a fullscreen window is active", () => {
  expect(dashboardStyle(config, 1, undefined, 0.25)["--dashboard-opacity"]).toBe(0.25);
});

it("returns to the configured opacity once fullscreen ends", () => {
  expect(dashboardStyle(config, 1, undefined, undefined)["--dashboard-opacity"]).toBe(0.92);
});

it("dims to zero when asked, rather than falling back to the configured opacity", () => {
  expect(dashboardStyle(config, 1, undefined, 0)["--dashboard-opacity"]).toBe(0);
});
