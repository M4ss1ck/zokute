import { cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { StartupToggle } from "./Startup";

const invoke = vi.fn((command: string) =>
  command === "autostart_enabled" ? Promise.resolve(true) : Promise.resolve(),
);
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => invoke(...args) }));

afterEach(cleanup);
afterEach(() => invoke.mockClear());

it("reflects the state reported by the backend", async () => {
  const { getByLabelText } = render(<StartupToggle />);
  await waitFor(() =>
    expect((getByLabelText("Start with the session") as HTMLInputElement).checked).toBe(true),
  );
  expect(invoke).toHaveBeenCalledWith("autostart_enabled");
});

it("writes the new state and updates immediately", async () => {
  const { getByLabelText } = render(<StartupToggle />);
  const toggle = getByLabelText("Start with the session") as HTMLInputElement;
  await waitFor(() => expect(toggle.checked).toBe(true));
  fireEvent.click(toggle);
  expect(invoke).toHaveBeenCalledWith("set_autostart", { enabled: false });
  await waitFor(() => expect(toggle.checked).toBe(false));
});
