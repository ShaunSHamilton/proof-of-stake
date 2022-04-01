import {
  parse,
  parseBuffer,
  info,
  error,
  warn,
  debug,
} from "../../../utils/websockets/index";

// export async function getTasks() {
//   // Listen for tasks from peers

//   // Assign tasks to respective servers
//   const quizzes = await (await fetch("/quiz.json")).json();
//   const getTasksSample = quizzes.map((quiz, index) => {
//     return {
//       nodeOwner: "Camper",
//       quiz,
//       id: index,
//     };
//   });
//   // return tasks attached to node names
//   return getTasksSample;
// }

// const dataReceived = { type: "response", data: 200 };

const nodeEvents = {
  connect: (data, name) => {
    info(`Connected as Node: ${name}`);
    return { name, chain: data.chain, tasks: data.tasks };
  },
  ping: (data, name) => {},
  "update-chain": (data, name) => {
    info(`Chain received: ${data}`);
    return { chain: data.chain, tasks: data.tasks };
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
      info("opened");
      // sock("Client says 'Hello'", "connect");
    });

    // Listen for messages
    socket.addEventListener("message", (event) => {
      const message = parseBuffer(event.data);
      const { data, name, type } = message;
      info(`[${type}] From Server (${name}): `, data);
      handleEvent(state, setState, { data, name, type });
    });
    socket.addEventListener("error", (err) => {
      error(err);
    });
    socket.addEventListener("close", (event) => {
      warn(`Closed: ${event.code}`);
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
