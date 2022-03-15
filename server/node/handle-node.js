const WebSocket = require("ws");
const { WebSocketServer } = WebSocket;
const { nodeEvents } = require("./events");
const { parse, parseBuffer } = require("./utils");

async function getNodeWebSocketServer() {
  let portFound = null;
  let error = null;
  let nodeWebSocketServer = null;
  let port = 8020;
  do {
    try {
      nodeWebSocketServer = new WebSocketServer({ port });
      nodeWebSocketServer.on("error", (err) => {
        error = err;
        port++;
      });
      // Timeout for 2 seconds
      await new Promise(
        (resolve, reject) => setTimeout(() => (error ? reject() : resolve())),
        2000
      );
      error = null;
      portFound = port;
    } catch (e) {
      error = e;
      port++;
    }
  } while (!portFound && port < 8030);

  if (error) {
    throw Error(error);
  }

  console.log("Node port: ", port, portFound);

  nodeWebSocketServer.on("connection", function connection(ws) {
    ws.on("message", function message(data) {
      nodeWebSocketServer.clients.forEach(function each(node) {
        if (node !== ws && node.readyState === WebSocket.OPEN) {
          // node.send(data, { binary: isBinary });
          const parsedData = parseBuffer(data);
          nodeEvents[parsedData.event]?.(node, parsedData);
        }
      });
    });
    sock("connect", { message: "Node says 'Hello!'" });

    function sock(type, data = {}) {
      ws.send(parse({ event: type, data }));
    }
  });

  return nodeWebSocketServer;
}

module.exports = {
  getNodeWebSocketServer,
};
