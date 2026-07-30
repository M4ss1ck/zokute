import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { Stats } from "./useStats";
import { Settings } from "./Settings";

const invoke = vi.fn((command: string): Promise<unknown> =>
  Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined),
);
let closeHandler: ((payload: boolean) => void) | null = null;
let externalHandler: (() => void) | null = null;
const destroy = vi.fn(() => Promise.resolve());
class MockIntersectionObserver {
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();
}
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...(args as [string])) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: (event: string, handler: (event: { payload: boolean }) => void) => {
    if (event === "settings-close-requested") closeHandler = (payload) => handler({ payload });
    if (event === "external-config-changed") externalHandler = () => handler({ payload: false });
    return Promise.resolve(() => {});
  },
}));
vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: () => ({ destroy }) }));

function statsWith(opacity: number): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [], network: [], cpu_temperature: null, uptime: 0, now_ms: 0,
    edit_mode: false, edit_touched: false, fullscreen: false, system_fields: [],
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
  invoke.mockReset();
  invoke.mockImplementation((command) =>
    Promise.resolve(command === "needs_onboarding_cmd" ? false : command === "accept_external_config"
      ? { config: statsWith(0.4).config, profile: statsWith(0.4).profile }
      : undefined));
  closeHandler = null;
  externalHandler = null;
  destroy.mockClear();
  vi.stubGlobal("IntersectionObserver", MockIntersectionObserver);
});
afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

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

it("prompts from synchronous backend touched state", async () => {
  render(<Settings stats={statsWith(1)} />);
  act(() => closeHandler?.(true));
  expect(await screen.findByRole("dialog")).not.toBeNull();
});

it("keeps Save enabled and reports a rejected save", async () => {
  invoke.mockImplementation((command) => command === "save_settings" ? Promise.reject("disk full") : Promise.resolve(command === "needs_onboarding_cmd" ? false : undefined));
  const touched = statsWith(1); touched.edit_touched = true;
  render(<Settings stats={touched} />);
  await waitFor(() => expect(screen.getByRole("button", { name: "Save" })).toBeEnabled());
  fireEvent.click(screen.getByRole("button", { name: "Save" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("disk full");
  expect(screen.getByRole("button", { name: "Save" })).toBeEnabled();
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
