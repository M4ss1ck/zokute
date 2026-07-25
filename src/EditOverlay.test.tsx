import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { EditOverlay } from "./EditOverlay";

const startDragging = vi.fn(() => Promise.resolve());
const startResizeDragging = vi.fn(() => Promise.resolve());

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({ startDragging, startResizeDragging }),
}));

beforeEach(() => {
  startDragging.mockClear();
  startResizeDragging.mockClear();
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

it("resizes from every edge and corner without also starting a move", () => {
  const onResizeStart = vi.fn();
  const { getByLabelText } = render(<EditOverlay label="cpu" onResizeStart={onResizeStart} />);
  fireEvent.mouseDown(getByLabelText("Resize cpu widget northwest"));
  fireEvent.mouseDown(getByLabelText("Resize cpu widget south"));
  fireEvent.mouseDown(getByLabelText("Resize cpu widget east"));
  expect(startResizeDragging).toHaveBeenCalledWith("NorthWest");
  expect(startResizeDragging).toHaveBeenCalledWith("South");
  expect(startResizeDragging).toHaveBeenCalledWith("East");
  expect(startResizeDragging).toHaveBeenCalledTimes(3);
  expect(onResizeStart.mock.calls.map(([direction]) => direction)).toEqual(["NorthWest", "South", "East"]);
  expect(startDragging).not.toHaveBeenCalled();
});
