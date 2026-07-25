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
    system_fields: [],
    config: {
      opacity,
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
  expect(queryByLabelText("Opacity")).toBeNull();
  getByText("Waiting for the first reading…");
});

it("previews without persisting, then persists on commit", () => {
  const { getByLabelText } = render(<Settings stats={statsWith(0.5)} />);
  const slider = getByLabelText("Opacity");
  fireEvent.input(slider, { target: { value: "0.8" } });
  expect(invoke).toHaveBeenCalledWith("preview_opacity", { value: 0.8 });
  fireEvent.change(slider, { target: { value: "0.8" } });
  expect(invoke).toHaveBeenCalledWith("update_config", {
    next: expect.objectContaining({ opacity: 0.8 }),
  });
});

it("keeps editing its own draft rather than a later reading", () => {
  const { getByLabelText, rerender } = render(<Settings stats={statsWith(0.5)} />);
  fireEvent.change(getByLabelText("Opacity"), { target: { value: "0.8" } });
  rerender(<Settings stats={statsWith(0.5)} />);
  expect((getByLabelText("Opacity") as HTMLInputElement).value).toBe("0.8");
});
