import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

let stats: any;
let history: any;
let windowLabel = "system";
let observer: MockResizeObserver | null = null;
const setSize = vi.fn(() => Promise.resolve());

class MockResizeObserver {
  callback: ResizeObserverCallback;
  active = true;
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn(() => {
    this.active = false;
  });

  constructor(callback: ResizeObserverCallback) {
    this.callback = callback;
    observer = this;
  }

  trigger(borderBoxWidth: number, borderBoxHeight = borderBoxWidth) {
    if (!this.active) return;
    this.callback(
      [{ borderBoxSize: [{ inlineSize: borderBoxWidth, blockSize: borderBoxHeight }] } as ResizeObserverEntry],
      this as unknown as ResizeObserver,
    );
  }
}

vi.mock("@tauri-apps/api/window", () => ({ getCurrentWindow: () => ({ label: windowLabel, setSize, setResizable: vi.fn(() => Promise.resolve()) }) }));
vi.mock("@tauri-apps/api/dpi", () => ({ LogicalSize: class LogicalSize { constructor(public width: number, public height: number) {} } }));
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
    edit_mode: false,
    system_fields: [],
    config: {
      opacity: 0.42,
    },
    profile: {
      sections: [
        { id: "system", instance: "system", enabled: false, monitor: 0, x: 0, y: 0, width: 401 },
        { id: "system", instance: "system-2", enabled: true, monitor: 0, x: 0, y: 0, width: 777 },
      ],
      system_fields: [],
      show_cpu_cores: true,
      disks: [],
    },
  };
  history = { cpuAggregate: [], networkDown: [], networkUp: [] };
  windowLabel = "system-2";
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

it("uses the section matching the current instance label", async () => {
  const { getByTestId } = await renderApp();
  expect(getByTestId("system")).toBeTruthy();
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger(777, 88.4);
  await waitFor(() => expect(setSize).toHaveBeenCalledTimes(1));
  expect(setSize.mock.calls[0][0]).toMatchObject({ width: 777, height: 89 });
});

it("disconnects when the matching instance becomes disabled", async () => {
  const { default: App } = await import("./App");
  const { queryByTestId, rerender } = render(<App />);
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger(777.2, 88.4);
  await waitFor(() => expect(setSize).toHaveBeenCalledTimes(1));
  stats.profile.sections[1].enabled = false;
  rerender(<App />);
  expect(queryByTestId("system")).toBeNull();
  expect(observer?.unobserve).toHaveBeenCalled();
  expect(observer?.disconnect).toHaveBeenCalled();
  observer?.trigger(777, 99.9);
  expect(setSize).toHaveBeenCalledTimes(1);
});
