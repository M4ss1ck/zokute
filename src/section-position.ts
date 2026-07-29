import type { SectionConfig } from "./useStats";

export type SectionPosition =
  | { type: "Absolute"; monitor_identity: string; x: number; y: number }
  | { type: "Anchored"; monitor_identity: string; anchor: string; offset_x?: number; offset_y?: number };

/// Where a widget actually sits.
///
/// `x` and `y` are skip_serializing on the Rust side, so a profile read back
/// from disk always reports 0. The stored `position` is the real placement, and
/// reading it is what stops the layout editor showing 0,0 for every widget and
/// stops an arrow key sending the widget to the corner instead of nudging it.
export function sectionPosition(section: SectionConfig): { x: number; y: number } {
  const position = section.position;
  if (position?.type === "Absolute") {
    return { x: position.x, y: position.y };
  }
  if (position?.type === "Anchored") {
    return { x: position.offset_x ?? 0, y: position.offset_y ?? 0 };
  }
  return { x: section.x ?? 0, y: section.y ?? 0 };
}
