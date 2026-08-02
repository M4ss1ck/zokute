import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { SettingsNav } from "./SettingsNav";
import type { SectionConfig } from "./useStats";

let observed: Element[] = [];
let fire: ((entries: IntersectionObserverEntry[]) => void) | null = null;
const scrollIntoView = vi.fn();

class MockIntersectionObserver {
  constructor(callback: IntersectionObserverCallback) {
    fire = (entries) => callback(entries, this as unknown as IntersectionObserver);
  }
  observe = (element: Element) => { observed.push(element); };
  unobserve = vi.fn();
  disconnect = vi.fn();
}

beforeEach(() => {
  observed = [];
  fire = null;
  scrollIntoView.mockClear();
  vi.stubGlobal("IntersectionObserver", MockIntersectionObserver);
  Element.prototype.scrollIntoView = scrollIntoView;
  const pane = document.createElement("div");
  pane.id = "settings-pane";
  for (const id of ["appearance", "cpu", "visualizer", "instance-spectrum", "instance-spectrum-2", "instance-spectrum,left"]) {
    const anchor = document.createElement("div");
    anchor.id = id;
    pane.appendChild(anchor);
  }
  document.body.appendChild(pane);
});

afterEach(() => {
  cleanup();
  document.body.innerHTML = "";
  vi.unstubAllGlobals();
});

it("lists every section under its group heading", () => {
  const { getByText, getByRole } = render(<SettingsNav sections={[]} />);
  expect(getByText("Overlay")).not.toBeNull();
  expect(getByText("Widgets")).not.toBeNull();
  expect(getByRole("button", { name: "Network" })).not.toBeNull();
});

it("scrolls the pane to a section when its link is clicked", () => {
  const { getByRole } = render(<SettingsNav sections={[]} />);
  fireEvent.click(getByRole("button", { name: "CPU" }));
  expect(scrollIntoView).toHaveBeenCalledTimes(1);
});

it("marks the section that scrolled into view as current", async () => {
  const { getByRole } = render(<SettingsNav sections={[]} />);
  expect(getByRole("button", { name: "Appearance" })).toHaveAttribute("aria-current", "true");
  // The observer callback fires outside React's event system, so React 19
  // batches the state update to a microtask. Existing tests in this repo
  // (src/App.resize.test.tsx) wait the same way after an observer callback.
  fire?.([{ isIntersecting: true, target: document.getElementById("cpu")! } as unknown as IntersectionObserverEntry]);
  await waitFor(() => expect(getByRole("button", { name: "CPU" })).toHaveAttribute("aria-current", "true"));
  expect(getByRole("button", { name: "Appearance" })).not.toHaveAttribute("aria-current");
});

it("observes each rendered section anchor", () => {
  render(<SettingsNav sections={[]} />);
  expect(observed.map((element) => element.id)).toEqual(["appearance", "cpu"]);
});

function visualizers(): SectionConfig[] {
  return [
    { id: "spectrum", instance: "spectrum", enabled: true, monitor: 0, x: 0, y: 0, width: 1920 },
    { id: "spectrum", instance: "spectrum-2", enabled: true, monitor: 0, x: 0, y: 0, width: 1920 },
  ];
}

it("lists each instance as a child of its parent section", () => {
  const { getByRole } = render(<SettingsNav sections={visualizers()} />);
  expect(getByRole("button", { name: "Visualizer" })).not.toBeNull();
  expect(getByRole("button", { name: "Spectrum" })).not.toBeNull();
  expect(getByRole("button", { name: "Spectrum 2" })).not.toBeNull();
});

it("omits a parent whose widget has no instances", () => {
  const { queryByRole } = render(<SettingsNav sections={[]} />);
  expect(queryByRole("button", { name: "Visualizer" })).toBeNull();
  expect(queryByRole("button", { name: "Clock" })).toBeNull();
});

it("scrolls to the instance anchor when a child is clicked", () => {
  const { getByRole } = render(<SettingsNav sections={visualizers()} />);
  fireEvent.click(getByRole("button", { name: "Spectrum 2" }));
  expect(scrollIntoView).toHaveBeenCalledTimes(1);
  // `scrollIntoView` is stubbed on Element.prototype, so the element it was
  // called on is the mock's `this` context, not an instance.
  expect(scrollIntoView.mock.contexts[0]).toBe(document.getElementById("instance-spectrum-2"));
});

it("marks the parent current while one of its children is in view", async () => {
  const { getByRole } = render(<SettingsNav sections={visualizers()} />);
  fire?.([{ isIntersecting: true, target: document.getElementById("instance-spectrum-2")! } as unknown as IntersectionObserverEntry]);
  await waitFor(() => expect(getByRole("button", { name: "Spectrum 2" })).toHaveAttribute("aria-current", "true"));
  expect(getByRole("button", { name: "Visualizer" })).toHaveAttribute("aria-current", "true");
  expect(getByRole("button", { name: "Spectrum" })).not.toHaveAttribute("aria-current");
});

it("tracks a parent through its children rather than its own anchor", () => {
  render(<SettingsNav sections={visualizers()} />);
  expect(observed.map((element) => element.id)).toEqual([
    "appearance", "cpu", "instance-spectrum", "instance-spectrum-2",
  ]);
});

it("tracks a dynamic instance whose id contains a comma", () => {
  const commaInstance: SectionConfig[] = [
    { id: "spectrum", instance: "spectrum,left", enabled: true, monitor: 0, x: 0, y: 0, width: 1920 },
  ];
  render(<SettingsNav sections={commaInstance} />);
  expect(observed.map((element) => element.id)).toContain("instance-spectrum,left");
});

it("distinguishes colliding Clock and Date parent and child names", () => {
  const sections: SectionConfig[] = [
    { id: "clock", instance: "clock", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
    { id: "date", instance: "date", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
  ];
  const { getByRole } = render(<SettingsNav sections={sections} />);
  expect(getByRole("button", { name: "Clock settings" })).toHaveTextContent("Clock");
  expect(getByRole("button", { name: "Clock instance" })).toHaveTextContent("Clock");
  expect(getByRole("button", { name: "Date settings" })).toHaveTextContent("Date");
  expect(getByRole("button", { name: "Date instance" })).toHaveTextContent("Date");
});
