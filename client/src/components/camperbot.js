// import { useCallback } from "react";
// import useDraggable from "../tools/draggable";
import "./camperbot.css";
import glow from "../tools/glow";

const Camperbot = ({
  text,
  isLightOn,
  handleNextBubble,
  handlePreviousBubble,
}) => {
  // const handleDrag = useCallback(
  //   ({ x, y }) => ({
  //     x: Math.max(0, x),
  //     y: Math.max(0, y),
  //   }),
  //   []
  // );

  // // eslint-disable-next-line no-unused-vars
  // const [ref, _pressed] = useDraggable({
  //   onDrag: handleDrag,
  // });

  return (
    <div className="camperbot">
      <div className="camperbot-body">
        <div className="camperbot-hat">
          <div className="ball"></div>

          <div className="rod">
            <div className="ring"></div>
            <div className="ring"></div>
          </div>
          <div className="bowl">
            <div className="logo"></div>
          </div>
        </div>
        <div
          className="camperbot-head"
          style={glow(".camperbot-head", isLightOn)}
        >
          <div className="camperbot-eye left"></div>
          <div className="camperbot-eye right"></div>
          <div className="camperbot-mouth">
            {text && (
              <div className="speech-smoke">
                <div className="speech-bubble">
                  <p>{text}</p>
                  <button onClick={() => handlePreviousBubble()}>&lt;</button>
                  <button onClick={() => handleNextBubble()}>&gt;</button>
                </div>
              </div>
            )}
          </div>
        </div>
        <div className="camperbot-neck"></div>
        <div className="camperbot-torso">CAMPERBOT</div>
        <div className="camperbot-prop">
          <div className="top"></div>
          <div className="mid"></div>
          <div className="bot"></div>
        </div>
      </div>
    </div>
  );
};

export default Camperbot;
