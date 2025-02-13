import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./css/styles.css";
import "./css/tabs_wrapper.css";
import "./css/data.css";
import "./css/edit.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
