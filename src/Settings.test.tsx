import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { Stats } from "./useStats";
import { Settings } from "./Settings";

const invoke = vi.fn(() => Promise.resolve());
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...args) }));

function statsWith(opacity: number): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    cpu_temperature: null,
    uptime: 0,
    edit_mode: false,
    system_fields: [],
    config: {
      opacity,
      text_opacity: 1,
      text_color: "#292824",
      show_background: true,
      sections: [{ id: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 }],
      system_fields: [],
      show_cpu_cores: true,
      disks: [],
    },
  };
}

beforeEach(() => invoke.mockClear());
// This project does not set vitest `globals`, so Testing Library never
// registers its automatic cleanup and renders would otherwise accumulate.
afterEach(cleanup);

it("waits for the first reading before showing controls", () => {
  const { queryByLabelText, getByText } = render(<Settings stats={null} />);
  expect(queryByLabelText("Background opacity")).toBeNull();
  getByText("Waiting for the first reading…");
});

it("previews without persisting, then persists on commit", () => {
  const { getByLabelText } = render(<Settings stats={statsWith(0.5)} />);
  const slider = getByLabelText("Background opacity");
  fireEvent.input(slider, { target: { value: "0.8" } });
  expect(invoke).toHaveBeenCalledWith("preview_opacity", { value: 0.8 });
  fireEvent.change(slider, { target: { value: "0.8" } });
  expect(invoke).toHaveBeenCalledWith("update_config", {
    next: expect.objectContaining({ opacity: 0.8 }),
  });
});

it("keeps editing its own draft rather than a later reading", () => {
  const { getByLabelText, rerender } = render(<Settings stats={statsWith(0.5)} />);
  fireEvent.change(getByLabelText("Background opacity"), { target: { value: "0.8" } });
  rerender(<Settings stats={statsWith(0.5)} />);
  expect((getByLabelText("Background opacity") as HTMLInputElement).value).toBe("0.8");
});

it("previews and persists text opacity independently", () => {
  const { getByLabelText } = render(<Settings stats={statsWith(0.5)} />);
  const slider = getByLabelText("Text opacity");
  fireEvent.input(slider, { target: { value: "0.6" } });
  expect(invoke).toHaveBeenCalledWith("preview_text_opacity", { value: 0.6 });
  fireEvent.change(slider, { target: { value: "0.6" } });
  expect(invoke).toHaveBeenCalledWith("update_config", {
    next: expect.objectContaining({ opacity: 0.5, text_opacity: 0.6 }),
  });
});

it("persists colors from the picker and presets without changing text opacity", () => {
  const { getByLabelText } = render(<Settings stats={statsWith(0.5)} />);
  fireEvent.change(getByLabelText("Text color"), { target: { value: "#123456" } });
  expect(invoke).toHaveBeenLastCalledWith("update_config", {
    next: expect.objectContaining({ text_color: "#123456", text_opacity: 1 }),
  });
  fireEvent.click(getByLabelText("Use white text"));
  expect(invoke).toHaveBeenLastCalledWith("update_config", {
    next: expect.objectContaining({ text_color: "#ffffff", text_opacity: 1 }),
  });
});

it("can hide the background without changing either opacity", () => {
  const { getByLabelText } = render(<Settings stats={statsWith(0.5)} />);
  fireEvent.click(getByLabelText("Show background"));
  expect(invoke).toHaveBeenCalledWith("update_config", {
    next: expect.objectContaining({ opacity: 0.5, text_opacity: 1, show_background: false }),
  });
});
