import { useEffect, useState } from "react";

const greenTints = ["#00FF00", "#88FF00", "#00FF88", "#0DEE0D", "#18D418"];

const redTints = ["#FF0000", "#FF0088", "#FF8800", "#EE0D0D", "#D41818"];

const unstakedTint = "#aaa";

const animationConfig = {
  animationIterationCount: "infinite",
  animationTimingFunction: "linear",
  animationName: "blink",
};

const Server = ({ serverData: { tasks, tokens, staked } }) => {
  const [leds, setLeds] = useState([]);
  // const [tasksAssigned, setTasksAssigned] = useState(0);

  useEffect(() => {
    let tasksAssigned = 0;
    setLeds(
      [...Array(tokens).keys()].map((_, i) => {
        if (i >= staked) {
          return {
            animationDuration: Math.floor(Math.random() * 200) / 100 + 1 + "s",
            backgroundColor: unstakedTint,
          };
        }
        let needsWork = false;
        const stakedTokensLeft = staked - i;
        const tasksLeftToAssign = tasks - tasksAssigned;
        if (tasksLeftToAssign !== 0 && tasksLeftToAssign === stakedTokensLeft) {
          needsWork = true;
        } else if (tasksLeftToAssign > 0) {
          needsWork = Math.floor(Math.random() * 2) === 1;
        }
        if (needsWork) {
          tasksAssigned++;
        }
        const backgroundColor = needsWork
          ? redTints[i % redTints.length]
          : greenTints[i % greenTints.length];
        return {
          animationDuration: Math.floor(Math.random() * 200) / 100 + 1 + "s",
          backgroundColor,
        };
      })
    );
  }, [tasks, staked, tokens]);

  return (
    <div className="server">
      {leds.map((led, i) => (
        <div
          className="status-led"
          key={i}
          style={{ ...animationConfig, ...led }}
        ></div>
      ))}
    </div>
  );
};

export default Server;
