interface Props {
  value: number;
  onPreview: (value: number) => void;
  onCommit: (value: number) => void;
}

// `input` fires continuously while dragging and only previews; `change` fires
// on release and is the only event that persists, so a drag writes the file
// once instead of once per pixel.
export function OpacityControl({ value, onPreview, onCommit }: Props) {
  return (
    <div className="settingsRow">
      <label className="settingsRowLabel" htmlFor="settings-opacity">
        Opacity
      </label>
      <input
        id="settings-opacity"
        type="range"
        min="0.1"
        max="1"
        step="0.01"
        value={value}
        onInput={(event) => onPreview(Number(event.currentTarget.value))}
        onChange={(event) => {
          if (event.nativeEvent.type === "change") {
            onCommit(Number(event.currentTarget.value));
          }
        }}
      />
      <span className="settingsValue">{Math.round(value * 100)}%</span>
    </div>
  );
}
