import { act, cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { Stats } from "./useStats";
import { Settings } from "./Settings";

const invoke = vi.fn((command: string) =>
  Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined),
);
let removedHandler: ((event: { payload: string }) => void) | null = null;
let closeHandler: (() => void) | null = null;
const destroy = vi.fn(() => Promise.resolve());
class MockIntersectionObserver {
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();
}
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...(args as [string])) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: (event: string, handler: (payload: { payload: string }) => void) => {
    if (event === "settings-close-requested") closeHandler = () => handler({ payload: "" });
    else removedHandler = handler;
    return Promise.resolve(() => {});
  },
}));
vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: () => ({ destroy }) }));

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

function dragSlider(slider: HTMLElement, value: string) {
  const thumb = slider.closest(".settingsSliderThumb") as HTMLElement;
  fireEvent.mouseDown(thumb, { button: 0 });
  fireEvent.change(slider, { target: { value } });
  fireEvent.mouseUp(window);
}

beforeEach(() => {
  invoke.mockClear();
  removedHandler = null;
  closeHandler = null;
  destroy.mockClear();
  vi.stubGlobal("IntersectionObserver", MockIntersectionObserver);
});
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

it("begins a session on mount", async () => {
  render(<Settings stats={statsWith(1)} />);
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("begin_settings_session"));
});

it("keeps Save inert until something changes", async () => {
  const { getByRole } = render(<Settings stats={statsWith(1)} />);
  await waitFor(() => expect(getByRole("button", { name: "Save" })).toBeDisabled());
});

it("arms Save once a control is touched", async () => {
  const { getByRole, getAllByRole } = render(<Settings stats={statsWith(1)} />);
  await waitFor(() => expect(getAllByRole("slider").length).toBeGreaterThan(0));
  act(() => dragSlider(getAllByRole("slider")[0], "0.5"));
  await waitFor(() => expect(getByRole("button", { name: "Save" })).toBeEnabled());
});

it("treats a dragged widget as a change even with no control touched", async () => {
  const touched = statsWith(1);
  touched.edit_touched = true;
  const { getByRole } = render(<Settings stats={touched} />);
  await waitFor(() => expect(getByRole("button", { name: "Save" })).toBeEnabled());
});

it("saves through the session command", async () => {
  const touched = statsWith(1);
  touched.edit_touched = true;
  const { getByRole } = render(<Settings stats={touched} />);
  await waitFor(() => expect(getByRole("button", { name: "Save" })).toBeEnabled());
  fireEvent.click(getByRole("button", { name: "Save" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("save_settings"));
});

it("applies autostart only when it actually changed", async () => {
  const touched = statsWith(1);
  touched.edit_touched = true;
  const { getByRole } = render(<Settings stats={touched} />);
  await waitFor(() => expect(getByRole("button", { name: "Save" })).toBeEnabled());
  fireEvent.click(getByRole("button", { name: "Save" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("save_settings"));
  expect(invoke).not.toHaveBeenCalledWith("set_autostart", expect.anything());
});

it("flips arrange mode through the backend", async () => {
  const { getByRole } = render(<Settings stats={statsWith(1)} />);
  await waitFor(() => expect(getByRole("switch", { name: "Arrange widgets" })).not.toBeNull());
  fireEvent.click(getByRole("switch", { name: "Arrange widgets" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("set_arrange", { enabled: true }));
});

it("prompts instead of closing when the draft is dirty", async () => {
  const touched = statsWith(1);
  touched.edit_touched = true;
  const { getByRole } = render(<Settings stats={touched} />);
  await waitFor(() => expect(getByRole("button", { name: "Save" })).toBeEnabled());
  act(() => closeHandler?.());
  await waitFor(() => expect(getByRole("dialog")).not.toBeNull());
});

it("closes straight away when the draft is clean", async () => {
  render(<Settings stats={statsWith(1)} />);
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("begin_settings_session"));
  act(() => closeHandler?.());
  await waitFor(() => expect(destroy).toHaveBeenCalledTimes(1));
});
