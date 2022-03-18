import { WebSocketServer } from "ws";
import { parseBuffer, parse, info } from "./utils/index.js";

export async function handleClientWebSocket() {
  // Create a server to listen for client connections
  const availableClientPort = await findAvailablePort(31000, 31100);
  const clientWebSocketServer = new WebSocketServer({
    port: availableClientPort,
  });
  info(`Listening for clients on port: ${availableClientPort}`);

  clientWebSocketServer.on("connection", (ws, req) => {
    ws.on("message", async (requestData) => {
      const { type, name, data } = parseBuffer(requestData);
      info(`From client (${name}): `, data);
      const res = await handleClientType({ type, name, data });
      sock("message", NAME, {
        data: res,
      });
    });
    ws.on("close", (event) => {
      info(`Client disconnected: ${event}`);
    });
    ws.on("error", (err) => {
      warn(`Client connection error: ${err}`);
    });

    sock("connect", NAME, { data: "Node says 'Hello!'" });

    function sock(type, name, data = {}) {
      ws.send(parse({ type, name, data }));
    }
  });
}
