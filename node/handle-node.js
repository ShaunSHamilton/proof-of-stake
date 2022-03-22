import fs from "fs";
import WebSocket, { WebSocketServer } from "ws";
import {
  findPortWebSocketServerListens,
  parseBuffer,
  parse,
  info,
  error,
  debug,
  findAvailablePort,
  warn,
} from "../utils/websockets/index.js";
// import * as wasmBuffer from "blockchain";
import { initialise_chain } from "../blockchain/pkg/blockchain.js";

export async function handleNodeWebsockets() {
  // Find peers
  const peerPorts = await findPortWebSocketServerListens(WebSocket, {
    timeout: 400,
    startPort: 30000,
    endPort: 30100,
    numberOfPorts: 3,
  });
  debug("peerPorts: ", peerPorts);
  if (!peerPorts.length) {
    // If no peers are found, then, as first node on network, initialise chain
    info("No peers found, initialising chain...");
    const { chain } = initialise_chain();
    warn(chain);
  }
  // Connect to peers
  for (const peerPort of peerPorts) {
    const peerSocket = new WebSocket(`ws://localhost:${peerPort}`);
    // Connection opened
    peerSocket.on("open", () => {});
    peerSocket.on("message", (data) => {
      const message = parseBuffer(data);
      info(`From peer (${message.name}): `, message.data);
    });
    peerSocket.on("error", (err) => {
      error(err);
    });
  }

  // Create a server for future peers to connect to
  const availableServerPort = await findAvailablePort(30000, 30100);
  const nodeWebSocketServer = new WebSocketServer({
    port: availableServerPort,
  });
  info(`Listening for peers on port ${availableServerPort}`);

  nodeWebSocketServer.on("connection", (ws, req) => {
    ws.on("message", (data) => {
      const message = parseBuffer(data);
      info(`From peer (${message.name}): `, message.data);
    });

    sock("connect", NAME, "Node says 'Hello!!'");

    function sock(type, name, data = {}) {
      ws.send(parse({ type, name, data }));
    }
  });
  nodeWebSocketServer.on("error", (err) => {
    error(err);
  });
}
