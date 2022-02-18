import { useEffect, useState } from "react";
import ScreenNav from "./screen-nav";
import { scramble } from "../tools/utils";

import glow from "../tools/glow";

const Screen = ({ task = {}, isLightOn }) => {
  const [isTask, setIsTask] = useState(Object.keys(task).length > 0);
  const [isShowActualScreen, setIsShowActualScreen] = useState(false);

  useEffect(() => {
    setIsTask(Object.keys(task).length > 0);
  }, [task]);

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

  async function handleSub(orderNumberSelected) {
    // Send data to peers
    // submitTask(task, orderNumberSelected);

    // Change view back to main screen
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

  async function submitTask(task, orderNumber) {
    try {
      const response = await fetch(`/api/tasks/${task.id}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          orderNumber,
        }),
      });
      const data = await response.json();
      console.log(data);
    } catch (err) {
      console.log(err);
    }
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
          <Quizzer
            handleSub={handleSub}
            question={task.quiz.question}
            options={scramble(task.quiz.options)}
          />
        </div>
      )}
    </>
  );
};

const Quizzer = ({ handleSub, question, options }) => {
  const [selected, setSelected] = useState(null);

  return (
    <>
      <section className="description">
        <div className="content">{question}</div>
      </section>
      <ul className="options">
        {options.map((option) => {
          return (
            <li
              className={
                "options-item" + (selected === option.order ? " selected" : "")
              }
              key={option.order}
            >
              <label>
                <input
                  type="radio"
                  name="language"
                  value={option.order}
                  onChange={() => setSelected(option.order)}
                />
                {option.code}
              </label>
            </li>
          );
        })}
      </ul>
      <div className="submit">
        <button
          onClick={(e) => {
            if (e.defaultPrevented) return;
            e.preventDefault();
            e.stopPropagation();
            handleSub(selected);
          }}
        >
          Submit
        </button>
      </div>
    </>
  );
};

export default Screen;
