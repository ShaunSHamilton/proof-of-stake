import "./monitor.css";

import Screen from "./screen";

const Monitor = ({ tasks, isLightOn }) => {
  return (
    <div className="monitor">
      <Screen tasks={tasks} isLightOn={isLightOn} />
      <div className="arm"></div>
      <div className="stand"></div>
    </div>
  );
};

export default Monitor;
