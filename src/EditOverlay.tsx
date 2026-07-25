import { getCurrentWindow } from "@tauri-apps/api/window";

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

export type ResizeDirection = typeof resizeDirections[number];

interface Props {
  label: string;
  onResizeStart?: (direction: ResizeDirection) => void;
}

// Only mounted while the backend reports edit mode, so nothing here can
// interfere with the click-through HUD in normal operation.
export function EditOverlay({ label, onResizeStart }: Props) {
  return (
    <div className="editOverlay" data-testid="edit-overlay">
      <button
        type="button"
        className="editDrag"
        aria-label={`Move ${label} widget`}
        onMouseDown={() => {
          void getCurrentWindow().startDragging();
        }}
      />
      {resizeDirections.map((direction) => (
        <button
          type="button"
          className="editResize"
          data-direction={direction}
          aria-label={`Resize ${label} widget ${direction.toLowerCase()}`}
          key={direction}
          onMouseDown={(event) => {
            event.stopPropagation();
            onResizeStart?.(direction);
            void getCurrentWindow().startResizeDragging(direction);
          }}
        />
      ))}
    </div>
  );
}
