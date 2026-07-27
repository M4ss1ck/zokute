import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";
import type { SectionConfig, Stats } from "../useStats";
import { DateWidget } from "./Date";

afterEach(cleanup);

const DAY = new Date(2024, 2, 5, 14, 23, 7).getTime();

function section(overrides: Partial<SectionConfig> = {}): SectionConfig {
  return { id: "date", instance: "date", enabled: true, monitor: 0, x: 0, y: 0, width: 360, ...overrides };
}

function stats(text_color?: string): Stats {
  return {
    cpu: { aggregate_percent: 0, core_percents: [] },
    memory: { used_bytes: 0, total_bytes: 0, swap_used_bytes: 0, swap_total_bytes: 0 },
    disks: [],
    network: { down_bytes_per_second: 0, up_bytes_per_second: 0 },
    cpu_temperature: null,
    uptime: 0,
    now_ms: DAY,
    edit_mode: false,
    system_fields: [],
    config: { opacity: 1, text_color, sections: [], system_fields: [], show_cpu_cores: true, disks: [] },
  };
}

function formatted(format: "long" | "short" | "numeric", weekday = true) {
  const month = format === "long" ? "long" : format === "short" ? "short" : "2-digit";
  return new Intl.DateTimeFormat(undefined, {
    day: format === "numeric" ? "2-digit" : "numeric",
    month,
    year: "numeric",
    weekday: weekday ? "long" : undefined,
  }).format(DAY);
}

it("shows a long date with the weekday by default", () => {
  const { getByText } = render(<DateWidget stats={stats()} section={section()} />);
  expect(getByText(formatted("long"))).toBeInTheDocument();
});

it("formats each configured variant and can hide the weekday", () => {
  const { getByText, rerender } = render(
    <DateWidget stats={stats()} section={section({ date_format: "short", date_weekday: false })} />,
  );
  expect(getByText(formatted("short", false))).toBeInTheDocument();
  rerender(<DateWidget stats={stats()} section={section({ date_format: "numeric" })} />);
  expect(getByText(formatted("numeric"))).toBeInTheDocument();
});

it("prefers the instance colour over the global text colour", () => {
  const { container, rerender } = render(<DateWidget stats={stats("#292824")} section={section()} />);
  expect((container.querySelector(".date") as HTMLElement).style.color).toBe("rgb(41, 40, 36)");
  rerender(<DateWidget stats={stats("#292824")} section={section({ date_color: "#c07100" })} />);
  expect((container.querySelector(".date") as HTMLElement).style.color).toBe("rgb(192, 113, 0)");
});
