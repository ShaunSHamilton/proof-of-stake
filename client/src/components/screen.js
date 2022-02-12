import { useEffect, useState } from "react";
import Description from "./description";
import Editor from "./editor";
import ScreenNav from "./screen-nav";

import glow from "../tools/glow";

const Screen = ({ tasks = [], isLightOn }) => {
  const [isTask, setIsTask] = useState(tasks.length > 0);
  const [isShowActualScreen, setIsShowActualScreen] = useState(false);

  useEffect(() => {
    setIsTask(tasks.length > 0);
  }, [tasks]);

  async function startTask() {
    // Stop flashing
    setIsTask(false);
    await animateIntoScreen();
    popActualScreen();
    // and text editor.
    // On submit, tests are run, and results are displayed.
  }

  async function animateIntoScreen() {
    const width = window.innerWidth / 130;
    const bodyStyle = document.querySelector("body").style;

    const screenBounds = document
      .querySelector(".screen")
      .getBoundingClientRect();
    const screenCenter = screenBounds.y + screenBounds.height / 2;
    bodyStyle.transition = "transform 2s";
    bodyStyle.transformOrigin = `50% ${screenCenter}px`;
    bodyStyle.transform = `scale(${width})`;

    await new Promise((resolve) => setTimeout(resolve, 1800));
  }

  async function handleSub() {
    const width = window.innerWidth / 130;
    const bodyStyle = document.querySelector("body").style;
    const screenBounds = document
      .querySelector(".screen")
      .getBoundingClientRect();
    const screenCenter = screenBounds.y + screenBounds.height / 2;
    bodyStyle.transformOrigin = `50% ${screenCenter}px`;
    bodyStyle.transform = `scale(${width})`;
    await new Promise((resolve) => setTimeout(resolve, 1000));

    bodyStyle.transition = "transform 2s";

    bodyStyle.transform = "scale(1)";
    setIsShowActualScreen(false);
  }

  function popActualScreen() {
    const bodyStyle = document.querySelector("body").style;
    bodyStyle.transition = "unset";
    bodyStyle.transform = "scale(1)";
    setIsShowActualScreen(true);
  }

  return (
    <>
      <div
        onClick={() => startTask()}
        style={glow(".screen", isLightOn)}
        className={"screen" + (isTask ? " flash" : "")}
      ></div>
      {isShowActualScreen && (
        <div className="actual-screen">
          <ScreenNav />
          <Description handleSub={handleSub} tasks={tasks} />
          <Editor />
        </div>
      )}
    </>
  );
};

export default Screen;
