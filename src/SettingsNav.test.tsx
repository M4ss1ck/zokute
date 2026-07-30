import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { SettingsNav } from "./SettingsNav";

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
  for (const id of ["appearance", "cpu"]) {
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
  const { getByText, getByRole } = render(<SettingsNav />);
  expect(getByText("Overlay")).not.toBeNull();
  expect(getByText("Widgets")).not.toBeNull();
  expect(getByRole("button", { name: "Network" })).not.toBeNull();
});

it("scrolls the pane to a section when its link is clicked", () => {
  const { getByRole } = render(<SettingsNav />);
  fireEvent.click(getByRole("button", { name: "CPU" }));
  expect(scrollIntoView).toHaveBeenCalledTimes(1);
});

it("marks the section that scrolled into view as current", async () => {
  const { getByRole } = render(<SettingsNav />);
  expect(getByRole("button", { name: "Appearance" })).toHaveAttribute("aria-current", "true");
  // The observer callback fires outside React's event system, so React 19
  // batches the state update to a microtask. Existing tests in this repo
  // (src/App.resize.test.tsx) wait the same way after an observer callback.
  fire?.([{ isIntersecting: true, target: document.getElementById("cpu")! } as IntersectionObserverEntry]);
  await waitFor(() => expect(getByRole("button", { name: "CPU" })).toHaveAttribute("aria-current", "true"));
  expect(getByRole("button", { name: "Appearance" })).not.toHaveAttribute("aria-current");
});

it("observes each rendered section anchor", () => {
  render(<SettingsNav />);
  expect(observed.map((element) => element.id)).toEqual(["appearance", "cpu"]);
});
