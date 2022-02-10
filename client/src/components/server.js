// Colourful LED
const randomHexColours = [
  "#FF0000",
  "#FF7F00",
  "#FFFF00",
  "#00FF00",
  "#0000FF",
  "#4B0082",
  "#9400D3",
  "#FF1493",
];
const Server = ({ isServerOn, isTaskAvailable }) => {
  const randomLed = (i, j) => {
    const randomNumber = Math.floor(Math.random() * i);
    return randomNumber % j !== i;
  };
  const randomBlink = (i, j) => {
    const randomNumber = Math.floor(Math.random() * i);
    const randomInd = Math.floor(
      Math.random() * (isTaskAvailable ? 2 : randomHexColours.length)
    );
    return {
      animationDuration: `${randomNumber % j}s`,
      animationIterationCount: "infinite",
      animationTimingFunction: "linear",
      animationName: "blink",
      backgroundColor: randomHexColours[randomInd],
    };
  };
  return (
    <div className="server">
      {[...Array(6)].map((_, i) => {
        return (
          <div className="status-group" key={i}>
            {[...Array(4)].map((_, j) => {
              return (
                <div
                  className="status-led"
                  style={isServerOn && randomLed(i, j) ? randomBlink(i, j) : {}}
                  key={j}
                ></div>
              );
            })}
          </div>
        );
      })}
    </div>
  );
};

export default Server;
