import type { StatsConfig, StatsProfile } from "./useStats";

export interface MergedDraft {
  config: StatsConfig;
  profile: StatsProfile;
  autostart: boolean;
}

/// Drops the section a "widget-removed" event names, matching on instance and
/// falling back to id for sections predating per-instance ids.
export function withoutSection(draft: MergedDraft | null, instance: string): MergedDraft | null {
  if (!draft) return draft;
  return {
    ...draft,
    profile: {
      ...draft.profile,
      sections: draft.profile.sections.filter((section) => (section.instance ?? section.id) !== instance),
    },
  };
}

/// Deep-equal rather than JSON.stringify: spreading a config appends keys
/// that were absent, so a stringify comparison would report a false change.
function same(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  const left = a as Record<string, unknown>;
  const right = b as Record<string, unknown>;
  const keys = new Set([...Object.keys(left), ...Object.keys(right)]);
  for (const key of keys) {
    if (!same(left[key], right[key])) return false;
  }
  return true;
}

export function isDirty(draft: MergedDraft, baseline: MergedDraft): boolean {
  return !same(draft, baseline);
}
