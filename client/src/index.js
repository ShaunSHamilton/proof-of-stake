import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import Navigation from "./components/navigation";
import MainView from "./components/main-view";

const App = () => {
  return (
    <>
      <Navigation />
      <MainView />
    </>
  );
};

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
