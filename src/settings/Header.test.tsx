import { fireEvent, render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import type { StatsConfig } from "../useStats";
import { HeaderToggle } from "./Header";

const config: StatsConfig = {
  opacity: 1,
  sections: [
    { id: "system", enabled: true, show_header: true, monitor: 0, x: 0, y: 0, width: 360 },
    { id: "cpu", enabled: true, show_header: false, monitor: 0, x: 0, y: 0, width: 360 },
  ],
  system_fields: [],
  show_cpu_cores: true,
  disks: [],
};

it("shows the current state and updates only its widget section", () => {
  const onChange = vi.fn();
  const { getByRole } = render(
    <HeaderToggle id="cpu" config={config} onChange={onChange} />,
  );

  expect(getByRole("switch", { name: "Show header" })).not.toBeChecked();
  fireEvent.click(getByRole("switch", { name: "Show header" }));
  expect(onChange.mock.calls[0][0].sections).toEqual([
    config.sections[0],
    { ...config.sections[1], show_header: true },
  ]);
});
