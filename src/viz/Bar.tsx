interface Props {
  percent: number;
  height?: number;
}

const DEFAULT_HEIGHT = 5;

// Redrawn only when `percent` changes (a new stats tick) — no internal timer.
// Square corners, not rounded: preserveAspectRatio="none" scales x and y by
// different factors on a wide, short bar, which turns a rounded end into a
// visible oval.
export function Bar({ percent, height = DEFAULT_HEIGHT }: Props) {
  const clamped = Number.isFinite(percent) ? Math.max(0, Math.min(100, percent)) : 0;
  return (
    <svg width="100%" height={height} viewBox={`0 0 100 ${height}`} preserveAspectRatio="none" aria-hidden="true">
      <rect width={100} height={height} style={{ fill: "var(--bar-track-color)" }} />
      <rect
        height={height}
        style={{
          width: `${clamped}%`,
          fill: "var(--viz-stroke-color)",
          transition: "width var(--bar-transition-duration) ease-out",
        }}
      />
    </svg>
  );
}
