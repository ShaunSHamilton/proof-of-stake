// Connect to both the client and node websockets
const WebSocket = require("ws");
const { getNodeWebSocketServer } = require("./handle-node");
const { getClientWebSocketServer } = require("./handle-client");
const { parseBuffer } = require("./utils");

(async () => {
  const nodeWebSocketServer = await getNodeWebSocketServer();
  const clientWebSocketServer = await getClientWebSocketServer();

  console.log(
    "Node address: ",
    JSON.stringify(nodeWebSocketServer.address(), null, 2)
  );
  console.log(
    "Client address: ",
    JSON.stringify(clientWebSocketServer.address(), null, 2)
  );

  // const myPort = nodeWebSocketServer.address()?.port;

  for (let i = 8020; i < 8030; i++) {
    try {
      const ws = new WebSocket(`ws://localhost:${i}`);

      ws.on("open", () => {
        ws.send(
          JSON.stringify({ event: "connect", data: { message: "Hello!" } })
        );
      });

      ws.on("message", (data) => {
        const parsedData = parseBuffer(data);
        console.log("ind: ", parsedData);
      });
      ws.on("error", (err) => {
        console.log("No connection on: ", i);
      });
    } catch (e) {
      console.error("Failed to connect: ", e);
    }
  }
})();
