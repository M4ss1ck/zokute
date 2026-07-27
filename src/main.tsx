import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "@fontsource/ibm-plex-sans/400.css";
import "@fontsource/ibm-plex-sans/700.css";
import "@fontsource/jetbrains-mono/400.css";
import "@fontsource/jetbrains-mono/700.css";
import "./App.css";
import "./panel-text.css";
import "./widget-layout.css";
import "./clock.css";
import "./date.css";
import "./viz-canvas.css";
import "./edit-overlay.css";
import "./settings.css";
import "./settings-controls.css";
import "./settings-toggle-group.css";
import "./settings-widgets.css";
import "./settings-color.css";
import "./settings-fields.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
