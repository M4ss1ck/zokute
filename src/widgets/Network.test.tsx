import { render, within } from "@testing-library/react";
import { beforeEach, expect, it, vi } from "vitest";
import type { Stats, StatsHistory } from "../useStats";
import { NetworkWidget } from "./Network";

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

function baseStats(): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: { down_bytes_per_second: 1536, up_bytes_per_second: 512 },
    cpu_temperature: null,
    uptime: 0,
    now_ms: 0,
    system_fields: [],
    config: { opacity: 1, sections: [], system_fields: [], show_cpu_cores: true, disks: [] },
  };
}

const history: StatsHistory = {
  cpuAggregate: [],
  networkDown: [5, 10, 20],
  networkUp: [1, 2, 4],
};

it("renders value-led rows with full-width auto-scaled sparklines", () => {
  const { container } = render(<NetworkWidget stats={baseStats()} history={history} />);

  expect(container.querySelectorAll(".networkRow")).toHaveLength(2);
  expect(sparklineCalls).toEqual([
    { min: undefined, max: undefined },
    { min: undefined, max: undefined },
  ]);

  const rows = container.querySelectorAll(".networkRow");
  expect(within(rows[0] as HTMLElement).getByText("Down")).toBeInTheDocument();
  expect(within(rows[0] as HTMLElement).getByText("1.5 KB/s")).toBeInTheDocument();
  expect(within(rows[1] as HTMLElement).getByText("Up")).toBeInTheDocument();
  expect(within(rows[1] as HTMLElement).getByText("512 B/s")).toBeInTheDocument();
});
