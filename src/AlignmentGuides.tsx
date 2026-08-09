export interface Guide {
  orientation: "Vertical" | "Horizontal";
  position: number;
}

interface Props {
  guides: Guide[];
}

export function AlignmentGuides({ guides }: Props) {
  if (guides.length === 0) return null;
  return (
    <div className="alignmentGuides" aria-hidden="true">
      {guides.map((guide, i) => (
        <div
          key={i}
          className={
            guide.orientation === "Vertical" ? "alignmentGuideVertical" : "alignmentGuideHorizontal"
          }
          style={
            guide.orientation === "Vertical"
              ? { left: guide.position }
              : { top: guide.position }
          }
        />
      ))}
    </div>
  );
}
