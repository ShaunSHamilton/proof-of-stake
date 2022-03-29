import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import Navigation from "./components/navigation";
import MainView from "./components/main-view";
import { clientWebSocket } from "./tools/handle-tasks";
import { NodeContext, state as initState } from "./node-state";

const App = () => {
  const [state, setState] = React.useState(initState);

  React.useEffect(() => {
    (async () => {
      const socket = await clientWebSocket(state);
      setState((prev) => ({ ...prev, socket }));
    })();
  }, []);

  return (
    <NodeContext.Provider value={state}>
      <Navigation />
      <MainView />
    </NodeContext.Provider>
  );
};

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
