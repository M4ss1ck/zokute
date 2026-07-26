import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig } from "../useStats";
import { VisualizerPreferences } from "./Visualizer";

afterEach(cleanup);

function config(): StatsConfig {
  return {
    opacity: 1,
    sections: [
      { id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
      { id: "spectrum", instance: "spectrum", enabled: true, monitor: 0, x: 0, y: 900, width: 1920 },
    ],
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

it("renders nothing when no visualizer is enabled", () => {
  const bare: StatsConfig = { ...config(), sections: [config().sections[0]] };
  const { container } = render(<VisualizerPreferences config={bare} onChange={vi.fn()} />);
  expect(container.firstChild).toBeNull();
});

it("switches a visualizer into gradient mode", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<VisualizerPreferences config={config()} onChange={onChange} />);
  fireEvent.click(getByRole("radio", { name: "Gradient" }));
  expect(onChange.mock.calls[0][0].sections[1].color_mode).toBe("gradient");
  expect(onChange.mock.calls[0][0].sections[0].color_mode).toBeUndefined();
});

it("edits the primary color of the matching instance only", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(<VisualizerPreferences config={config()} onChange={onChange} />);
  fireEvent.change(getByLabelText("spectrum color"), { target: { value: "#c07100" } });
  expect(onChange.mock.calls[0][0].sections[1].color_a).toBe("#c07100");
});
