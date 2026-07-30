import { act, cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { Stats } from "./useStats";
import { Settings } from "./Settings";

const invoke = vi.fn((command: string) =>
  Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined),
);
let removedHandler: ((event: { payload: string }) => void) | null = null;
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...(args as [string])) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: (event: string, handler: (event: { payload: string }) => void) => {
    if (event === "widget-removed") removedHandler = handler;
    return Promise.resolve(() => {});
  },
}));
vi.mock("./SettingsNav", () => ({ SettingsNav: () => <nav /> }));

function statsWith(opacity: number): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [], network: [], cpu_temperature: null, uptime: 0, now_ms: 0,
    edit_mode: false, edit_touched: false, system_fields: [],
    config: {
      opacity, text_opacity: 1, text_color: "#292824", graph_color: null,
      icon_color: null, show_background: true,
    },
    profile: {
      sections: [{ id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 }],
      system_fields: [], show_cpu_cores: true, disks: [],
    },
  };
}

beforeEach(() => {
  invoke.mockClear();
  removedHandler = null;
});
afterEach(cleanup);

function dragSlider(slider: HTMLElement, value: string) {
  const thumb = slider.closest(".settingsSliderThumb") as HTMLElement;
  fireEvent.mouseDown(thumb, { button: 0 });
  fireEvent.change(slider, { target: { value } });
  fireEvent.mouseUp(window);
}

it("waits for the first reading before showing controls", () => {
  const { queryByRole, getByText } = render(<Settings stats={null} />);
  expect(queryByRole("slider", { name: "Background opacity" })).toBeNull();
  getByText("Waiting for the first reading…");
});

it("previews without persisting, then persists on commit", async () => {
  const { findByRole } = render(<Settings stats={statsWith(0.5)} />);
  const slider = await findByRole("slider", { name: "Background opacity" });
  dragSlider(slider, "0.8");
  expect(invoke).toHaveBeenCalledWith("preview_opacity", { value: 0.8 });
  expect(invoke).toHaveBeenCalledWith("draft_config", {
    next: expect.objectContaining({ opacity: 0.8 }),
  });
});

it("keeps editing its own draft rather than a later reading", async () => {
  const { findByRole, getByRole, rerender } = render(<Settings stats={statsWith(0.5)} />);
  dragSlider(await findByRole("slider", { name: "Background opacity" }), "0.8");
  rerender(<Settings stats={statsWith(0.5)} />);
  expect(
    (getByRole("slider", { name: "Background opacity" }) as HTMLInputElement).value,
  ).toBe("0.8");
});

it("previews and persists text opacity independently", async () => {
  const { findByRole } = render(<Settings stats={statsWith(0.5)} />);
  const slider = await findByRole("slider", { name: "Text opacity" });
  dragSlider(slider, "0.6");
  expect(invoke).toHaveBeenCalledWith("preview_text_opacity", { value: 0.6 });
  expect(invoke).toHaveBeenCalledWith("draft_config", {
    next: expect.objectContaining({ opacity: 0.5, text_opacity: 0.6 }),
  });
});

it("persists colors from the picker and presets without changing text opacity", async () => {
  const { findByLabelText, getByLabelText } = render(<Settings stats={statsWith(0.5)} />);
  await findByLabelText("Text color");
  fireEvent.change(getByLabelText("Text color"), { target: { value: "#123456" } });
  expect(invoke).toHaveBeenLastCalledWith("draft_config", {
    next: expect.objectContaining({ text_color: "#123456", text_opacity: 1 }),
  });
  fireEvent.click(getByLabelText("Use white text color"));
  expect(invoke).toHaveBeenLastCalledWith("draft_config", {
    next: expect.objectContaining({ text_color: "#ffffff", text_opacity: 1 }),
  });
  fireEvent.change(getByLabelText("Graph color"), { target: { value: "#123456" } });
  expect(invoke).toHaveBeenLastCalledWith("draft_config", {
    next: expect.objectContaining({ graph_color: "#123456", text_opacity: 1 }),
  });
  fireEvent.click(getByLabelText("Use blue icon color"));
  expect(invoke).toHaveBeenLastCalledWith("draft_config", {
    next: expect.objectContaining({ icon_color: "#2563eb", text_opacity: 1 }),
  });
});

it("can hide the background without changing either opacity", async () => {
  const { findByRole, getByRole } = render(<Settings stats={statsWith(0.5)} />);
  await findByRole("switch", { name: "Show background" });
  fireEvent.click(getByRole("switch", { name: "Show background" }));
  expect(invoke).toHaveBeenCalledWith("draft_config", {
    next: expect.objectContaining({ opacity: 0.5, text_opacity: 1, show_background: false }),
  });
});

it("adds repeated widget instances instead of toggling a fixed section", async () => {
  const { findByRole, getByRole } = render(<Settings stats={statsWith(0.5)} />);
  await findByRole("button", { name: "Add cpu widget (1 active)" });
  fireEvent.click(getByRole("button", { name: "Add cpu widget (1 active)" }));
  await waitFor(() =>
    expect(invoke).toHaveBeenCalledWith("draft_profile", {
      next: expect.objectContaining({
        sections: expect.arrayContaining([
          expect.objectContaining({ id: "cpu", instance: "cpu" }),
          expect.objectContaining({ id: "cpu", instance: "cpu-2", enabled: true }),
        ]),
      }),
    }),
  );
});

it("does not restore a removed instance on the next settings change", async () => {
  const stats = statsWith(0.5);
  stats.profile.sections.push({
    id: "cpu", instance: "cpu-2", enabled: true, monitor: 0, x: 24, y: 24, width: 360,
  });
  const { findByRole, getByRole } = render(<Settings stats={stats} />);
  await findByRole("slider", { name: "Background opacity" });
  await waitFor(() => expect(removedHandler).not.toBeNull());
  act(() => removedHandler?.({ payload: "cpu-2" }));
  dragSlider(getByRole("slider", { name: "Background opacity" }), "0.8");
  expect(invoke).toHaveBeenLastCalledWith("draft_config", {
    next: expect.objectContaining({
      opacity: 0.8,
    }),
  });
});
