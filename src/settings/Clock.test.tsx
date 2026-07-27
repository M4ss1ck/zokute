import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import type { StatsConfig } from "../useStats";
import { ClockPreferences } from "./Clock";

afterEach(cleanup);

function config(): StatsConfig {
  return {
    opacity: 1,
    sections: [
      { id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
      { id: "clock", instance: "clock", enabled: true, monitor: 0, x: 0, y: 0, width: 360, height: 200 },
    ],
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

it("renders nothing when no clock is enabled", () => {
  const bare: StatsConfig = { ...config(), sections: [config().sections[0]] };
  const { container } = render(<ClockPreferences config={bare} onChange={vi.fn()} />);
  expect(container.firstChild).toBeNull();
});

it("disables the meridiem switch while 24-hour time is on", () => {
  const { getByRole, rerender } = render(<ClockPreferences config={config()} onChange={vi.fn()} />);
  expect(getByRole("switch", { name: "Show AM/PM" })).toBeEnabled();

  const twentyFour = config();
  twentyFour.sections[1].clock_24h = true;
  rerender(<ClockPreferences config={twentyFour} onChange={vi.fn()} />);
  expect(getByRole("switch", { name: "Show AM/PM" })).toBeDisabled();
});

it("patches only the clock section when a switch is toggled", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<ClockPreferences config={config()} onChange={onChange} />);
  fireEvent.click(getByRole("switch", { name: "Show seconds" }));
  expect(onChange.mock.calls[0][0].sections[1].clock_seconds).toBe(true);
  expect(onChange.mock.calls[0][0].sections[0].clock_seconds).toBeUndefined();
});

it("resets the box when the layout changes, leaving the zoom alone", () => {
  const onChange = vi.fn();
  const scaled = config();
  scaled.sections[1].scale = 1.5;
  const { getByRole } = render(<ClockPreferences config={scaled} onChange={onChange} />);
  fireEvent.click(getByRole("radio", { name: "Column" }));
  const patched = onChange.mock.calls[0][0].sections[1];
  expect(patched.clock_layout).toBe("column");
  expect(patched.width).toBe(160);
  expect(patched.height).toBeUndefined();
  expect(patched.scale).toBe(1.5);
});

it("edits the colour of the clock instance", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(<ClockPreferences config={config()} onChange={onChange} />);
  fireEvent.change(getByLabelText("clock color"), { target: { value: "#c07100" } });
  expect(onChange.mock.calls[0][0].sections[1].clock_color).toBe("#c07100");
});
