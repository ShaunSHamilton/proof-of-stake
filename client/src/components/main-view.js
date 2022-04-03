import { useContext, useEffect, useState } from "react";
import Server from "./server";
import Ceiling from "./ceiling";
import Ground from "./ground";
import Monitor from "./monitor";
import Camperbot from "./camperbot";
import { scramble } from "../tools/utils";
import { getSelf, NodeContext, sampleTask } from "../node-state";
import bubbleJson from "../../public/bubbles.json";

const MAX_TOKENS_PER_SERVER = 20;

const MainView = ({ setState, setIsTutorialing }) => {
  const nodeState = useContext(NodeContext);
  // State to do with Camper Node
  const [tasks, setTasks] = useState([]);
  const [nodeAccount, setNodeAccount] = useState(null);
  const [serverData, setServerData] = useState([]);
  const [isLightOn, setIsLightOn] = useState(true);
  const [text, setText] = useState("");
  const [lesson, setLesson] = useState(0);

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
    const self = getSelf(nodeState);
    console.log("self", self);
    setNodeAccount(self);
    setTasks(nodeState.tasks);
    console.log("nodeState: ", nodeState);
  }, [nodeState]);

  /*
  TODO: const event = new CustomEvent('tasks', { detail: { tasks } });
  socket.dispatchEvent(event);
  useEffect(() => {
    socket.addEventListener('tasks', ({detail: {tasks}}) =>{
      setTasks(tasks);
    });
  }, []);
  */

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
      console.log("nodeAccount: ", nodeAccount);
      const { tokens, staked, racks } = nodeAccount;
      let tokensRemaining = tokens;
      let stakedRemaining = staked;
      let tasksRemaining = tasks.length;
      // Create `racks` servers
      const servers = [];
      for (let i = 0; i < racks; i++) {
        const numTokensInServer =
          tokensRemaining >= MAX_TOKENS_PER_SERVER
            ? MAX_TOKENS_PER_SERVER
            : tokensRemaining;
        const numStakedInServer =
          stakedRemaining >= MAX_TOKENS_PER_SERVER
            ? MAX_TOKENS_PER_SERVER
            : stakedRemaining;
        const numTasksInServer =
          tasksRemaining >= MAX_TOKENS_PER_SERVER
            ? MAX_TOKENS_PER_SERVER
            : tasksRemaining;
        const server = {
          tasks: numTasksInServer,
          tokens: numTokensInServer,
          staked: numStakedInServer,
        };
        servers.push(server);
        tokensRemaining -= numTokensInServer;
        stakedRemaining -= numStakedInServer;
        tasksRemaining -= numTasksInServer;
      }

      setServerData(servers);
    }
  }, [nodeAccount, tasks]);

  useEffect(() => {
    console.log("Less: ", lesson);
    switch (lesson) {
      case 6:
        // Must have staked another token
        setState((prev) => {
          prev.chain[0].data[0].reputation += 1;
          prev.chain[0].data[0].tokens += 1;
          prev.chain[0].data[0].staked += 1;
          return { ...prev };
        });
        break;
      case 7:
        // Set task
        setState((prev) => ({ ...prev, tasks: [sampleTask] }));
        break;
      case 9:
        // Need to submit task
        break;
      case 10:
        setState((prev) => ({ ...prev, tasks: [] }));
        break;
      case 12:
        // Task has been validated
        // If correct, move on,
        // If incorrect, explain why
        setState((prev) => {
          prev.chain[0].data[0].reputation += 1;
          prev.chain[0].data[0].tokens += 1;

          return { ...prev };
        });
        break;
      case 16:
        // New rack bought
        setState((prev) => {
          prev.chain[0].data[0].reputation += 1;
          prev.chain[0].data[0].tokens += 1;
          prev.chain[0].data[0].racks += 1;
          return { ...prev };
        });
        break;
      case 18:
        // Hacked
        hacked();
        // Default starting state
        setIsTutorialing(false);
        break;
      case 19:
        break;
      default:
        break;
    }
    function hacked() {
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
