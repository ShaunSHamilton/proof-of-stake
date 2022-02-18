import { useEffect, useState } from "react";
import Server from "./server";
import Ceiling from "./ceiling";
import Ground from "./ground";
import Monitor from "./monitor";
import Camperbot from "./camperbot";
import { scramble } from "../tools/utils";
import { getNodeAccount, getTasks } from "../tools/handle-tasks";

const MainView = () => {
  // State to do with bots
  // const [otherTasks, setOtherTasks] = useState([]);
  // const [otherAccounts, setOtherAccounts] = useState([]);

  // State to do with Camper Node
  const [tasks, setTasks] = useState([]);
  const [nodeAccount, setNodeAccount] = useState(null);
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
      const aTasks = getTasks();
      setTasks(aTasks.filter((t) => t.nodeOwner === "Camper"));

      // get account data
      // const accountData = await fetch("/api/account");
      const nAccount = getNodeAccount();
      setNodeAccount(nAccount);

      setBubbleJson(await (await fetch("/bubbles.json")).json());
    })();
  }, []);

  useEffect(() => {
    setText(bubbleJson[lesson]?.text ?? "");
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [bubbleJson]);

  function toggleLight() {
    document.querySelector(".room").classList.toggle("dark");
    setIsLightOn(!isLightOn);
  }

  useEffect(() => {
    if (nodeAccount) {
      const { tokens, staked, reputation } = nodeAccount;

      const sData = [...Array(reputation).keys()].map(() => {
        return {
          tasks: 0,
          tokens: Math.floor(tokens / reputation),
          staked: Math.floor(staked / reputation),
        };
      });
      setServerData(sData);
    }
  }, [nodeAccount]);

  useEffect(() => {
    if (lesson === 18) {
      let i = 0;
      const interval = setInterval(() => {
        if (i >= 10) {
          if (!isLightOn) {
            toggleLight();
          }
          return clearInterval(interval);
        }
        toggleLight();
        i++;
      }, 200);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [lesson]);

  return (
    <main className={lesson === 18 ? "show-take-over" : ""}>
      <Camperbot
        text={text}
        setText={setText}
        isLightOn={isLightOn}
        handleNextBubble={handleNextBubble}
        handlePreviousBubble={handlePreviousBubble}
      />
      <Ceiling isLightOn={isLightOn} toggleLight={toggleLight} />
      <section className="room">
        <div className="station">
          <Monitor task={scramble(tasks)?.[0]} isLightOn={isLightOn} />
          <div className="server-stack">
            {serverData.map((server, i) => (
              <Server serverData={server} key={i} isLightOn={isLightOn} />
            ))}
          </div>
        </div>
      </section>
      <Ground />
    </main>
  );
};

export default MainView;
