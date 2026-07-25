interface Props {
  values: number[];
  min?: number;
  max?: number;
  width?: number;
  height?: number;
}

const DEFAULT_WIDTH = 64;
const DEFAULT_HEIGHT = 20;

function resolveBounds(values: number[], min?: number, max?: number) {
  if (min !== undefined && max !== undefined) {
    if (min === max) {
      return { min: min - 0.5, max: max + 0.5 };
    }
    return { min, max };
  }
  const finiteValues = values.filter(Number.isFinite);
  if (finiteValues.length < 2) {
    return null;
  }
  const resolvedMin = min ?? Math.min(...finiteValues);
  const resolvedMax = max ?? Math.max(...finiteValues);
  if (resolvedMin === resolvedMax) {
    return { min: resolvedMin - 0.5, max: resolvedMax + 0.5 };
  }
  return { min: resolvedMin, max: resolvedMax };
}

// Redrawn only when `values` changes (a new stats tick) — no internal timer.
export function Sparkline({ values, min, max, width = DEFAULT_WIDTH, height = DEFAULT_HEIGHT }: Props) {
  const bounds = resolveBounds(values, min, max);
  if (bounds === null) {
    return <svg width="100%" height={height} aria-hidden="true" />;
  }
  const range = bounds.max - bounds.min;
  const stepX = width / (values.length - 1);
  const points = values
    .map((value, index) => {
      const x = index * stepX;
      const y = height - ((value - bounds.min) / range) * height;
      return `${x.toFixed(1)},${y.toFixed(1)}`;
    })
    .join(" ");
  return (
    <svg width="100%" height={height} viewBox={`0 0 ${width} ${height}`} preserveAspectRatio="none" aria-hidden="true">
      <polyline
        points={points}
        fill="none"
        strokeWidth="1.5"
        strokeLinejoin="round"
        strokeLinecap="round"
        vectorEffect="non-scaling-stroke"
        style={{ stroke: "var(--viz-stroke-color)" }}
      />
    </svg>
  );
}
