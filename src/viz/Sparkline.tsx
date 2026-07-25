interface Props {
  values: number[];
  min?: number;
  max?: number;
  width?: number;
  height?: number;
}

const DEFAULT_WIDTH = 64;
const DEFAULT_HEIGHT = 20;

function resolveData(values: number[], min?: number, max?: number) {
  const finiteValues = values.filter(Number.isFinite);
  if (finiteValues.length < 2) {
    return null;
  }
  if (min !== undefined && max !== undefined && Number.isFinite(min) && Number.isFinite(max)) {
    return min === max
      ? { values: finiteValues, min: min - 0.5, max: max + 0.5 }
      : { values: finiteValues, min: Math.min(min, max), max: Math.max(min, max) };
  }
  const resolvedMin = Math.min(...finiteValues);
  const resolvedMax = Math.max(...finiteValues);
  return resolvedMin === resolvedMax
    ? { values: finiteValues, min: resolvedMin - 0.5, max: resolvedMax + 0.5 }
    : { values: finiteValues, min: resolvedMin, max: resolvedMax };
}

// Redrawn only when `values` changes (a new stats tick) — no internal timer.
export function Sparkline({ values, min, max, width = DEFAULT_WIDTH, height = DEFAULT_HEIGHT }: Props) {
  const data = resolveData(values, min, max);
  if (data === null) {
    return <svg width="100%" height={height} aria-hidden="true" />;
  }
  const range = data.max - data.min;
  const stepX = width / (data.values.length - 1);
  const points = data.values
    .map((value, index) => {
      const x = index * stepX;
      const y = height - ((value - data.min) / range) * height;
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
