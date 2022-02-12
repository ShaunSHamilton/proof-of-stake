import "./light-bulb.css";

const LightBulb = ({ isOn }) => {
  return (
    <div className="light-bulb">
      <div className="wire"></div>
      <div className="shade"></div>
      <div className="bulb"></div>
      <div className="light"></div>
    </div>
  );
};

export default LightBulb;
