import LightBulb from "./light-bulb";

const Ceiling = () => {
  const isOn = true;
  return (
    <div id="ceiling">
      <LightBulb isOn={isOn} />
    </div>
  );
};

export default Ceiling;
