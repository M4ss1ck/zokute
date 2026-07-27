import { render, within } from "@testing-library/react";
import { beforeEach, expect, it, vi } from "vitest";
import type { Stats } from "../useStats";
import { MemoryWidget } from "./Memory";

const arcCalls: number[] = [];
const barCalls: number[] = [];

vi.mock("../viz/Arc", () => ({
  Arc: (props: { percent: number }) => {
    arcCalls.push(props.percent);
    return <svg data-testid="arc" />;
  },
}));

vi.mock("../viz/Bar", () => ({
  Bar: () => {
    barCalls.push(1);
    return <svg data-testid="bar" />;
  },
}));

beforeEach(() => {
  arcCalls.length = 0;
  barCalls.length = 0;
});

function baseStats(): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 8 * 1024 ** 3, total_bytes: 16 * 1024 ** 3, swap_used_bytes: 2 * 1024 ** 3, swap_total_bytes: 4 * 1024 ** 3 },
    disks: [],
    network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    cpu_temperature: null,
    uptime: 0,
    now_ms: 0,
    system_fields: [],
    config: { opacity: 1, sections: [], system_fields: [], show_cpu_cores: true, disks: [] },
  };
}

it("renders paired Memory and Swap rings with labels and totals below the rings", () => {
  const { container } = render(<MemoryWidget stats={baseStats()} />);

  expect(container.querySelectorAll(".memoryItem")).toHaveLength(2);
  expect(arcCalls).toHaveLength(2);
  expect(barCalls).toHaveLength(0);

  const items = container.querySelectorAll(".memoryItem");
  expect(within(items[0] as HTMLElement).getByText("Memory")).toBeInTheDocument();
  expect(within(items[0] as HTMLElement).getByText("8.0 GB / 16 GB")).toBeInTheDocument();
  expect(within(items[1] as HTMLElement).getByText("Swap")).toBeInTheDocument();
  expect(within(items[1] as HTMLElement).getByText("2.0 GB / 4.0 GB")).toBeInTheDocument();
});

it("omits Swap when the total is zero", () => {
  const stats = baseStats();
  stats.memory.swap_total_bytes = 0;
  stats.memory.swap_used_bytes = 0;

  const { container } = render(<MemoryWidget stats={stats} />);

  expect(container.querySelectorAll(".memoryItem")).toHaveLength(1);
  expect(arcCalls).toHaveLength(1);
  expect(container.querySelector(".memoryGrid--single")).toBeTruthy();
});

it("keeps the paired layout marker off when Swap is present", () => {
  const { container } = render(<MemoryWidget stats={baseStats()} />);

  expect(container.querySelectorAll(".memoryItem")).toHaveLength(2);
  expect(container.querySelector(".memoryGrid--single")).toBeNull();
});
