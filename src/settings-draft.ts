import type { StatsConfig, StatsProfile } from "./useStats";

export interface MergedDraft {
  config: StatsConfig;
  profile: StatsProfile;
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
