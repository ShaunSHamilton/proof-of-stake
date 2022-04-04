import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import Navigation from "./components/navigation";
import MainView from "./components/main-view";
import { clientWebSocket } from "./tools/handle-tasks";
import { NodeContext, state as initState, tutorialState } from "./node-state";

const App = () => {
  const [isTutorialing, setIsTutorialing] = React.useState(true);
  const [state, setState] = React.useState(
    isTutorialing ? tutorialState : initState
  );

  React.useEffect(() => {
    if (!isTutorialing) {
      (async () => {
        const socket = await clientWebSocket(state, setState);
        setState((prev) => ({ ...prev, sock: socket }));
      })();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isTutorialing]);

  React.useEffect(() => {
    setState(isTutorialing ? tutorialState : initState);
  }, [isTutorialing]);

  React.useEffect(() => {
    if (isTutorialing) {
      setState((prev) => ({ ...prev, setTutorialState: setState }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <NodeContext.Provider value={state}>
      <Navigation />
      <MainView
        setIsTutorialing={setIsTutorialing}
        isTutorialing={isTutorialing}
      />
    </NodeContext.Provider>
  );
};

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
