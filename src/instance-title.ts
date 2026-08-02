/// Instance ids are generated as `spectrum`, `spectrum-2`, `clock-3`. The
/// sidebar and the card sub-headers show them to a person, so they are
/// title-cased for display only — the config keeps the raw id.
export function instanceTitle(instance: string): string {
  return instance
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}
