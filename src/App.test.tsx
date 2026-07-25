import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

let stats: any;
let history: any;
let windowLabel = "system";
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
  getCurrentWindow: () => ({ label: windowLabel, setSize }),
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

beforeEach(() => {
  stats = {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    cpu_temperature: null,
    uptime: 0,
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
  stats.config.opacity = 0.42;
  history = { cpuAggregate: [], networkDown: [], networkUp: [] };
  windowLabel = "system";
  setSize.mockClear();
  observer = null;
  vi.stubGlobal("ResizeObserver", MockResizeObserver);
});

afterEach(() => {
  cleanup();
  vi.unstubAllGlobals();
});

async function renderApp() {
  const { default: App } = await import("./App");
  return render(<App />);
}

it.each([
  ["system", "system"],
  ["cpu", "cpu"],
  ["memory", "memory"],
  ["disk", "disk"],
  ["network", "network"],
])("renders only the %s widget", async (_, id) => {
  windowLabel = id;
  const { queryByTestId, getByTestId } = await renderApp();
  expect(getByTestId(id)).toBeTruthy();
  for (const other of ["system", "cpu", "memory", "disk", "network"].filter((candidate) => candidate !== id)) {
    expect(queryByTestId(other)).toBeNull();
  }
});

it("renders an empty dashboard for an unknown label", async () => {
  windowLabel = "unknown";
  const { container, getByLabelText } = await renderApp();
  getByLabelText("Zokute dashboard");
  expect(container.querySelector("[data-testid]")).toBeNull();
});

it("applies the configured opacity to the dashboard", async () => {
  const { getByLabelText } = await renderApp();
  const dashboard = getByLabelText("Zokute dashboard");
  expect(dashboard.getAttribute("style")).toContain("--dashboard-opacity: 0.42");
});

it("uses the configured width and re-establishes sizing after width changes", async () => {
  windowLabel = "system";
  const { default: App } = await import("./App");
  const { rerender } = render(<App />);
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger(227.1, 88.4);
  await waitFor(() => {
    expect(setSize).toHaveBeenCalledTimes(1);
  });
  expect(setSize.mock.calls[0][0]).toMatchObject({ width: 401, height: 89 });
  stats.config.sections[0].width = 555;
  rerender(<App />);
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger(227.1, 88.4);
  await waitFor(() => {
    expect(setSize).toHaveBeenCalledTimes(2);
  });
  expect(setSize.mock.calls[1][0]).toMatchObject({ width: 555, height: 89 });
});

it("disconnects and unobserves on unmount", async () => {
  const { unmount } = await renderApp();
  await waitFor(() => expect(observer).not.toBeNull());
  unmount();
  expect(observer?.unobserve).toHaveBeenCalled();
  expect(observer?.disconnect).toHaveBeenCalled();
});
