import { render, within } from "@testing-library/react";
import { useEffect } from "react";
import { expect, it, vi } from "vitest";
import type { Stats } from "../useStats";
import { DiskWidget } from "./Disk";

const barMounts = new Map<string, number>();

vi.mock("../viz/Bar", () => ({
  Bar: ({ percent }: { percent: number }) => {
    useEffect(() => {
      barMounts.set("count", (barMounts.get("count") ?? 0) + 1);
    }, []);
    return <div data-testid="bar">{percent}</div>;
  },
}));

function baseStats(): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [
      {
        id: "disk-1",
        name: "nvme0n1p2",
        mount: "/",
        used_bytes: 512 * 1024 ** 3,
        total_bytes: 1024 * 1024 ** 3,
        temperature_celsius: 34.2,
        display_label: "Games",
      },
      {
        id: "disk-2",
        name: "sda1",
        mount: "/data",
        used_bytes: 128 * 1024 ** 3,
        total_bytes: 256 * 1024 ** 3,
        temperature_celsius: null,
        display_label: null,
      },
    ],
    network: [],
    cpu_temperature: null,
    uptime: 0,
    now_ms: 0,
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

it("renders selected disks with contextual labels and temperatures", () => {
  const { container } = render(<DiskWidget stats={baseStats()} />);
  const rows = container.querySelectorAll(".diskRow");
  const labeledRow = rows[0] as HTMLElement;

  expect(container.querySelectorAll(".diskRow")).toHaveLength(2);
  expect(within(labeledRow).getByText("Games")).toBeInTheDocument();
  expect(within(labeledRow).getByText("/")).toBeInTheDocument();
  expect(within(labeledRow).getByText("512 GiB / 1.0 TiB")).toBeInTheDocument();
  expect(within(labeledRow).getByText("34.2°C")).toBeInTheDocument();
});

it("uses the disk name when no custom label is present and omits missing temperature text", () => {
  const { container } = render(<DiskWidget stats={baseStats()} />);

  const rows = container.querySelectorAll(".diskRow");
  const unlabeledRow = rows[1] as HTMLElement;

  expect(within(unlabeledRow).getByText("sda1")).toBeInTheDocument();
  expect(within(unlabeledRow).getByText("/data")).toBeInTheDocument();
  expect(within(unlabeledRow).getByText("128 GiB / 256 GiB")).toBeInTheDocument();
  expect(within(unlabeledRow).queryByText(/°C/)).toBeNull();
});

it("keeps the same disk row mounted when only presentation fields change", () => {
  barMounts.clear();

  const stats = baseStats();
  const { container, rerender } = render(<DiskWidget stats={stats} />);
  const firstRow = container.querySelectorAll(".diskRow")[0] as HTMLElement;

  expect(barMounts.get("count")).toBe(2);
  expect(within(firstRow).getByText("Games")).toBeInTheDocument();

  const nextStats = {
    ...stats,
    disks: [
      {
        ...stats.disks[0],
        name: "nvme0n1p2-renamed",
        mount: "/games",
      },
      stats.disks[1],
    ],
  };

  rerender(<DiskWidget stats={nextStats} />);

  expect(barMounts.get("count")).toBe(2);
  const updatedFirstRow = container.querySelectorAll(".diskRow")[0] as HTMLElement;
  expect(within(updatedFirstRow).getByText("Games")).toBeInTheDocument();
  expect(within(updatedFirstRow).getByText("/games")).toBeInTheDocument();
});
