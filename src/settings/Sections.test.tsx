import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig } from "../useStats";
import { SectionToggles } from "./Sections";

afterEach(cleanup);

function config(): StatsConfig {
  return {
    opacity: 1,
    sections: [
      { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
      { id: "cpu", enabled: false, monitor: 1, x: 10, y: 20, width: 360 },
    ],
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

it("renders one row per configured section with its current state", () => {
  const { getByLabelText, queryByRole } = render(
    <SectionToggles config={config()} onChange={vi.fn()} />,
  );
  expect((getByLabelText("Show system") as HTMLInputElement).checked).toBe(true);
  expect((getByLabelText("Show cpu") as HTMLInputElement).checked).toBe(false);
  expect(queryByRole("combobox")).toBeNull();
});

it("toggles only the section it was given", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <SectionToggles config={config()} onChange={onChange} />,
  );
  fireEvent.click(getByLabelText("Show cpu"));
  expect(onChange.mock.calls[0][0].sections).toEqual([
    { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
    { id: "cpu", enabled: true, monitor: 1, x: 10, y: 20, width: 360 },
  ]);
});
