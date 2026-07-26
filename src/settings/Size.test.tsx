import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { SizePreferences } from "./Size";
import type { StatsConfig } from "../useStats";

afterEach(cleanup);

function config(): StatsConfig {
  return {
    opacity: 1,
    text_opacity: 1,
    text_color: "#292824",
    show_background: true,
    show_cpu_cores: true,
    system_fields: [],
    disks: [],
    sections: [
      { id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360, scale: 1 },
      { id: "disk", instance: "disk", enabled: false, monitor: 0, x: 0, y: 0, width: 360, scale: 1 },
    ],
  } as unknown as StatsConfig;
}

it("lists only enabled widgets", () => {
  const { queryByRole } = render(<SizePreferences config={config()} onChange={vi.fn()} />);
  expect(queryByRole("slider", { name: "cpu" })).not.toBeNull();
  expect(queryByRole("slider", { name: "disk" })).toBeNull();
});

it("scales the widget whose slider moved and leaves the others alone", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<SizePreferences config={config()} onChange={onChange} />);
  const slider = getByRole("slider", { name: "cpu" }) as HTMLInputElement;
  fireEvent.mouseDown(slider.closest(".settingsSliderThumb") as HTMLElement, { button: 0 });
  fireEvent.change(slider, { target: { value: "2.5" } });
  const next = onChange.mock.calls.at(-1)?.[0] as StatsConfig;
  expect(next.sections[0].scale).toBe(2.5);
  expect(next.sections[1].scale).toBe(1);
});

it("offers zoom well past the range the old drag gesture allowed", () => {
  const { getByRole } = render(<SizePreferences config={config()} onChange={vi.fn()} />);
  const slider = getByRole("slider", { name: "cpu" });
  expect(slider.getAttribute("min")).toBe("0.25");
  expect(slider.getAttribute("max")).toBe("4");
});

it("renders nothing when no widget is enabled", () => {
  const empty = { ...config(), sections: [] };
  const { container } = render(<SizePreferences config={empty} onChange={vi.fn()} />);
  expect(container.firstChild).toBeNull();
});
