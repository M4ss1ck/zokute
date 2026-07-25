import { getCurrentWindow } from "@tauri-apps/api/window";

interface Props {
  label: string;
}

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
      <button
        type="button"
        className="editResize"
        aria-label={`Resize ${label} widget width`}
        onMouseDown={(event) => {
          event.stopPropagation();
          void getCurrentWindow().startResizeDragging("East");
        }}
      />
    </div>
  );
}
