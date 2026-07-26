import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { IconX } from "@tabler/icons-react";
import { useState } from "react";

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
  bare?: boolean;
  onResizeStart?: (direction: ResizeDirection) => void;
}

// Only mounted while the backend reports edit mode, so nothing here can
// interfere with the click-through HUD in normal operation.
export function EditOverlay({ label, bare, onResizeStart }: Props) {
  const [removeState, setRemoveState] = useState<"default" | "loading" | "error" | "success">("default");
  async function remove() {
    setRemoveState("loading");
    try {
      await invoke("remove_widget", { instance: label });
      setRemoveState("success");
    } catch {
      setRemoveState("error");
    }
  }
  return (
    <div className={bare ? "editOverlay editOverlay--bare" : "editOverlay"} data-testid="edit-overlay">
      <button
        type="button"
        className="editClose"
        aria-label={`Remove ${label} widget`}
        data-state={removeState}
        disabled={removeState === "loading" || removeState === "success"}
        title={removeState === "error" ? "Could not remove widget" : undefined}
        onClick={() => void remove()}
      >
        <IconX aria-hidden="true" />
      </button>
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
