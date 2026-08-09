import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import App from "./App";
import { GuidesOverlay, isGuidesLabel } from "./GuidesOverlay";
import "@fontsource/ibm-plex-sans/400.css";
import "@fontsource/ibm-plex-sans/700.css";
import "@fontsource/jetbrains-mono/400.css";
import "@fontsource/jetbrains-mono/700.css";
import "./theme-base.css";
import "./theme-light.css";
import "./theme-dark.css";
import "./theme-system.css";
import "./theme-motion.css";
import "./App.css";
import "./panel-text.css";
import "./widget-layout.css";
import "./clock.css";
import "./date.css";
import "./viz-canvas.css";
import "./edit-overlay.css";
import "./panel.css";
import "./alignment-guides.css";
import "./settings.css";
import "./settings-shell.css";
import "./settings-controls.css";
import "./settings-toggle-group.css";
import "./settings-widgets.css";
import "./settings-color.css";
import "./settings-fields.css";

// An overlay window renders guides only. Routing here rather than inside App
// keeps useStats, and its per-tick re-render, out of these windows entirely.
const root = isGuidesLabel(getCurrentWindow().label) ? (
  <GuidesOverlay />
) : (
  <App />
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>{root}</React.StrictMode>,
);
