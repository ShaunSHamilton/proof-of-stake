import { useEffect, useState } from "react";
import Server from "./server";
import Ceiling from "./ceiling";
import Ground from "./ground";
import Monitor from "./monitor";

const sampleTask = {
  id: 1,
  name: "Sample task",
  description: "Sample description",
  tests: [
    {
      testText: "Sample test",
      test: "assert.equal(1, 1)",
    },
  ],
};

const MainView = () => {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    (async () => {
      // const data = await fetch("/api/tasks");
      const tasks = [sampleTask]; // await data.json();
      setTasks(tasks);
    })();
  }, []);
  const isServer1On = true;
  const isTaskAvailable = true;
  return (
    <main>
      <Ceiling />
      <Monitor tasks={tasks} />
      <div className="server-stack">
        <Server isServerOn={isServer1On} isTaskAvailable={isTaskAvailable} />
        <Server isServerOn={isServer1On} isTaskAvailable={false} />
        <Server isServerOn={isServer1On} isTaskAvailable={false} />
      </div>
      <Ground />
    </main>
  );
};

export default MainView;
