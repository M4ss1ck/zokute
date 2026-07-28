import { useMemo, type CSSProperties } from "react";
import type { SectionConfig, Stats } from "../useStats";

export const CLOCK_WIDTH = { row: 360, column: 160 } as const;

const ALIGNMENT = { left: "start", center: "center", right: "end" } as const;

interface Props {
  stats: Stats;
  section: SectionConfig;
}

function clockParts(date: Date, section: SectionConfig) {
  const hours = date.getHours();
  const display = section.clock_24h ? hours : hours % 12 || 12;
  const units = [
    section.clock_pad === false ? String(display) : String(display).padStart(2, "0"),
    String(date.getMinutes()).padStart(2, "0"),
  ];
  if (section.clock_seconds) units.push(String(date.getSeconds()).padStart(2, "0"));
  const wantsMeridiem = !section.clock_24h && section.clock_ampm !== false;
  return { units, meridiem: wantsMeridiem ? (hours < 12 ? "AM" : "PM") : null };
}

export function ClockWidget({ stats, section }: Props) {
  const { units, meridiem } = useMemo(() => clockParts(new Date(stats.now_ms), section), [stats.now_ms, section.clock_24h, section.clock_ampm, section.clock_seconds, section.clock_pad]);
  const column = section.clock_layout === "column";
  const style = {
    color: section.clock_color ?? stats.config.text_color ?? "#292824",
    "--clock-font-family": section.clock_font === "sans" ? "var(--font-sans)" : "var(--font-mono)",
    "--clock-align": ALIGNMENT[section.clock_align ?? "left"] ?? ALIGNMENT.left,
  } as CSSProperties;
  return (
    <section className={`panel clock ${column ? "clock--column" : "clock--row"}`} style={style}>
      {column ? (
        units.map((unit, index) => (
          <span className="clockUnit" key={index}>
            {unit}
          </span>
        ))
      ) : (
        <span className="clockUnit">{units.join(":")}</span>
      )}
      {meridiem ? <span className="clockMeridiem">{meridiem}</span> : null}
    </section>
  );
}
