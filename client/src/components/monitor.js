import Screen from "./screen";

const Monitor = ({ tasks }) => {
  return (
    <div className="monitor">
      <Screen tasks={tasks} />
      <div className="arm"></div>
      <div className="stand"></div>
    </div>
  );
};

export default Monitor;
