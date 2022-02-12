import { useEffect, useState } from "react";
import Server from "./server";
import Ceiling from "./ceiling";
import Ground from "./ground";
import Monitor from "./monitor";
import Camperbot from "./camperbot";

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

const sampleServerData_1 = {
  tasks: 2,
  tokens: 20,
  staked: 13,
};
const sampleServerData_2 = {
  tasks: 1,
  tokens: 20,
  staked: 20,
};

const MAX_TOKENS_PER_SERVER = 20;

const MainView = () => {
  const [tasks, setTasks] = useState([]);
  const [tokens, setTokens] = useState(20);
  const [reputation, setReputation] = useState(3);
  const [serverData, setServerData] = useState([]);

  useEffect(() => {
    (async () => {
      // const data = await fetch("/api/tasks");
      const tasks = [sampleTask]; // await data.json();
      setTasks(tasks);
      setServerData([sampleServerData_1, sampleServerData_2]);
    })();
  }, []);

  const handleServerDishout = (i) => {
    const data = serverData[i];
    return data ?? { tasks: [], tokens: 0, staked: 0 };
  };
  return (
    <main>
      <Camperbot />
      <Ceiling />
      <section className="room">
        <div className="station">
          <Monitor tasks={tasks} />
          <div className="server-stack">
            {[...Array(reputation).keys()].map((_, i) => (
              <Server serverData={handleServerDishout(i)} key={i} />
            ))}
          </div>
        </div>
      </section>
      <Ground />
    </main>
  );
};

export default MainView;
