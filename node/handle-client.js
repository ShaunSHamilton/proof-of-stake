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
  const prom = new Promise((resolve, reject) => {
    clientWebSocketServer.on("connection", (ws, req) => {
      nodeState.clientSocks.push(ws);
      ws.on("message", async (requestData) => {
        const { type, name, data } = parseBuffer(requestData);
        info(`From client (${name}): `, data);
        const res = await handleClientEvent({ type, name, data });
        sock(res, nodeState.name, "res");
      });
      ws.on("close", (event) => {
        info(`Client disconnected: ${event}`);
      });
      ws.on("error", (err) => {
        warn(`Client connection error: ${err}`);
      });

      sock({ data: "Node says 'Hello!'" }, nodeState.name, "connect");

      function sock(data, name, type) {
        ws.send(parse({ type, name, data }));
      }
      resolve(sock);
    });
  });
  return prom;
}
