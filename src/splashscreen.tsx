import React from "react";
import ReactDOM from "react-dom/client";
import "../index.css";
import TitleBar from "@/components/titlebar";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Splashscreen />
  </React.StrictMode>
);

function Splashscreen() {
  return (
    <div>
      <TitleBar />
      <div className="pt-[1.7rem] overflow-hidden h-screen">
        <h1>My App ISOTOPEEEEEE</h1>
        <p>Loading...</p>
      </div>
    </div>
  );
}
