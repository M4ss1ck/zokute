import { useEffect, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Stats, SectionConfig } from "./useStats";
import { sectionPosition } from "./section-position";
import { LayoutWidgetPicker } from "./LayoutWidgetPicker";

interface Props {
  stats: Stats | null;
}

function nudge(section: SectionConfig, dx: number, dy: number) {
  const at = sectionPosition(section);
  return invoke("edit_move_widget", {
    instance: section.instance ?? section.id,
    x: at.x + dx,
    y: at.y + dy,
  });
}

function Field({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="layoutEditorField">
      <label>{label}</label>
      <span>{value}</span>
    </div>
  );
}

function remove(section: SectionConfig) {
  return invoke("remove_widget", { instance: section.instance ?? section.id });
}

export function LayoutEditor({ stats }: Props) {
  const [selected, setSelected] = useState<string | null>(null);
  const [canUndo, setCanUndo] = useState(false);
  const [canRedo, setCanRedo] = useState(false);
  const [eventCount, setEventCount] = useState(0);

  const sections = stats?.profile.sections ?? [];

  const selection = selected ? sections.find((s) => (s.instance ?? s.id) === selected) ?? null : null;

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
    if (selection) {
      const step = e.shiftKey ? 10 : 1;
      if (e.key === "ArrowUp") { e.preventDefault(); void nudge(selection, 0, -step); return; }
      if (e.key === "ArrowDown") { e.preventDefault(); void nudge(selection, 0, step); return; }
      if (e.key === "ArrowLeft") { e.preventDefault(); void nudge(selection, -step, 0); return; }
      if (e.key === "ArrowRight") { e.preventDefault(); void nudge(selection, step, 0); return; }
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
      <LayoutWidgetPicker sections={sections} selected={selected} onSelect={setSelected} />
      {selection ? (
        <div className="layoutEditorSelection">
          <Field label="Widget" value={selection.instance ?? selection.id} />
          <Field label="X" value={sectionPosition(selection).x} />
          <Field label="Y" value={sectionPosition(selection).y} />
          <Field label="Width" value={selection.width} />
          <Field label="Height" value={selection.height ?? "auto"} />
          <button type="button" className="layoutEditorButton layoutEditorDelete" onClick={() => remove(selection)}>Delete</button>
        </div>
      ) : (
        <p className="layoutEditorEmpty">Select a widget to edit its position.</p>
      )}
    </main>
  );
}
