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
  const { getByLabelText } = render(<EditOverlay label="cpu" />);
  fireEvent.mouseDown(getByLabelText("Move cpu widget"));
  expect(startDragging).toHaveBeenCalledTimes(1);
  expect(startResizeDragging).not.toHaveBeenCalled();
});

it("resizes east only, without also starting a move", () => {
  const { getByLabelText } = render(<EditOverlay label="cpu" />);
  fireEvent.mouseDown(getByLabelText("Resize cpu widget width"));
  expect(startResizeDragging).toHaveBeenCalledWith("East");
  expect(startDragging).not.toHaveBeenCalled();
});
