import { render, within } from "@testing-library/react";
import { expect, it } from "vitest";
import type { Stats } from "../useStats";
import { DiskWidget } from "./Disk";

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
    network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    cpu_temperature: null,
    uptime: 0,
    system_fields: [],
    config: {
      opacity: 1,
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
  expect(within(labeledRow).getByText("512 GB / 1.0 TB")).toBeInTheDocument();
  expect(within(labeledRow).getByText("34.2°C")).toBeInTheDocument();
});

it("uses the disk name when no custom label is present and omits missing temperature text", () => {
  const { container } = render(<DiskWidget stats={baseStats()} />);

  const rows = container.querySelectorAll(".diskRow");
  const unlabeledRow = rows[1] as HTMLElement;

  expect(within(unlabeledRow).getByText("sda1")).toBeInTheDocument();
  expect(within(unlabeledRow).getByText("/data")).toBeInTheDocument();
  expect(within(unlabeledRow).getByText("128 GB / 256 GB")).toBeInTheDocument();
  expect(within(unlabeledRow).queryByText(/°C/)).toBeNull();
});
