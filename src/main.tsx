import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./css/styles.css";
import "./css/app_display.css";
import "./css/data.css";
import "./css/edit.css";
import "./css/setting.css"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
