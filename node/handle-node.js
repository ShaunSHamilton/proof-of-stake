import WebSocket, { WebSocketServer } from "ws";
import {
  findPortWebSocketServerListens,
  parseBuffer,
  parse,
  info,
  error,
  debug,
  findAvailablePort,
} from "../utils/websockets/index.js";
import { initialise } from "../blockchain/pkg/blockchain.js";
import { nodeState, NAME } from "./state.js";
import { handleNodeEvent } from "./events.js";

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
    const { chain } = initialise(NAME);
    debug(chain);
    nodeState.chain = chain;
  }
  // Connect to peers
  for (const peerPort of peerPorts) {
    const peerSocket = new WebSocket(`ws://localhost:${peerPort}`);
    // Connection opened
    peerSocket.on("open", () => {
      nodeState.nodeSocks.push(peerSocket);
    });
    // Ask for latest chain
    peerSocket.on("message", async (requestData) => {
      const { data, name, type } = parseBuffer(requestData);
      debug(`[${type}] From peer (${name}): `, data);
      const res = await handleNodeEvent({ data, name, type });
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
    ws.on("message", async (requestData) => {
      const { data, name, type } = parseBuffer(requestData);
      debug(`[${type}] From peer (${name}): `, data);
      const res = await handleNodeEvent({ data, name, type });
      // sock(res, nodeState.name, "res");
    });

    sock({ chain: nodeState.chain }, nodeState.name, "update-chain");

    function sock(data, name, type = {}) {
      ws.send(parse({ data, name, type }));
    }
    nodeState.nodeSocks.push(ws);
  });
  nodeWebSocketServer.on("error", (err) => {
    error(err);
  });
}
