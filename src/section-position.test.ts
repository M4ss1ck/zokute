import { expect, it } from "vitest";
import { sectionPosition } from "./section-position";
import type { SectionConfig } from "./useStats";

function section(extra: Partial<SectionConfig>): SectionConfig {
  return { id: "clock", instance: "clock", enabled: true, monitor: 0, x: 0, y: 0, width: 360, ...extra } as SectionConfig;
}

// x and y are skip_serializing on the Rust side, so a profile loaded from disk
// always carries zeros. The real placement lives in `position`.
it("reads an anchored widget's offsets rather than the unserialized x and y", () => {
  const at = sectionPosition(section({
    position: { type: "Anchored", monitor_identity: "monitor-0", anchor: "TopLeft", offset_x: 24, offset_y: 264 },
  }));
  expect(at).toEqual({ x: 24, y: 264 });
});

it("reads an absolute widget's coordinates", () => {
  const at = sectionPosition(section({
    position: { type: "Absolute", monitor_identity: "monitor-0", x: 800, y: 450 },
  }));
  expect(at).toEqual({ x: 800, y: 450 });
});

it("treats a missing offset as zero", () => {
  const at = sectionPosition(section({
    position: { type: "Anchored", monitor_identity: "monitor-0", anchor: "TopLeft" },
  }));
  expect(at).toEqual({ x: 0, y: 0 });
});

it("falls back to the live x and y when no position is stored", () => {
  expect(sectionPosition(section({ x: 12, y: 34 }))).toEqual({ x: 12, y: 34 });
});

it("never reports zero for a widget the onboarding placed away from the corner", () => {
  const at = sectionPosition(section({
    x: 0, y: 0,
    position: { type: "Anchored", monitor_identity: "monitor-0", anchor: "TopLeft", offset_x: 24, offset_y: 144 },
  }));
  expect(at.y).toBe(144);
  expect(at).not.toEqual({ x: 0, y: 0 });
});
