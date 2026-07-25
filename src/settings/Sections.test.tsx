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
  const { getByLabelText } = render(
    <SectionToggles config={config()} monitorCount={2} onChange={vi.fn()} />,
  );
  expect((getByLabelText("Show system") as HTMLInputElement).checked).toBe(true);
  expect((getByLabelText("Show cpu") as HTMLInputElement).checked).toBe(false);
});

it("toggles only the section it was given", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <SectionToggles config={config()} monitorCount={2} onChange={onChange} />,
  );
  fireEvent.click(getByLabelText("Show cpu"));
  expect(onChange.mock.calls[0][0].sections).toEqual([
    { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
    { id: "cpu", enabled: true, monitor: 1, x: 10, y: 20, width: 360 },
  ]);
});

it("changes the monitor without touching placement", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(
    <SectionToggles config={config()} monitorCount={3} onChange={onChange} />,
  );
  fireEvent.change(getByLabelText("Monitor for cpu"), { target: { value: "2" } });
  expect(onChange.mock.calls[0][0].sections[1]).toEqual({
    id: "cpu",
    enabled: false,
    monitor: 2,
    x: 10,
    y: 20,
    width: 360,
  });
});
