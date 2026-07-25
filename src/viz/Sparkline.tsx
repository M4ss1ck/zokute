interface Props {
  values: number[];
  min?: number;
  max?: number;
  width?: number;
  height?: number;
}

const DEFAULT_WIDTH = 64;
const DEFAULT_HEIGHT = 20;

// Redrawn only when `values` changes (a new stats tick) — no internal timer.
export function Sparkline({ values, min, max, width = DEFAULT_WIDTH, height = DEFAULT_HEIGHT }: Props) {
  if (values.length < 2) {
    return <svg width={width} height={height} aria-hidden="true" />;
  }
  const resolvedMax = max ?? Math.max(...values, 1);
  const resolvedMin = min ?? Math.min(...values, 0);
  const range = resolvedMax - resolvedMin || 1;
  const stepX = width / (values.length - 1);
  const points = values
    .map((value, index) => {
      const x = index * stepX;
      const y = height - ((value - resolvedMin) / range) * height;
      return `${x.toFixed(1)},${y.toFixed(1)}`;
    })
    .join(" ");
  return (
    <svg width={width} height={height} viewBox={`0 0 ${width} ${height}`} aria-hidden="true">
      <polyline
        points={points}
        fill="none"
        strokeWidth="1.5"
        strokeLinejoin="round"
        strokeLinecap="round"
        style={{ stroke: "var(--viz-stroke-color)" }}
      />
    </svg>
  );
}
