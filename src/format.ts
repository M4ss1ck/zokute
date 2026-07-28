const BYTE_UNITS_BINARY = ["B", "KiB", "MiB", "GiB", "TiB"] as const;
const BYTE_UNITS_DECIMAL = ["B", "KB", "MB", "GB", "TB"] as const;

export function formatBytes(bytes: number, mode: "binary" | "decimal" = "binary"): string {
  const units = mode === "decimal" ? BYTE_UNITS_DECIMAL : BYTE_UNITS_BINARY;
  const divisor = mode === "decimal" ? 1000 : 1024;
  let size = Math.max(0, bytes);
  let index = 0;
  while (size >= divisor && index < units.length - 1) {
    size /= divisor;
    index += 1;
  }
  return `${size.toFixed(size >= 10 || index === 0 ? 0 : 1)} ${units[index]}`;
}

export function formatRate(bytesPerSecond: number, mode: "binary" | "decimal" = "binary"): string {
  const units = mode === "decimal"
    ? ["B/s", "KB/s", "MB/s", "GB/s"]
    : ["B/s", "KiB/s", "MiB/s", "GiB/s"];
  const divisor = mode === "decimal" ? 1000 : 1024;
  let size = Math.max(0, bytesPerSecond);
  let index = 0;
  while (size >= divisor && index < units.length - 1) {
    size /= divisor;
    index += 1;
  }
  return `${size.toFixed(size >= 10 || index === 0 ? 0 : 1)} ${units[index]}`;
}

export function formatTemperature(celsius: number, unit: "celsius" | "fahrenheit" = "celsius"): string {
  if (unit === "fahrenheit") {
    return `${(celsius * 9 / 5 + 32).toFixed(1)}°F`;
  }
  return `${celsius.toFixed(1)}°C`;
}
