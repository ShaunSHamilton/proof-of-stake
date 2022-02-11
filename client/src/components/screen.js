import { useEffect, useState } from "react";
import Description from "./description";
import Editor from "./editor";
import ScreenNav from "./screen-nav";

const Screen = ({ tasks = [] }) => {
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
    const width = 100; //window.innerWidth / 130;
    const bodyStyle = document.querySelector("body").style;

    bodyStyle.transition = "transform 2s";
    bodyStyle.transformOrigin = `calc(50% + ${7 / width}%) 49%`;
    bodyStyle.transform = `scale(${width})`;

    // document.querySelector(".screen").style.cursor = "default";
    await new Promise((resolve) => setTimeout(resolve, 1800));
  }

  async function handleSub() {
    const width = window.innerWidth / 130;
    const bodyStyle = document.querySelector("body").style;
    bodyStyle.transformOrigin = `calc(50% + ${7 / width}%) 49%`;
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
