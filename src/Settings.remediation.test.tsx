import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { StrictMode } from "react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { Stats } from "./useStats";
import { Settings } from "./Settings";
const invoke = vi.fn((command: string): Promise<unknown> => Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined));
let closeHandler: ((payload: boolean) => void) | null = null;
let externalHandler: (() => void) | null = null;
let deferClose = false;
let closeResolvers: Array<(cleanup: () => void) => void> = [];
const closeUnlisten = vi.fn();
const destroy = vi.fn(() => Promise.resolve());
class MockIntersectionObserver { observe = vi.fn(); unobserve = vi.fn(); disconnect = vi.fn(); }
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...(args as [string])) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: (event: string, handler: (event: { payload: boolean }) => void) => {
    if (event === "settings-close-requested") closeHandler = (payload) => handler({ payload });
    if (event === "external-config-changed") externalHandler = () => handler({ payload: false });
    if (event === "settings-close-requested" && deferClose) return new Promise<() => void>((resolve) => closeResolvers.push(resolve));
    return Promise.resolve(() => {});
  },
}));
vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: () => ({ destroy }) }));
function statsWith(opacity: number): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [], network: [], cpu_temperature: null, uptime: 0, now_ms: 0, edit_mode: false,
    edit_touched: false, fullscreen: false, system_fields: [],
    config: { opacity, text_opacity: 1, text_color: "#292824", graph_color: null, icon_color: null, show_background: true },
    profile: { sections: [{ id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 }], system_fields: [], show_cpu_cores: true, disks: [] },
  };
}
beforeEach(() => {
  invoke.mockReset();
  invoke.mockImplementation((command) =>
    Promise.resolve(command === "needs_onboarding_cmd" ? false : command === "accept_external_config"
      ? { config: statsWith(0.4).config, profile: statsWith(0.4).profile }
      : undefined));
  closeHandler = null;
  externalHandler = null;
  deferClose = false;
  closeResolvers = [];
  closeUnlisten.mockClear();
  destroy.mockClear();
  vi.stubGlobal("IntersectionObserver", MockIntersectionObserver);
});
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });
function dragSlider(slider: HTMLElement, value: string) {
  const thumb = slider.closest(".settingsSliderThumb") as HTMLElement;
  fireEvent.mouseDown(thumb, { button: 0 });
  fireEvent.change(slider, { target: { value } });
  fireEvent.mouseUp(window);
}
it("adopts merged geometry when Arrange turns off", async () => {
  const merged = statsWith(1).profile; merged.sections[0].width = 480;
  invoke.mockImplementation((command) => Promise.resolve(
    command === "needs_onboarding_cmd" ? false : command === "begin_settings_session" ? true : command === "set_arrange" ? merged : undefined,
  ));
  const { getByRole } = render(<Settings stats={statsWith(1)} />);
  fireEvent.click(await screen.findByRole("switch", { name: "Arrange widgets", checked: true }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("set_arrange", { enabled: false }));
  fireEvent.click(getByRole("switch", { name: "Show CPU cores" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("draft_profile", {
    next: expect.objectContaining({ sections: [expect.objectContaining({ width: 480 })] }),
  }));
});
it("keeps Arrange unchanged and reports a rejected transition", async () => {
  invoke.mockImplementation((command) => command === "set_arrange" ? Promise.reject("arrange failed") : Promise.resolve(
    command === "needs_onboarding_cmd" ? false : command === "begin_settings_session" ? true : undefined,
  ));
  render(<Settings stats={statsWith(1)} />);
  fireEvent.click(await screen.findByRole("switch", { name: "Arrange widgets", checked: true }));
  expect(await screen.findByRole("alert")).toHaveTextContent("arrange failed");
  expect(screen.getByRole("switch", { name: "Arrange widgets" })).toBeChecked();
});
it("prompts from synchronous backend touched state", async () => {
  render(<Settings stats={statsWith(1)} />);
  act(() => closeHandler?.(true));
  expect(await screen.findByRole("dialog")).not.toBeNull();
});
it("closes a deferred close listener after Strict Mode cleanup", async () => {
  deferClose = true;
  const { unmount } = render(<StrictMode><Settings stats={null} /></StrictMode>);
  expect(closeResolvers).toHaveLength(2); unmount();
  await act(async () => closeResolvers.forEach((resolve) => resolve(closeUnlisten)));
  expect(closeUnlisten).toHaveBeenCalledTimes(2);
});
it("keeps Save enabled and reports a rejected save", async () => {
  invoke.mockImplementation((command) => command === "save_settings" ? Promise.reject("disk full") : Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined));
  render(<Settings stats={statsWith(1)} />);
  dragSlider(await screen.findByRole("slider", { name: "Background opacity" }), "0.5");
  await waitFor(() => expect(screen.getByRole("button", { name: "Save" })).toBeEnabled());
  fireEvent.click(screen.getByRole("button", { name: "Save" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("disk full");
  expect(screen.getByRole("button", { name: "Save" })).toBeEnabled();
});
it("disables Save after a successful save advances the baseline", async () => {
  render(<Settings stats={statsWith(1)} />);
  dragSlider(await screen.findByRole("slider", { name: "Background opacity" }), "0.5");
  const save = screen.getByRole("button", { name: "Save" });
  await waitFor(() => expect(save).toBeEnabled());
  fireEvent.click(save);
  await waitFor(() => expect(save).toBeDisabled());
});
it("keeps only autostart dirty and does not close when autostart fails", async () => {
  invoke.mockImplementation((command) => command === "set_autostart" ? Promise.reject("denied") : Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined));
  render(<Settings stats={statsWith(1)} />);
  fireEvent.click(await screen.findByRole("switch", { name: "Start with the session" }));
  fireEvent.click(screen.getByRole("button", { name: "Save" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("denied");
  expect(screen.getByRole("button", { name: "Save" })).toBeEnabled();
  act(() => closeHandler?.(false));
  fireEvent.click(await screen.findByRole("button", { name: "Save and close" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("set_autostart", { enabled: true }));
  expect(destroy).not.toHaveBeenCalled();
});
it("reloads TOML while preserving unsaved autostart", async () => {
  render(<Settings stats={statsWith(1)} />);
  fireEvent.click(await screen.findByRole("switch", { name: "Start with the session" }));
  act(() => externalHandler?.());
  fireEvent.click(await screen.findByRole("button", { name: "Reload from disk" }));
  await waitFor(() => expect(screen.getAllByRole("slider")[0]).toHaveValue("0.4"));
  expect(screen.getByRole("button", { name: "Save" })).toBeEnabled();
});
it("makes the shell inert while Discard is pending", async () => {
  let resolveDiscard!: () => void;
  invoke.mockImplementation((command) => command === "discard_settings" ? new Promise<void>((resolve) => { resolveDiscard = resolve; }) : Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined));
  render(<Settings stats={statsWith(1)} />);
  dragSlider(await screen.findByRole("slider", { name: "Background opacity" }), "0.5");
  fireEvent.click(screen.getByRole("button", { name: "Discard" }));
  const shell = screen.getByRole("main", { name: "Zokute settings" });
  expect(shell).toHaveAttribute("inert");
  await act(async () => resolveDiscard());
  await waitFor(() => expect(shell).not.toHaveAttribute("inert"));
});
it("restores the shell after a pending Reload rejects", async () => {
  let rejectReload!: (error: string) => void;
  invoke.mockImplementation((command) => command === "accept_external_config" ? new Promise((_, reject) => { rejectReload = reject; }) : Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined));
  render(<Settings stats={statsWith(1)} />);
  act(() => externalHandler?.());
  fireEvent.click(await screen.findByRole("button", { name: "Reload from disk" }));
  const shell = screen.getByRole("main", { name: "Zokute settings" });
  expect(shell).toHaveAttribute("inert");
  await act(async () => rejectReload("reload failed"));
  await waitFor(() => expect(shell).not.toHaveAttribute("inert"));
  expect(screen.getByRole("alert")).toHaveTextContent("reload failed");
});
