import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import Navigation from "./components/navigation";
import MainView from "./components/main-view";
import { clientWebSocket } from "./tools/handle-tasks";
import { NodeContext, state as initState } from "./node-state";

// const UpdateNodeContext = createContext(null);
const App = () => {
  const [state, setState] = React.useState(initState);

  React.useEffect(() => {
    (async () => {
      const socket = await clientWebSocket(state, setState);
      setState((prev) => ({ ...prev, sock: socket }));
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <NodeContext.Provider value={state}>
      {/* <UpdateNodeContext.Provider value={setState}> */}
      <Navigation />
      <MainView />
      {/* </UpdateNodeContext.Provider> */}
    </NodeContext.Provider>
  );
};

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
