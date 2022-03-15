const WebSocket = require("ws");
const { WebSocketServer } = WebSocket;
const { clientEvents } = require("./events");
const { parse, parseBuffer } = require("./utils");

async function getClientWebSocketServer() {
  let portFound = null;
  let error = null;
  let clientWebSocketServer = null;
  let port = 8080;
  do {
    try {
      clientWebSocketServer = new WebSocketServer({ port });
      clientWebSocketServer.on("error", (err) => {
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
  } while (!portFound && port < 8090);

  if (error) {
    throw Error(error);
  }

  console.log("port: ", port);

  clientWebSocketServer.on("connection", function connection(ws) {
    ws.on("message", function message(data) {
      const parsedData = parseBuffer(data);
      clientEvents[parsedData.event]?.(ws, parsedData);
    });

    // wss.clients.forEach(function each(client) {
    //   if (node !== ws && node.readyState === WebSocket.OPEN) {
    //     // node.send(data, { binary: isBinary });
    //     const parsedData = parseBuffer(data);
    //     clientEvents[parsedData.event]?.(node, parsedData);
    //   }
    // });
    sock("connect", { message: "Node says 'Hello!!'" });

    function sock(type, data = {}) {
      ws.send(parse({ event: type, data }));
    }
  });

  return clientWebSocketServer;
}

module.exports = {
  getClientWebSocketServer,
};
