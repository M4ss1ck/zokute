import { cleanup, render } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { makeHistory, makeStats } from "./app-test-fixture";

let stats: any;
let history: any;
let windowLabel = "system";

class MockResizeObserver {
  observe = vi.fn();
  unobserve = vi.fn();
  disconnect = vi.fn();
}

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    label: windowLabel,
    setSize: vi.fn(() => Promise.resolve()),
    setResizable: vi.fn(() => Promise.resolve()),
  }),
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
  stats = makeStats();
  history = makeHistory();
  windowLabel = "system";
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

it("renders a panel window through the standalone dashboard wrapper with both children", async () => {
  windowLabel = "panel-1";
  stats.profile.sections.push({
    id: "panel",
    instance: "panel-1",
    enabled: true,
    monitor: 0,
    x: 0,
    y: 0,
    width: 500,
    children: [
      { id: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 401 },
      { id: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 402 },
    ],
  });
  const { container, getByTestId } = await renderApp();
  getByTestId("system");
  getByTestId("cpu");
  expect(container.querySelector(".dashboard")).toBeTruthy();
});
