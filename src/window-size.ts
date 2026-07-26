function borderBoxHeight(entry: ResizeObserverEntry, element: HTMLElement) {
  const size = Array.isArray(entry.borderBoxSize) ? entry.borderBoxSize[0] : entry.borderBoxSize;
  return size ? size.blockSize : element.getBoundingClientRect().height;
}

interface Box {
  // The height the user has asked for: the live window while editing, the
  // persisted height otherwise, and zero mid-zoom so the widget stays
  // content-tight and grows rather than gaining empty space.
  height: number;
  width: number;
  scale: number;
}

// Height is a floor, not a target. The box is the user's to size, but it can
// never be dragged shorter than the content it has to show.
export function targetWindowSize(entry: ResizeObserverEntry, element: HTMLElement, paddingY: number, box: Box) {
  const content = Math.ceil((borderBoxHeight(entry, element) + paddingY) * box.scale);
  return { width: box.width, height: Math.max(box.height, content) };
}
