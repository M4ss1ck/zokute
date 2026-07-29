import type { SectionConfig } from "./useStats";

interface Props {
  sections: SectionConfig[];
  selected: string | null;
  onSelect: (instance: string) => void;
}

export function LayoutWidgetPicker({ sections, selected, onSelect }: Props) {
  if (sections.length === 0) return null;
  return (
    <div className="layoutWidgetPicker" role="group" aria-label="Widgets">
      {sections.map((section) => {
        const instance = section.instance ?? section.id;
        return (
          <button
            type="button"
            className="layoutWidgetChoice"
            data-selected={selected === instance || undefined}
            aria-pressed={selected === instance}
            aria-label={instance === section.id ? instance : `${instance} (${section.id})`}
            onClick={() => onSelect(instance)}
            key={instance}
          >
            <span>{instance}</span>
            {instance === section.id ? null : <small>{section.id}</small>}
          </button>
        );
      })}
    </div>
  );
}
