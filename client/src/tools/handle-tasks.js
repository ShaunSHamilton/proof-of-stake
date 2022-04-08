import { parse, parseBuffer } from "../../../utils/websockets/index";
import { info, error, warn, debug } from "../../../utils/logger";

const nodeEvents = {
  connect: (data, name) => {
    info(`Connected as Node: ${name}`);
    return {
      name,
      chain: data.chain.reverse(),
      tasks: data.tasks,
      transactionPool: data.transactionPool,
    };
  },
  ping: (data, name) => {},
  "update-chain": (data, name) => {
    debug(`Chain received: ${data}`);
    return {
      chain: data.chain.reverse(),
      tasks: data.tasks,
      transactionPool: data.transactionPool,
    };
  },
};

function handleEvent(state, setState, { data, name, type }) {
  const updatedStatePairs = nodeEvents[type]?.(data, name);
  setState((state) => ({ ...state, ...updatedStatePairs }));
}

export async function clientWebSocket(state, setState) {
  // Fetch port from server
  const { portForClient } = await (await fetch("/port")).json();
  info(`Connected on port ${portForClient}`);
  if (!portForClient) {
    throw new Error("No port found");
  }
  const prom = new Promise((resolve, _reject) => {
    const socket = new WebSocket(`ws://localhost:${portForClient}`);

    // Connection opened
    socket.addEventListener("open", (_event) => {
      info("Connection opened with serverside");
    });

    // Listen for messages
    socket.addEventListener("message", (event) => {
      const message = parseBuffer(event.data);
      const { data, name, type } = message;
      debug(`[${type}] From Server (${name}): `, data);
      handleEvent(state, setState, { data, name, type });
    });
    socket.addEventListener("error", (err) => {
      error(err);
    });
    socket.addEventListener("close", (event) => {
      warn(`Closed connection with: ${event.code}`);
      socket.close();
    });

    function sock(data, type) {
      const parsed = parse({ type, data });
      debug(parsed);
      socket.send(parsed);
    }
    resolve(sock);
  });
  return prom;
}
