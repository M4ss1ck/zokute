import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { EditOverlay } from "./EditOverlay";

const startDragging = vi.fn(() => Promise.resolve());
const startResizeDragging = vi.fn(() => Promise.resolve());
const invoke = vi.fn(() => Promise.resolve());

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ startDragging, startResizeDragging }),
}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...args) }));

beforeEach(() => {
  startDragging.mockClear();
  startResizeDragging.mockClear();
  invoke.mockClear();
});

afterEach(() => {
  cleanup();
});

it("moves the window when the drag surface is pressed", () => {
  const { getByLabelText, queryByText } = render(<EditOverlay label="cpu" />);
  fireEvent.mouseDown(getByLabelText("Move cpu widget"));
  expect(startDragging).toHaveBeenCalledTimes(1);
  expect(startResizeDragging).not.toHaveBeenCalled();
  expect(queryByText("cpu")).toBeNull();
});

it("removes its own widget instance from the hover control", async () => {
  const { getByRole } = render(<EditOverlay label="cpu-2" />);
  fireEvent.click(getByRole("button", { name: "Remove cpu-2 widget" }));
  expect(invoke).toHaveBeenCalledWith("remove_widget", { instance: "cpu-2" });
  await waitFor(() => expect(getByRole("button", { name: "Remove cpu-2 widget" })).toBeDisabled());
});

it("resizes from every edge and corner without also starting a move", () => {
  const { getByLabelText } = render(<EditOverlay label="cpu" />);
  fireEvent.mouseDown(getByLabelText("Resize cpu widget northwest"));
  fireEvent.mouseDown(getByLabelText("Resize cpu widget south"));
  fireEvent.mouseDown(getByLabelText("Resize cpu widget east"));
  expect(startResizeDragging).toHaveBeenCalledWith("NorthWest");
  expect(startResizeDragging).toHaveBeenCalledWith("South");
  expect(startResizeDragging).toHaveBeenCalledWith("East");
  expect(startResizeDragging).toHaveBeenCalledTimes(3);
  expect(startDragging).not.toHaveBeenCalled();
});
