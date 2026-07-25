interface Props {
  percent: number;
  size?: number;
  strokeWidth?: number;
}

const DEFAULT_SIZE = 40;
const DEFAULT_STROKE = 4;

// Redrawn only when `percent` changes (a new stats tick) — no internal timer.
export function Arc({ percent, size = DEFAULT_SIZE, strokeWidth = DEFAULT_STROKE }: Props) {
  const clamped = Number.isFinite(percent) ? Math.max(0, Math.min(100, percent)) : 0;
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference * (1 - clamped / 100);
  const center = size / 2;
  return (
    <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`} aria-hidden="true">
      <circle cx={center} cy={center} r={radius} fill="none" strokeWidth={strokeWidth} style={{ stroke: "var(--bar-track-color)" }} />
      <circle
        cx={center}
        cy={center}
        r={radius}
        fill="none"
        strokeWidth={strokeWidth}
        strokeDasharray={circumference}
        strokeLinecap="round"
        transform={`rotate(-90 ${center} ${center})`}
        style={{
          stroke: "var(--viz-stroke-color)",
          strokeDashoffset: offset,
          transition: "stroke-dashoffset var(--bar-transition-duration) ease-out",
        }}
      />
    </svg>
  );
}
