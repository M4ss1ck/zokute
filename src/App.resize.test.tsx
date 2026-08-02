import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

let stats: any;
let history: any;
let windowLabel = "system";
let observer: MockResizeObserver | null = null;
const events: string[] = [];
let resizeFailures = 0;
let relockFailures = 0;

class MockResizeObserver {
  callback: ResizeObserverCallback;
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();

  constructor(callback: ResizeObserverCallback) {
    this.callback = callback;
    observer = this;
  }

  trigger() {
    this.callback([{ borderBoxSize: [{ inlineSize: 999.1, blockSize: 88.4 }] } as unknown as ResizeObserverEntry], this as unknown as ResizeObserver);
  }
}

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    label: windowLabel,
    setSize: (size: { width: number; height: number }) => {
      events.push(`size:${size.width}x${size.height}`);
      if (resizeFailures > 0) {
        resizeFailures -= 1;
        return Promise.reject(new Error("boom"));
      }
      return Promise.resolve();
    },
    setResizable: (value: boolean) => {
      events.push(`resizable:${value}`);
      if (!value && relockFailures > 0) {
        relockFailures -= 1;
        return Promise.reject(new Error("relock failed"));
      }
      return Promise.resolve();
    },
  }),
}));

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
    network: [],
    cpu_temperature: null,
    uptime: 0,
    edit_mode: false,
    system_fields: [],
    config: { opacity: 0.42 },
    profile: { sections: [{ id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 401 }], system_fields: [], show_cpu_cores: true, disks: [] },
  };
  history = { cpuAggregate: [], networkDown: [], networkUp: [] };
  windowLabel = "system";
  observer = null;
  events.length = 0;
  resizeFailures = 0;
  relockFailures = 0;
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

it("marks the dashboard when its background is hidden", async () => {
  stats.config.show_background = false;
  const { getByLabelText } = await renderApp();
  expect(getByLabelText("Zokute dashboard").classList).toContain("dashboard--background-hidden");
});

it("applies a persisted scale to the card and measured height", async () => {
  stats.profile.sections[0].scale = 1.5;
  const { getByLabelText } = await renderApp();
  expect(getByLabelText("Zokute dashboard").getAttribute("style")).toContain("--dashboard-scale: 1.5");
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger();
  await waitFor(() => expect(events).toContain("size:401x169"));
});

it("relocks after a failed programmatic resize and retries later", async () => {
  resizeFailures = 1;
  await renderApp();
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger();
  await waitFor(() => expect(events).toContain("resizable:false"));
  expect(events.slice(0, 3)).toEqual(["resizable:true", "size:401x113", "resizable:false"]);
  observer?.trigger();
  await waitFor(() => expect(events.filter((event) => event === "resizable:true")).toHaveLength(2));
  expect(events.slice(3)).toEqual(["resizable:true", "size:401x113", "resizable:false"]);
});

it("retries the same size when relocking fails", async () => {
  relockFailures = 1;
  await renderApp();
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger();
  await waitFor(() => expect(events).toContain("resizable:false"));
  expect(events.slice(0, 3)).toEqual(["resizable:true", "size:401x113", "resizable:false"]);
  observer?.trigger();
  await waitFor(() => expect(events.filter((event) => event === "size:401x113")).toHaveLength(2));
  expect(events.filter((event) => event === "resizable:true")).toHaveLength(2);
});

it("honours a persisted height taller than the content", async () => {
  stats.profile.sections[0].height = 300;
  await renderApp();
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger();
  await waitFor(() => expect(events).toContain("size:401x300"));
});

it("floors the window height at the content height", async () => {
  stats.profile.sections[0].height = 40;
  await renderApp();
  await waitFor(() => expect(observer).not.toBeNull());
  observer?.trigger();
  await waitFor(() => expect(events).toContain("size:401x113"));
});
