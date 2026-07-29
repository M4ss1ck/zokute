import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

let stats: any;
let history: any;
let windowLabel = "system";
let observer: MockResizeObserver | null = null;
const setSize = vi.fn(() => Promise.resolve());
const setResizable = vi.fn(() => Promise.resolve());
const invoke = vi.fn(() => Promise.resolve());
let startResize: ((direction: string) => void) | undefined;

class MockResizeObserver {
  callback: ResizeObserverCallback;
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();

  constructor(callback: ResizeObserverCallback) {
    this.callback = callback;
    observer = this;
  }

  trigger(contentWidth: number, contentHeight: number, borderBoxWidth = contentWidth, borderBoxHeight = contentHeight) {
    this.callback(
      [
        {
          contentRect: { width: contentWidth, height: contentHeight },
          borderBoxSize: [{ inlineSize: borderBoxWidth, blockSize: borderBoxHeight }],
        } as ResizeObserverEntry,
      ],
      this as unknown as ResizeObserver,
    );
  }
}

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ label: windowLabel, setSize, setResizable }),
}));

vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...(args as [])) }));

vi.mock("@tauri-apps/api/dpi", () => ({
  LogicalSize: class LogicalSize {
    constructor(public width: number, public height: number) {}
  },
}));

vi.mock("./useStats", () => ({ default: () => ({ stats, history }) }));
vi.mock("./EditOverlay", () => ({
  EditOverlay: (props: { onResizeStart?: (direction: string) => void }) => {
    startResize = props.onResizeStart;
    return <div data-testid="edit-overlay" />;
  },
}));
vi.mock("./widgets/System", () => ({ SystemWidget: () => <div data-testid="system" /> }));
vi.mock("./widgets/Cpu", () => ({ CpuWidget: () => <div data-testid="cpu" /> }));
vi.mock("./widgets/Memory", () => ({ MemoryWidget: () => <div data-testid="memory" /> }));
vi.mock("./widgets/Disk", () => ({ DiskWidget: () => <div data-testid="disk" /> }));
vi.mock("./widgets/Network", () => ({ NetworkWidget: () => <div data-testid="network" /> }));

beforeEach(() => {
  stats = {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: [],
    cpu_temperature: null,
    uptime: 0,
    edit_mode: false,
    system_fields: [],
    config: {
      opacity: 1,
    },
    profile: {
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
  windowLabel = "cpu";
  setSize.mockClear();
  setResizable.mockClear();
  observer = null;
  startResize = undefined;
  invoke.mockClear();
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

it("keeps clock zoom in backend during a native drag even without mouseup", async () => {
  stats.edit_mode = true;
  stats.profile.sections.push({ id: "clock", instance: "clock", enabled: true, monitor: 0, x: 0, y: 0, width: 400, scale: 1 });
  windowLabel = "clock";
  Object.defineProperty(globalThis, "innerWidth", { writable: true, value: 400 });
  const { getByLabelText } = await renderApp();
  await waitFor(() => expect(startResize).toBeDefined());
  startResize?.("SouthEast");
  Object.defineProperty(globalThis, "innerWidth", { writable: true, value: 3200 });
  globalThis.dispatchEvent(new Event("resize"));
  await waitFor(() =>
    expect(getByLabelText("Zokute dashboard").getAttribute("style")).toContain("--dashboard-scale: 8"),
  );
  expect(invoke).toHaveBeenCalledWith("update_widget_scale", { id: "clock", scale: 8 });
});

it("leaves zoom alone when an edge handle is dragged", async () => {
  stats.edit_mode = true;
  Object.defineProperty(globalThis, "innerWidth", { writable: true, value: 400 });
  const { getByLabelText } = await renderApp();
  await waitFor(() => expect(startResize).toBeDefined());
  startResize?.("East");
  Object.defineProperty(globalThis, "innerWidth", { writable: true, value: 900 });
  globalThis.dispatchEvent(new Event("resize"));
  globalThis.dispatchEvent(new Event("mouseup"));
  expect(getByLabelText("Zokute dashboard").getAttribute("style")).toContain("--dashboard-scale: 1");
  expect(invoke).not.toHaveBeenCalled();
});
