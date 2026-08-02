import { Fragment, useEffect, useState } from "react";
import type { SectionConfig } from "./useStats";
import { SETTINGS_SECTIONS, settingsNavTree } from "./settings-sections";

interface Props {
  sections: SectionConfig[];
}

export function SettingsNav({ sections }: Props) {
  const [active, setActive] = useState(SETTINGS_SECTIONS[0].id);
  const tree = settingsNavTree(sections);
  const anchorIds = tree.flatMap((group) => {
    const staticItems = group.items.filter((item) => item.children.length === 0).map((item) => item.id);
    const dynamicItems = group.items.filter((item) => item.children.length > 0).flatMap((item) => item.children.map((child) => child.anchorId));
    return [...staticItems, ...dynamicItems];
  });
  // A fresh array every render would tear the observer down on each keystroke,
  // so the effect keys off the anchor ids themselves.
  const anchorKey = JSON.stringify(anchorIds);

  useEffect(() => {
    const pane = document.getElementById("settings-pane");
    if (!pane) return;
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries.find((entry) => entry.isIntersecting);
        if (visible) setActive(visible.target.id);
      },
      { root: pane, rootMargin: "0px 0px -70% 0px" },
    );
    for (const id of anchorIds) {
      const anchor = document.getElementById(id);
      if (anchor) observer.observe(anchor);
    }
    return () => observer.disconnect();
  }, [anchorKey]);

  return (
    <nav className="settingsNav" aria-label="Settings sections">
      {tree.map((group) => (
        <div className="settingsNavGroup" key={group.id}>
          <span className="settingsNavGroupLabel">{group.label}</span>
          {group.items.map((item) => (
            <Fragment key={item.id}>
              <button
                type="button"
                className="settingsNavItem"
                aria-label={item.children.some((child) => child.label === item.label) ? `${item.label} settings` : undefined}
                aria-current={active === item.id || item.children.some((child) => child.anchorId === active) ? "true" : undefined}
                onClick={() => document.getElementById(item.id)?.scrollIntoView({ block: "start" })}
              >
                {item.label}
              </button>
              {item.children.map((child) => (
                <button
                  type="button"
                  className="settingsNavItem settingsNavChild"
                  key={child.anchorId}
                  aria-label={child.label === item.label ? `${child.label} instance` : undefined}
                  aria-current={active === child.anchorId ? "true" : undefined}
                  onClick={() => document.getElementById(child.anchorId)?.scrollIntoView({ block: "start" })}
                >
                  {child.label}
                </button>
              ))}
            </Fragment>
          ))}
        </div>
      ))}
    </nav>
  );
}
