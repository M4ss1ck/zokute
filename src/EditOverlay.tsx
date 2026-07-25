import { getCurrentWindow } from "@tauri-apps/api/window";

interface Props {
  label: string;
}

const resizeDirections = [
  "North",
  "NorthEast",
  "East",
  "SouthEast",
  "South",
  "SouthWest",
  "West",
  "NorthWest",
] as const;

// Only mounted while the backend reports edit mode, so nothing here can
// interfere with the click-through HUD in normal operation.
export function EditOverlay({ label }: Props) {
  return (
    <div className="editOverlay" data-testid="edit-overlay">
      <button
        type="button"
        className="editDrag"
        aria-label={`Move ${label} widget`}
        onMouseDown={() => {
          void getCurrentWindow().startDragging();
        }}
      >
        {label}
      </button>
      {resizeDirections.map((direction) => (
        <button
          type="button"
          className="editResize"
          data-direction={direction}
          aria-label={`Resize ${label} widget ${direction.toLowerCase()}`}
          key={direction}
          onMouseDown={(event) => {
            event.stopPropagation();
            void getCurrentWindow().startResizeDragging(direction);
          }}
        />
      ))}
    </div>
  );
}
