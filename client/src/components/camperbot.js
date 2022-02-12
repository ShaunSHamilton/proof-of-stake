import "./camperbot.css";

const Camperbot = () => {
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
        <div className="camperbot-head">
          <div className="camperbot-eye left"></div>
          <div className="camperbot-eye right"></div>
          <div className="camperbot-mouth">
            <div className="speech-smoke">
              <div className="speech-bubble">
                Hello, I'm Camperbot. :) I am your server-side companion.
              </div>
            </div>
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
