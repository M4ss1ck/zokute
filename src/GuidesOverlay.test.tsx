import { act, cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";

// vi.mock is hoisted above every import, so the array it captures has to be
// hoisted with it or the factory hits the temporal dead zone.
const listeners = vi.hoisted(
  () => [] as Array<(event: { payload: unknown }) => void>,
);

vi.mock("@tauri-apps/api/event", () => ({
  listen: (_name: string, handler: (event: { payload: unknown }) => void) => {
    listeners.push(handler);
    return Promise.resolve(() => {});
  },
}));

import { GuidesOverlay, isGuidesLabel } from "./GuidesOverlay";

afterEach(() => {
  listeners.length = 0;
  cleanup();
});

it("recognises overlay window labels", () => {
  expect(isGuidesLabel("guides-0")).toBe(true);
  expect(isGuidesLabel("guides-12")).toBe(true);
  expect(isGuidesLabel("clock")).toBe(false);
  expect(isGuidesLabel("settings")).toBe(false);
});

it("starts blank and draws the guides the backend sends", async () => {
  const { container } = render(<GuidesOverlay />);
  expect(container.firstChild).toBeNull();

  await waitFor(() => expect(listeners).toHaveLength(1));
  act(() =>
    listeners[0]({ payload: [{ orientation: "Vertical", position: 960 }] }),
  );

  await waitFor(() => {
    const line = container.querySelector(
      ".alignmentGuideVertical",
    ) as HTMLElement;
    expect(line).not.toBeNull();
    expect(line.style.left).toBe("960px");
  });
});

it("clears the guides when the backend sends an empty list", async () => {
  const { container } = render(<GuidesOverlay />);
  await waitFor(() => expect(listeners).toHaveLength(1));

  act(() =>
    listeners[0]({ payload: [{ orientation: "Vertical", position: 960 }] }),
  );
  await waitFor(() =>
    expect(container.querySelector(".alignmentGuideVertical")).not.toBeNull(),
  );

  act(() => listeners[0]({ payload: [] }));
  await waitFor(() => expect(container.firstChild).toBeNull());
});
