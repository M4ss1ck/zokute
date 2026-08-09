import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { AlignmentGuides, type Guide } from "./AlignmentGuides";

const LABEL_PREFIX = "guides-";

export function isGuidesLabel(label: string): boolean {
  return label.startsWith(LABEL_PREFIX);
}

export function GuidesOverlay() {
  const [guides, setGuides] = useState<Guide[]>([]);
  useEffect(() => {
    const pending = listen<Guide[]>("alignment-guides", (event) =>
      setGuides(event.payload),
    );
    return () => {
      void pending.then((unlisten) => unlisten());
    };
  }, []);
  return <AlignmentGuides guides={guides} />;
}
