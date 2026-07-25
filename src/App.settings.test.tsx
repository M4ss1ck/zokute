import { cleanup, render } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

let stats: any;
let history: any;
let windowLabel = "settings";
let observer: MockResizeObserver | null = null;
const setSize = vi.fn(() => Promise.resolve());

class MockResizeObserver {
  callback: ResizeObserverCallback;
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();

  constructor(callback: ResizeObserverCallback) {
    this.callback = callback;
    observer = this;
  }
}

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ label: windowLabel, setSize, setResizable: vi.fn(() => Promise.resolve()) }),
}));

vi.mock("@tauri-apps/api/dpi", () => ({
  LogicalSize: class LogicalSize {
    constructor(public width: number, public height: number) {}
  },
}));

vi.mock("./useStats", () => ({ default: () => ({ stats, history }) }));
vi.mock("./widgets/System", () => ({ SystemWidget: () => <div data-testid="system" /> }));
vi.mock("./widgets/Cpu", () => ({ CpuWidget: () => <div data-testid="cpu" /> }));
vi.mock("./widgets/Memory", () => ({ MemoryWidget: () => <div data-testid="memory" /> }));
vi.mock("./widgets/Disk", () => ({ DiskWidget: () => <div data-testid="disk" /> }));
vi.mock("./widgets/Network", () => ({ NetworkWidget: () => <div data-testid="network" /> }));
vi.mock("./Settings", () => ({ Settings: () => <div data-testid="settings" /> }));

beforeEach(() => {
  stats = {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    cpu_temperature: null,
    uptime: 0,
    edit_mode: false,
    system_fields: [],
    config: {
      opacity: 1,
      sections: [
        { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 401 },
        { id: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 402 },
        { id: "memory", enabled: true, monitor: 0, x: 0, y: 0, width: 403 },
        { id: "disk", enabled: true, monitor: 0, x: 0, y: 0, width: 404 },
        { id: "network", enabled: true, monitor: 0, x: 0, y: 0, width: 405 },
      ],
      system_fields: [],
      show_cpu_cores: true,
      disks: [],
    },
  };
  history = { cpuAggregate: [], networkDown: [], networkUp: [] };
  windowLabel = "settings";
  setSize.mockClear();
  observer = null;
  vi.stubGlobal("ResizeObserver", MockResizeObserver);
  vi.stubGlobal("getComputedStyle", () => ({ paddingTop: "12px", paddingBottom: "12px" }));
});

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

async function renderApp() {
  const { default: App } = await import("./App");
  return render(<App />);
}

it("renders settings instead of a widget for the settings label", async () => {
  windowLabel = "settings";
  const { getByTestId, queryByLabelText } = await renderApp();
  expect(getByTestId("settings")).toBeTruthy();
  expect(queryByLabelText("Zokute dashboard")).toBeNull();
});

it("never sizes the settings window from a section", async () => {
  windowLabel = "settings";
  await renderApp();
  expect(observer).toBeNull();
  expect(setSize).not.toHaveBeenCalled();
});
