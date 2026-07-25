import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "@fontsource/ibm-plex-sans/400.css";
import "@fontsource/ibm-plex-sans/700.css";
import "@fontsource/jetbrains-mono/400.css";
import "@fontsource/jetbrains-mono/700.css";
import "./App.css";
import "./widget-layout.css";
import "./settings.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
