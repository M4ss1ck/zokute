import { useCallback, useEffect, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Stats, SectionConfig } from "./useStats";

interface Props {
  stats: Stats | null;
}

export function LayoutEditor({ stats }: Props) {
  const [selected, setSelected] = useState<string | null>(null);
  const [canUndo, setCanUndo] = useState(false);
  const [canRedo, setCanRedo] = useState(false);
  const [eventCount, setEventCount] = useState(0);

  const sections = stats?.config.sections ?? [];

  const selection = selected ? sections.find((s) => (s.instance ?? s.id) === selected) ?? null : null;

  const getNext = useCallback((current: string | null, reverse: boolean) => {
    if (sections.length === 0) return null;
    const idx = current ? sections.findIndex((s) => (s.instance ?? s.id) === current) : -1;
    const next = reverse
      ? (idx - 1 + sections.length) % sections.length
      : (idx + 1) % sections.length;
    return sections[next]?.instance ?? sections[next]?.id ?? null;
  }, [sections]);

  useEffect(() => {
    if (selection) return;
    if (sections.length > 0) {
      setSelected(sections[0].instance ?? sections[0].id);
    }
  }, [sections, selection]);

  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | null = null;
    void listen("stats", () => {
      if (!active) return;
      setEventCount((c) => c + 1);
    }).then((cleanup) => { unlisten = cleanup; });
    return () => { active = false; if (unlisten) unlisten(); };
  }, []);

  useEffect(() => {
    if (eventCount === 0) return;
    void invoke<boolean>("can_undo_edit").then(setCanUndo).catch(() => {});
    void invoke<boolean>("can_redo_edit").then(setCanRedo).catch(() => {});
  }, [eventCount]);

  function onKeyDown(e: ReactKeyboardEvent) {
    if (e.key === "Tab") {
      e.preventDefault();
      setSelected(getNext(selected, e.shiftKey));
      return;
    }
    if (e.key === "Escape") {
      if (selected) { setSelected(null); return; }
      return;
    }
    if ((e.key === "Delete" || e.key === "Backspace") && selected) {
      void invoke("remove_widget", { instance: selected });
      setSelected(null);
      return;
    }
    if (e.ctrlKey || e.metaKey) {
      if (e.key === "z" && !e.shiftKey) {
        e.preventDefault();
        void invoke("undo_edit");
        return;
      }
      if ((e.key === "z" && e.shiftKey) || e.key === "y") {
        e.preventDefault();
        void invoke("redo_edit");
        return;
      }
    }
    if (selected) {
      const step = e.shiftKey ? 10 : 1;
      if (e.key === "ArrowUp") { e.preventDefault(); void invoke("edit_move_widget", { instance: selected, x: (selection?.x ?? 0), y: (selection?.y ?? 0) - step }); return; }
      if (e.key === "ArrowDown") { e.preventDefault(); void invoke("edit_move_widget", { instance: selected, x: (selection?.x ?? 0), y: (selection?.y ?? 0) + step }); return; }
      if (e.key === "ArrowLeft") { e.preventDefault(); void invoke("edit_move_widget", { instance: selected, x: (selection?.x ?? 0) - step, y: (selection?.y ?? 0) }); return; }
      if (e.key === "ArrowRight") { e.preventDefault(); void invoke("edit_move_widget", { instance: selected, x: (selection?.x ?? 0) + step, y: (selection?.y ?? 0) }); return; }
    }
  }

  return (
    <main className="layoutEditor" aria-label="Layout editor" tabIndex={-1} onKeyDown={onKeyDown}>
      <header className="layoutEditorHeader">
        <h1>Edit Layout</h1>
        <p>Arrange and resize your widgets.</p>
      </header>
      <div className="layoutEditorToolbar">
        <button type="button" className="layoutEditorButton layoutEditorSave" onClick={() => void invoke("save_layout").catch(() => {})}>
          Save
        </button>
        <button type="button" className="layoutEditorButton" onClick={() => void invoke("cancel_layout").catch(() => {})}>
          Cancel
        </button>
        <button type="button" className="layoutEditorButton" disabled={!canUndo} onClick={() => void invoke("undo_edit").catch(() => {})}>
          Undo
        </button>
        <button type="button" className="layoutEditorButton" disabled={!canRedo} onClick={() => void invoke("redo_edit").catch(() => {})}>
          Redo
        </button>
      </div>
      {selection ? (
        <div className="layoutEditorSelection">
          <div className="layoutEditorField">
            <label>Widget</label>
            <span>{selection.instance ?? selection.id}</span>
          </div>
          <div className="layoutEditorField">
            <label>X</label>
            <span>{selection.x ?? 0}</span>
          </div>
          <div className="layoutEditorField">
            <label>Y</label>
            <span>{selection.y ?? 0}</span>
          </div>
          <div className="layoutEditorField">
            <label>Width</label>
            <span>{selection.width}</span>
          </div>
          <div className="layoutEditorField">
            <label>Height</label>
            <span>{selection.height ?? "auto"}</span>
          </div>
          <button type="button" className="layoutEditorButton layoutEditorDelete" onClick={() => {
            void invoke("remove_widget", { instance: (selection.instance ?? selection.id) });
            setSelected(null);
          }}>
            Delete
          </button>
        </div>
      ) : (
        <p className="layoutEditorEmpty">Select a widget to edit its position.</p>
      )}
    </main>
  );
}
