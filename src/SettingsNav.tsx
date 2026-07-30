import { useEffect, useState } from "react";
import { SETTINGS_GROUPS, SETTINGS_SECTIONS } from "./settings-sections";

export function SettingsNav() {
  const [active, setActive] = useState(SETTINGS_SECTIONS[0].id);

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
    for (const meta of SETTINGS_SECTIONS) {
      const anchor = document.getElementById(meta.id);
      if (anchor) observer.observe(anchor);
    }
    return () => observer.disconnect();
  }, []);

  return (
    <nav className="settingsNav" aria-label="Settings sections">
      {SETTINGS_GROUPS.map((group) => (
        <div className="settingsNavGroup" key={group.id}>
          <span className="settingsNavGroupLabel">{group.label}</span>
          {SETTINGS_SECTIONS.filter((meta) => meta.group === group.id).map((meta) => (
            <button
              type="button"
              className="settingsNavItem"
              key={meta.id}
              aria-current={active === meta.id ? "true" : undefined}
              onClick={() => document.getElementById(meta.id)?.scrollIntoView({ block: "start" })}
            >
              {meta.label}
            </button>
          ))}
        </div>
      ))}
    </nav>
  );
}
