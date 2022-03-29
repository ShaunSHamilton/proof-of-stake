import { useEffect, useState } from "react";
import "./camperbot.css";
import glow from "../tools/glow";
import { getSelf } from "../node-state";

const Camperbot = ({
  text,
  setText,
  isLightOn,
  handleNextBubble,
  handlePreviousBubble,
}) => {
  const [isShowBubble, setIsShowBubble] = useState(true);
  const [typewriter, setTypewriter] = useState(text[0] || "");

  const toggleBubble = () => {
    setIsShowBubble(!isShowBubble);
  };

  const handleStats = (e) => {
    e.preventDefault();
    e.stopPropagation();
    if (!isShowBubble) {
      toggleBubble();
    }
    (async () => {
      const { tokens, staked, reputation } = await getSelf();
      const statText = `Tokens: ${tokens}\nStaked: ${staked}\nReputation: ${reputation}`;
      setText(statText);
    })();
  };

  useEffect(() => {
    let c = 0;
    const interval = setInterval(() => {
      if (c <= text.length) {
        setTypewriter(text.slice(0, c));
        c++;
      } else {
        return clearInterval(interval);
      }
    }, 20);
  }, [text]);

  return (
    <div className="camperbot" onClick={() => toggleBubble()}>
      <div className="camperbot-body">
        <div className="camperbot-hat" onClick={handleStats}>
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
            {isShowBubble && text && (
              <div className="speech-smoke">
                <div className="speech-bubble">
                  <div className="speech-buttons">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handlePreviousBubble();
                      }}
                    >
                      &lt;
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleNextBubble();
                      }}
                    >
                      &gt;
                    </button>
                  </div>
                  <p>{typewriter}</p>
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
