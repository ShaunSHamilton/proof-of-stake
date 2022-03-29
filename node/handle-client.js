import { WebSocketServer } from "ws";
import {
  parseBuffer,
  parse,
  info,
  findAvailablePort,
} from "../utils/websockets/index.js";
import { handleClientEvent } from "./events.js";
import { nodeState } from "./state.js";

export async function handleClientWebSocket() {
  // Create a server to listen for client connections
  const availableClientPort = await findAvailablePort(31000, 31100);
  const clientWebSocketServer = new WebSocketServer({
    port: availableClientPort,
  });
  info(`Listening for clients on port: ${availableClientPort}`);
  clientWebSocketServer.on("connection", (ws, req) => {
    nodeState.clientSocks.push(ws);
    ws.on("message", async (requestData) => {
      const { type, data } = parseBuffer(requestData);
      info(`From client: `, data);
      const res = await handleClientEvent({ type, name: nodeState.name, data });
      sock(res, nodeState.name, type);
    });
    ws.on("close", (event) => {
      info(`Client disconnected: ${event}`);
    });
    ws.on("error", (err) => {
      warn(`Client connection error: ${err}`);
    });

    sock({ chain: nodeState.chain }, nodeState.name, "connect");

    function sock(data, name, type) {
      ws.send(parse({ type, name, data }));
    }
  });

  return availableClientPort;
}
