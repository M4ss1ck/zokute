import { useMemo, type CSSProperties } from "react";
import type { SectionConfig, Stats } from "../useStats";

const FORMATS: Record<NonNullable<SectionConfig["date_format"]>, Intl.DateTimeFormatOptions> = {
  long: { day: "numeric", month: "long", year: "numeric" },
  short: { day: "numeric", month: "short", year: "numeric" },
  numeric: { day: "2-digit", month: "2-digit", year: "numeric" },
};

interface Props {
  stats: Stats;
  section: SectionConfig;
}

export function DateWidget({ stats, section }: Props) {
  const locale = stats.config.locale ?? undefined;
  const formatter = useMemo(() => {
    const options = {
      ...FORMATS[section.date_format ?? "long"],
      weekday: section.date_weekday === false ? undefined : "long",
    } satisfies Intl.DateTimeFormatOptions;
    return new Intl.DateTimeFormat(locale, options);
  }, [section.date_format, section.date_weekday, locale]);
  const style = {
    color: section.date_color ?? stats.config.text_color ?? "#292824",
  } as CSSProperties;
  return (
    <section className="panel date" style={style}>
      <span className="dateValue">{formatter.format(stats.now_ms)}</span>
    </section>
  );
}
