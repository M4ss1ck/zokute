import { useEffect, useRef } from "react";

interface Props {
  id: string;
  label: string;
  value: number;
  onPreview: (value: number) => void;
  onCommit: (value: number) => void;
}

// `input` fires continuously while dragging and only previews; `change` fires
// on release and is the only event that persists, so a drag writes the file
// once instead of once per pixel.
export function OpacityControl({ id, label, value, onPreview, onCommit }: Props) {
  const inputRef = useRef<HTMLInputElement>(null);
  useEffect(() => {
    const input = inputRef.current;
    if (!input) return;
    // Native change listener bypasses React's change-event dedupe, ensuring
    // commit fires exactly once on release despite the value prop changing per-tick.
    const handleChange = () => {
      onCommit(Number(input.value));
    };
    input.addEventListener("change", handleChange);
    return () => input.removeEventListener("change", handleChange);
  }, [onCommit]);
  return (
    <div className="settingsRow">
      <label className="settingsRowLabel" htmlFor={id}>
        {label}
      </label>
      <input
        ref={inputRef}
        id={id}
        type="range"
        min="0.1"
        max="1"
        step="0.01"
        value={value}
        onInput={(event) => onPreview(Number(event.currentTarget.value))}
      />
      <span className="settingsValue">{Math.round(value * 100)}%</span>
    </div>
  );
}
