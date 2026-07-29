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

it("moves the window when the drag surface is pressed", async () => {
  const { getByLabelText, queryByText } = render(<EditOverlay label="cpu" />);
  fireEvent.mouseDown(getByLabelText("Move cpu widget"));
  await waitFor(() => expect(startDragging).toHaveBeenCalledTimes(1));
  expect(startResizeDragging).not.toHaveBeenCalled();
  expect(queryByText("cpu")).toBeNull();
});

it("reports the drag so exiting edit mode keeps the new position", () => {
  const { getByLabelText } = render(<EditOverlay label="spectrum-2" />);
  fireEvent.mouseDown(getByLabelText("Move spectrum-2 widget"));
  expect(invoke).toHaveBeenCalledWith("mark_widget_moved", { id: "spectrum-2" });
});

it("reports a resize the same way a drag is reported", () => {
  const { getByLabelText } = render(<EditOverlay label="spectrum-2" />);
  fireEvent.mouseDown(getByLabelText("Resize spectrum-2 widget south"));
  expect(invoke).toHaveBeenCalledWith("mark_widget_moved", { id: "spectrum-2" });
});

it("stays silent when the widget is only shown, never grabbed", () => {
  render(<EditOverlay label="spectrum-2" />);
  expect(invoke).not.toHaveBeenCalled();
});

it("removes its own widget instance from the hover control", async () => {
  const { getByRole } = render(<EditOverlay label="cpu-2" />);
  fireEvent.click(getByRole("button", { name: "Remove cpu-2 widget" }));
  expect(invoke).toHaveBeenCalledWith("remove_widget", { instance: "cpu-2" });
  await waitFor(() => expect(getByRole("button", { name: "Remove cpu-2 widget" })).toBeDisabled());
});

it("resizes from every edge and corner without also starting a move", async () => {
  const { getByLabelText } = render(<EditOverlay label="cpu" />);
  fireEvent.mouseDown(getByLabelText("Resize cpu widget northwest"));
  fireEvent.mouseDown(getByLabelText("Resize cpu widget south"));
  fireEvent.mouseDown(getByLabelText("Resize cpu widget east"));
  await waitFor(() => expect(startResizeDragging).toHaveBeenCalledWith("NorthWest"));
  expect(startResizeDragging).toHaveBeenCalledWith("South");
  expect(startResizeDragging).toHaveBeenCalledWith("East");
  expect(startResizeDragging).toHaveBeenCalledTimes(3);
  expect(startDragging).not.toHaveBeenCalled();
});

it("hides the reset control while the widget is at its natural size", () => {
  const { queryByRole } = render(<EditOverlay label="cpu" scale={1} onReset={vi.fn()} />);
  expect(queryByRole("button", { name: "Reset cpu widget size" })).toBeNull();
});

it("offers a reset once the size has been changed", () => {
  const onReset = vi.fn();
  const { getByRole } = render(<EditOverlay label="cpu" scale={2.4} onReset={onReset} />);
  fireEvent.click(getByRole("button", { name: "Reset cpu widget size" }));
  expect(onReset).toHaveBeenCalledTimes(1);
});

it("offers a reset when the widget has been shrunk too", () => {
  const { getByRole } = render(<EditOverlay label="cpu" scale={0.6} onReset={vi.fn()} />);
  expect(getByRole("button", { name: "Reset cpu widget size" })).toBeTruthy();
});
