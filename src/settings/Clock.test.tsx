import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import type { StatsConfig } from "../useStats";
import { ClockPreferences } from "./Clock";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

beforeEach(() => {
  vi.mocked(invoke).mockClear();
});

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

// The box cannot be reset through the config: while settings is open edit mode
// is active, and every save recaptures each window's live geometry over it.
it("resets the box through the window when the layout changes", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<ClockPreferences config={config()} onChange={onChange} />);
  fireEvent.click(getByRole("radio", { name: "Column" }));
  const patched = onChange.mock.calls[0][0].sections[1];
  expect(patched.clock_layout).toBe("column");
  expect(patched.width).toBe(360);
  expect(invoke).toHaveBeenCalledWith("resize_widget", { id: "clock", width: 160 });
});

it("selects the row layout by default and reports the switch back to row", () => {
  const onChange = vi.fn();
  const column = config();
  column.sections[1].clock_layout = "column";
  const { getByRole } = render(<ClockPreferences config={column} onChange={onChange} />);
  expect(getByRole("radio", { name: "Column" })).toHaveAttribute("data-selected");
  fireEvent.click(getByRole("radio", { name: "Row" }));
  expect(onChange.mock.calls[0][0].sections[1].clock_layout).toBe("row");
  expect(invoke).toHaveBeenCalledWith("resize_widget", { id: "clock", width: 360 });
});

it("defaults alignment to left and patches the chosen one", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<ClockPreferences config={config()} onChange={onChange} />);
  expect(getByRole("radio", { name: "Left" })).toHaveAttribute("data-selected");
  fireEvent.click(getByRole("radio", { name: "Center" }));
  expect(onChange.mock.calls[0][0].sections[1].clock_align).toBe("center");
  expect(onChange.mock.calls[0][0].sections[0].clock_align).toBeUndefined();
});

it("switches the font through the toggle group", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<ClockPreferences config={config()} onChange={onChange} />);
  expect(getByRole("radio", { name: "JetBrains Mono" })).toHaveAttribute("data-selected");
  fireEvent.click(getByRole("radio", { name: "IBM Plex Sans" }));
  expect(onChange.mock.calls[0][0].sections[1].clock_font).toBe("sans");
});

it("edits the colour of the clock instance", () => {
  const onChange = vi.fn();
  const { getByLabelText } = render(<ClockPreferences config={config()} onChange={onChange} />);
  fireEvent.change(getByLabelText("Color"), { target: { value: "#c07100" } });
  expect(onChange.mock.calls[0][0].sections[1].clock_color).toBe("#c07100");
});

it("gives each clock its own titled block", () => {
  const two = config();
  two.sections.push({ id: "clock", instance: "clock-2", enabled: true, monitor: 0, x: 24, y: 24, width: 360 });
  const { getByRole } = render(<ClockPreferences config={two} onChange={vi.fn()} />);
  expect(getByRole("heading", { name: "Clock", level: 3 })).toHaveAttribute("id", "instance-clock");
  expect(getByRole("heading", { name: "Clock 2", level: 3 })).toHaveAttribute("id", "instance-clock-2");
});
