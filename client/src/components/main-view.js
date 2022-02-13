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

const MAX_TOKENS_PER_SERVER = 24;

const MainView = () => {
  const [tasks, setTasks] = useState([]);
  const [tokens, setTokens] = useState(20);
  const [reputation, setReputation] = useState(3);
  const [serverData, setServerData] = useState([]);
  const [isLightOn, setIsLightOn] = useState(true);
  const [text, setText] = useState("");
  const [lesson, setLesson] = useState(0);
  const [bubbleJson, setBubbleJson] = useState([]);

  const handleNextBubble = () => {
    if (lesson < bubbleJson.length - 1) {
      setText(bubbleJson[lesson + 1]?.text ?? "");
      setLesson(lesson + 1);
    }
  };

  const handlePreviousBubble = () => {
    if (lesson > 0) {
      setText(bubbleJson[lesson - 1]?.text ?? "");
      setLesson(lesson - 1);
    }
  };

  useEffect(() => {
    (async () => {
      // const data = await fetch("/api/tasks");
      const tasks = [sampleTask]; // await data.json();
      setTasks(tasks);
      setServerData([sampleServerData_1, sampleServerData_2]);

      setBubbleJson(await (await fetch("/bubbles.json")).json());
    })();
  }, []);

  useEffect(() => {
    setText(bubbleJson[lesson]?.text ?? "");
  }, [bubbleJson]);

  function toggleLight() {
    document.querySelector(".room").classList.toggle("dark");
    setIsLightOn(!isLightOn);
  }

  const handleServerDishout = (i) => {
    const data = serverData[i];
    return data ?? { tasks: [], tokens: 0, staked: 0 };
  };
  return (
    <main>
      <Camperbot
        text={text}
        isLightOn={isLightOn}
        handleNextBubble={handleNextBubble}
        handlePreviousBubble={handlePreviousBubble}
      />
      <Ceiling isLightOn={isLightOn} toggleLight={toggleLight} />
      <section className="room">
        <div className="station">
          <Monitor tasks={tasks} isLightOn={isLightOn} />
          <div className="server-stack">
            {[...Array(reputation).keys()].map((_, i) => (
              <Server
                serverData={handleServerDishout(i)}
                key={i}
                isLightOn={isLightOn}
              />
            ))}
          </div>
        </div>
      </section>
      <Ground />
    </main>
  );
};

export default MainView;
