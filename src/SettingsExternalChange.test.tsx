import { act, cleanup, fireEvent, render, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import type { StatsConfig, StatsProfile } from "./useStats";
import { SettingsExternalChange } from "./SettingsExternalChange";

const external = { config: { opacity: 0.4 }, profile: { sections: [] } };
const invoke = vi.fn((command: string) =>
  Promise.resolve(command === "accept_external_config" ? external : undefined),
);
const handlers = new Map<string, () => void>();
const unlisten = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({ invoke: (command: string) => invoke(command) }));
vi.mock("@tauri-apps/api/event", () => ({
  listen: (event: string, handler: () => void) => {
    handlers.set(event, handler);
    return Promise.resolve(unlisten);
  },
}));

beforeEach(() => {
  invoke.mockClear();
  handlers.clear();
  unlisten.mockClear();
});
afterEach(cleanup);

it("reloads the accepted external state", async () => {
  const onReload = vi.fn();
  const { getByRole } = render(<SettingsExternalChange onReload={onReload} />);
  act(() => handlers.get("external-config-changed")?.());
  fireEvent.click(getByRole("button", { name: "Reload from disk" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("accept_external_config"));
  expect(onReload).toHaveBeenCalledWith(
    external.config as StatsConfig,
    external.profile as unknown as StatsProfile,
  );
});

it("keeps the draft by dismissing the profile candidate", async () => {
  const { getByRole, queryByRole, unmount } = render(
    <SettingsExternalChange onReload={vi.fn()} />,
  );
  act(() => handlers.get("external-profile-changed")?.());
  fireEvent.click(getByRole("button", { name: "Keep my draft" }));
  await waitFor(() => expect(invoke).toHaveBeenCalledWith("dismiss_external_config"));
  await waitFor(() => expect(queryByRole("status")).toBeNull());
  unmount();
  await waitFor(() => expect(unlisten).toHaveBeenCalledTimes(2));
});
