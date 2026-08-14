import { cleanup, fireEvent, render } from "@testing-library/react";
import { useState } from "react";
import { afterEach, expect, it, vi } from "vitest";
import type { MergedConfig, SectionConfig } from "../useStats";
import { PanelPreferences } from "./Panels";

// This project does not set vitest `globals`, so Testing Library never
// registers its automatic cleanup and renders would otherwise accumulate.
afterEach(cleanup);

function config(sections: SectionConfig[]): MergedConfig {
  return {
    opacity: 1,
    sections,
    system_fields: [],
    show_cpu_cores: true,
    disks: [],
  };
}

function panel(overrides: Partial<SectionConfig> = {}): SectionConfig {
  return {
    id: "panel",
    instance: "panel",
    enabled: true,
    show_header: true,
    monitor: 0,
    x: 24,
    y: 24,
    width: 360,
    scale: 1,
    children: [],
    panel_gap: 4,
    panel_padding: 8,
    ...overrides,
  };
}

function child(id: string, instance: string): SectionConfig {
  return { id, instance, enabled: true, show_header: true, monitor: 0, x: 0, y: 0, width: 360, scale: 1 };
}

// Feeds each onChange back into the component so mutations accumulate the
// way they would through SettingsSections, which owns the draft state.
function Harness({ initial, onChange }: { initial: MergedConfig; onChange: (next: MergedConfig) => void }) {
  const [current, setCurrent] = useState(initial);
  return (
    <PanelPreferences
      config={current}
      onChange={(next) => {
        setCurrent(next);
        onChange(next);
      }}
    />
  );
}

it("renders the empty message when no panel is configured", () => {
  const { getByText } = render(<PanelPreferences config={config([])} onChange={vi.fn()} />);
  expect(getByText("No panels configured.")).toBeTruthy();
});

it("adds a panel section with its defaults", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<Harness initial={config([])} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Add panel" }));
  expect(onChange).toHaveBeenCalledTimes(1);
  expect(onChange.mock.calls[0][0].sections).toEqual([
    { id: "panel", instance: "panel", enabled: true, show_header: true, monitor: 0, x: 24, y: 24, width: 360, scale: 1, children: [], panel_gap: 4, panel_padding: 8, panel_dividers: true },
  ]);
});

it("labels a second panel differently from the first", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<Harness initial={config([])} onChange={onChange} />);
  const add = getByRole("button", { name: "Add panel" });
  fireEvent.click(add);
  fireEvent.click(add);
  const first = onChange.mock.calls[0][0].sections;
  const second = onChange.mock.calls[1][0].sections;
  expect(first[first.length - 1].instance).toBe("panel");
  expect(second[second.length - 1].instance).toBe("panel-2");
});

it("appends a cpu child to the panel and leaves other sections alone", () => {
  const onChange = vi.fn();
  const initial = config([
    panel(),
    { id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
  ]);
  const { getByRole } = render(<Harness initial={initial} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Add cpu to panel" }));
  const sections = onChange.mock.calls[0][0].sections;
  expect(sections).toHaveLength(2);
  expect(sections[0]).toEqual({ ...panel(), children: [child("cpu", "cpu")] });
  expect(sections[1]).toEqual({ id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 });
});

it("does not reuse a top-level section label for a child of the same kind", () => {
  const onChange = vi.fn();
  const initial = config([
    { id: "cpu", instance: "cpu", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
    panel(),
  ]);
  const { getByRole } = render(<Harness initial={initial} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Add cpu to panel" }));
  const sections = onChange.mock.calls[0][0].sections;
  expect(sections[0].instance).toBe("cpu");
  expect(sections[1].children![0].instance).toBe("cpu-2");
});

it("removes only the chosen child", () => {
  const onChange = vi.fn();
  const initial = config([
    panel({ children: [child("cpu", "cpu"), child("memory", "memory")] }),
    { id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
  ]);
  const { getByRole } = render(<Harness initial={initial} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Remove cpu from panel" }));
  const sections = onChange.mock.calls[0][0].sections;
  expect(sections[0].children).toEqual([initial.sections[0].children![1]]);
  expect(sections[1]).toEqual(initial.sections[1]);
});

it("removes only the chosen panel", () => {
  const onChange = vi.fn();
  const initial = config([
    panel(),
    panel({ instance: "panel-2", x: 48, y: 48 }),
    { id: "system", instance: "system", enabled: true, monitor: 0, x: 0, y: 0, width: 360 },
  ]);
  const { getByRole } = render(<Harness initial={initial} onChange={onChange} />);
  fireEvent.click(getByRole("button", { name: "Remove panel panel-2" }));
  const sections = onChange.mock.calls[0][0].sections;
  expect(sections.map((section: SectionConfig) => section.instance)).toEqual(["panel", "system"]);
});

it("writes panel_gap when the gap slider moves", () => {
  const onChange = vi.fn();
  const { getByRole } = render(<Harness initial={config([panel()])} onChange={onChange} />);
  const slider = getByRole("slider", { name: "Gap" }) as HTMLInputElement;
  const thumb = slider.closest(".settingsSliderThumb") as HTMLElement;
  fireEvent.mouseDown(thumb, { button: 0 });
  fireEvent.change(slider, { target: { value: "16" } });
  expect(onChange).toHaveBeenCalled();
  const sections = onChange.mock.calls.at(-1)![0].sections;
  expect(sections[0]).toMatchObject({ panel_gap: 16, panel_padding: 8 });
});
