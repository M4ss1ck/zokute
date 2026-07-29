import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { SectionConfig, Stats } from "../useStats";
import { ClockWidget } from "./Clock";

afterEach(cleanup);

// 2024-03-05 14:23:07 local time, whatever the machine's zone is: the widget
// reads local time, so the fixture is built locally too.
const AFTERNOON = new Date(2024, 2, 5, 14, 23, 7).getTime();
const MORNING = new Date(2024, 2, 5, 2, 23, 7).getTime();
const MIDNIGHT = new Date(2024, 2, 5, 0, 5, 0).getTime();
const NOON = new Date(2024, 2, 5, 12, 5, 0).getTime();

function section(overrides: Partial<SectionConfig> = {}): SectionConfig {
  return { id: "clock", instance: "clock", enabled: true, monitor: 0, x: 0, y: 0, width: 360, ...overrides };
}

function stats(now_ms: number, text_color?: string): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: [],
    cpu_temperature: null,
    uptime: 0,
    now_ms,
    edit_mode: false,
    system_fields: [],
    config: { opacity: 1, text_color },
    profile: { sections: [], system_fields: [], show_cpu_cores: true, disks: [] },
  };
}

it("pads the hour by default and drops padding on request", () => {
  const { getByText, rerender } = render(<ClockWidget stats={stats(MORNING)} section={section()} />);
  expect(getByText("02:23")).toBeInTheDocument();

  rerender(<ClockWidget stats={stats(MORNING)} section={section({ clock_pad: false })} />);
  expect(getByText("2:23")).toBeInTheDocument();
});

it("keeps minutes and seconds zero-padded even with hour padding off", () => {
  const { getByText } = render(
    <ClockWidget stats={stats(MIDNIGHT)} section={section({ clock_pad: false, clock_seconds: true })} />,
  );
  expect(getByText("12:05:00")).toBeInTheDocument();
});

it("hides seconds by default and shows them when enabled", () => {
  const { getByText, rerender } = render(<ClockWidget stats={stats(AFTERNOON)} section={section()} />);
  expect(getByText("02:23")).toBeInTheDocument();

  rerender(<ClockWidget stats={stats(AFTERNOON)} section={section({ clock_seconds: true })} />);
  expect(getByText("02:23:07")).toBeInTheDocument();
});

it("renders 12-hour time with a meridiem, and midnight as 12 AM", () => {
  const { getByText, rerender } = render(<ClockWidget stats={stats(AFTERNOON)} section={section()} />);
  expect(getByText("PM")).toBeInTheDocument();

  rerender(<ClockWidget stats={stats(MIDNIGHT)} section={section()} />);
  expect(getByText("12:05")).toBeInTheDocument();
  expect(getByText("AM")).toBeInTheDocument();

  rerender(<ClockWidget stats={stats(NOON)} section={section()} />);
  expect(getByText("12:05")).toBeInTheDocument();
  expect(getByText("PM")).toBeInTheDocument();
});

it("hides the meridiem when it is switched off", () => {
  const { queryByText } = render(<ClockWidget stats={stats(AFTERNOON)} section={section({ clock_ampm: false })} />);
  expect(queryByText("PM")).toBeNull();
});

it("renders 24-hour time and never a meridiem in that mode", () => {
  const { getByText, queryByText } = render(
    <ClockWidget stats={stats(AFTERNOON)} section={section({ clock_24h: true, clock_ampm: true })} />,
  );
  expect(getByText("14:23")).toBeInTheDocument();
  expect(queryByText("PM")).toBeNull();
});

it("stacks one unit per line in column layout and keeps one line in row layout", () => {
  const columnSection = section({ clock_layout: "column", clock_seconds: true });
  const { container, rerender } = render(<ClockWidget stats={stats(AFTERNOON)} section={columnSection} />);
  expect(container.querySelectorAll(".clockUnit")).toHaveLength(3);
  expect([...container.querySelectorAll(".clockUnit")].map((unit) => unit.textContent)).toEqual(["02", "23", "07"]);

  rerender(<ClockWidget stats={stats(AFTERNOON)} section={section({ clock_seconds: true })} />);
  expect(container.querySelectorAll(".clockUnit")).toHaveLength(1);
});

it("prefers the clock colour over the global text colour", () => {
  const { container, rerender } = render(<ClockWidget stats={stats(AFTERNOON, "#292824")} section={section()} />);
  expect((container.querySelector(".clock") as HTMLElement).style.color).toBe("rgb(41, 40, 36)");

  rerender(<ClockWidget stats={stats(AFTERNOON, "#292824")} section={section({ clock_color: "#c07100" })} />);
  expect((container.querySelector(".clock") as HTMLElement).style.color).toBe("rgb(192, 113, 0)");
});

it("aligns left by default and follows the chosen alignment", () => {
  const { container, rerender } = render(<ClockWidget stats={stats(AFTERNOON)} section={section()} />);
  const align = () => (container.querySelector(".clock") as HTMLElement).style.getPropertyValue("--clock-align");
  expect(align()).toBe("start");

  rerender(<ClockWidget stats={stats(AFTERNOON)} section={section({ clock_align: "center" })} />);
  expect(align()).toBe("center");

  rerender(<ClockWidget stats={stats(AFTERNOON)} section={section({ clock_align: "right" })} />);
  expect(align()).toBe("end");
});

it("maps the font choice onto a family token", () => {
  const { container, rerender } = render(<ClockWidget stats={stats(AFTERNOON)} section={section()} />);
  expect((container.querySelector(".clock") as HTMLElement).style.getPropertyValue("--clock-font-family")).toBe(
    "var(--font-mono)",
  );

  rerender(<ClockWidget stats={stats(AFTERNOON)} section={section({ clock_font: "sans" })} />);
  expect((container.querySelector(".clock") as HTMLElement).style.getPropertyValue("--clock-font-family")).toBe(
    "var(--font-sans)",
  );
});
