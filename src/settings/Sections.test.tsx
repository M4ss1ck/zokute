import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig } from "../useStats";
import { SectionToggles } from "./Sections";

vi.mock("@tauri-apps/api/window", () => ({
  currentMonitor: () =>
    Promise.resolve({ size: { width: 1920, height: 1080 }, scaleFactor: 1 }),
}));

afterEach(cleanup);

function config(): StatsConfig {
  return {
    opacity: 1,
    sections: [
      { id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
      { id: "cpu", instance: "cpu", enabled: true, monitor: 1, x: 10, y: 20, width: 360 },
    ],
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

it("renders one add control per widget type with its current count", () => {
  const { getByRole, queryByRole } = render(
    <SectionToggles config={config()} onChange={vi.fn()} />,
  );
  expect(getByRole("button", { name: "Add system widget (1 active)" })).toBeTruthy();
  expect(getByRole("button", { name: "Add cpu widget (1 active)" })).toBeTruthy();
  expect(getByRole("button", { name: "Add memory widget (0 active)" })).toBeTruthy();
  expect(getByRole("button", { name: "Add date widget (0 active)" })).toBeTruthy();
  expect(queryByRole("combobox")).toBeNull();
});

it("adds another independently labelled section", async () => {
  const onChange = vi.fn();
  const { getByRole } = render(
    <SectionToggles config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByRole("button", { name: "Add cpu widget (1 active)" }));
  await waitFor(() => expect(onChange).toHaveBeenCalled());
  expect(onChange.mock.calls[0][0].sections).toEqual([
    { id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
    { id: "cpu", instance: "cpu", enabled: true, monitor: 1, x: 10, y: 20, width: 360 },
    { id: "cpu", instance: "cpu-2", enabled: true, show_header: true, monitor: 1, x: 34, y: 44, width: 360, scale: 1 },
  ]);
});

it("seeds a spectrum across the full monitor width along the bottom", async () => {
  const onChange = vi.fn();
  const { getByRole } = render(<SectionToggles config={config()} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Add spectrum widget (0 active)" }));
  await waitFor(() => expect(onChange).toHaveBeenCalled());
  const added = onChange.mock.calls[0][0].sections.at(-1);
  expect(added).toMatchObject({ id: "spectrum", x: 0, y: 920, width: 1920 });
});

it("seeds a ring centred on the monitor", async () => {
  const onChange = vi.fn();
  const { getByRole } = render(<SectionToggles config={config()} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Add ring widget (0 active)" }));
  await waitFor(() => expect(onChange).toHaveBeenCalled());
  const added = onChange.mock.calls[0][0].sections.at(-1);
  expect(added).toMatchObject({ id: "ring", x: 750, y: 330, width: 420 });
});
