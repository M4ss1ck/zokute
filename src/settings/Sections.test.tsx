import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig } from "../useStats";
import { SectionToggles } from "./Sections";

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
  expect(queryByRole("combobox")).toBeNull();
});

it("adds another independently labelled section", () => {
  const onChange = vi.fn();
  const { getByRole } = render(
    <SectionToggles config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByRole("button", { name: "Add cpu widget (1 active)" }));
  expect(onChange.mock.calls[0][0].sections).toEqual([
    { id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
    { id: "cpu", instance: "cpu", enabled: true, monitor: 1, x: 10, y: 20, width: 360 },
    { id: "cpu", instance: "cpu-2", enabled: true, show_header: true, monitor: 1, x: 34, y: 44, width: 360, scale: 1 },
  ]);
});
