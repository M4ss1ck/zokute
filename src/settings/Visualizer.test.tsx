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
  fireEvent.change(getByLabelText("Color"), { target: { value: "#c07100" } });
  expect(onChange.mock.calls[0][0].sections[1].color_a).toBe("#c07100");
});

it("places gradient direction after the second color and updates it", () => {
  const onChange = vi.fn();
  const gradient = config();
  gradient.sections[1].color_mode = "gradient";
  const { getByLabelText, getByRole } = render(<VisualizerPreferences config={gradient} onChange={onChange} />);
  const secondColor = getByLabelText("Second color");
  const direction = getByRole("radiogroup", { name: "Gradient direction" });
  expect(secondColor.compareDocumentPosition(direction) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  fireEvent.click(getByRole("radio", { name: "Vertical" }));
  expect(onChange.mock.calls[0][0].sections[1].gradient_direction).toBe("vertical");
});

it("gives each visualizer its own titled block", () => {
  const two = config();
  two.sections.push({ id: "ring", instance: "ring", enabled: true, monitor: 0, x: 0, y: 0, width: 240 });
  const { getByRole } = render(<VisualizerPreferences config={two} onChange={vi.fn()} />);
  expect(getByRole("heading", { name: "Spectrum", level: 3 })).toHaveAttribute("id", "instance-spectrum");
  expect(getByRole("heading", { name: "Ring", level: 3 })).toHaveAttribute("id", "instance-ring");
});
