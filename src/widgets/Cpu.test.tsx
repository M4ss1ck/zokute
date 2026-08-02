import { render } from "@testing-library/react";
import { expect, it, vi, beforeEach } from "vitest";
import type { Stats, StatsHistory } from "../useStats";
import { CpuWidget } from "./Cpu";

const sparklineCalls: Array<{ min?: number; max?: number }> = [];

vi.mock("../viz/Sparkline", () => ({
  Sparkline: (props: { min?: number; max?: number }) => {
    sparklineCalls.push({ min: props.min, max: props.max });
    return <svg data-testid="sparkline" />;
  },
}));

beforeEach(() => {
  sparklineCalls.length = 0;
});

const history = { cpuAggregate: [10, 20], networkDown: [], networkUp: [] } satisfies StatsHistory;

function baseStats(): Stats {
  return {
    cpu: { aggregate_percent: 42, core_percents: [11, 22] },
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
    config: {
      opacity: 1,
    },
    profile: {
      sections: [],
      system_fields: [],
      show_cpu_cores: true,
      disks: [],
    },
  };
}

it("shows CPU temperature when present and omits it when absent", () => {
  const withTemperature = baseStats();
  withTemperature.cpu_temperature = { label: "Package", celsius: 55.4 };
  const { getByText, queryByText, rerender } = render(<CpuWidget stats={withTemperature} history={history} />);

  expect(getByText("Package")).toBeInTheDocument();
  expect(getByText("55.4°C")).toBeInTheDocument();

  const withoutTemperature = baseStats();
  rerender(<CpuWidget stats={withoutTemperature} history={history} />);

  expect(queryByText("Package")).toBeNull();
});

it("shows core bars only when configured", () => {
  const { container, rerender } = render(<CpuWidget stats={baseStats()} history={history} />);

  expect(container.querySelectorAll(".coreCell")).toHaveLength(2);

  const hiddenCores = baseStats();
  hiddenCores.profile.show_cpu_cores = false;
  rerender(<CpuWidget stats={hiddenCores} history={history} />);

  expect(container.querySelectorAll(".coreCell")).toHaveLength(0);
});

it("passes autoscaled history to the sparkline", () => {
  render(<CpuWidget stats={baseStats()} history={history} />);

  expect(sparklineCalls).toEqual([{ min: undefined, max: undefined }]);
});
