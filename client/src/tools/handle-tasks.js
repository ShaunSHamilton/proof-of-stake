import {
  findPortWebSocketServerListens,
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

export async function getNodeAccount(nodeOwner = "Camper") {
  // get account meta from blockchain

  // return account meta
  return getNodeAccountSample;
}

export async function putStaking(isStake, nodeOwner = "Camper") {
  // put staking to blockchain
  if (isStake) {
    getNodeAccountSample.staked += 1;
  } else {
    getNodeAccountSample.staked -= 1;
  }
}

const getNodeAccountSample = {
  nodeOwner: "Camper",
  tokens: 100,
  staked: 90,
  reputation: 8,
};

// clientTypes:
// - stake
// - unstake
// - getNodeAccount
// - submitTask
// -

async function main() {
  const clientPorts = [];
  while (clientPorts.length === 0) {
    // Find listening ports on network.
    clientPorts.push(
      ...(await findPortWebSocketServerListens(WebSocket, {
        timeout: 900,
        startPort: 31000,
        endPort: 31100,
        numberOfPorts: 4,
      }))
    );
    info(`Found all these: ${clientPorts}`);
  }
  const CP = clientPorts[Math.floor(Math.random() * clientPorts.length)];
  info(`Connected on port ${CP}`);
  if (!CP) {
    throw new Error("No port found");
  }
  const socket = new WebSocket(`ws://localhost:${CP}`);

  // Connection opened
  socket.addEventListener("open", (_event) => {
    info("opened");
    sock("connect", "", "Client says 'Hello'");
  });

  // Listen for messages
  socket.addEventListener("message", (event) => {
    const message = parseBuffer(event.data);
    info(`From Server (${event.origin}): `, message.data);
  });
  socket.addEventListener("error", (err) => {
    error(err);
  });
  socket.addEventListener("close", (event) => {
    warn(`Closed: ${event.code}`);
    socket.close();
  });

  function sock(type, name, data = "") {
    const parsed = parse({ type, name, data });
    debug(parsed);
    socket.send(parsed);
  }
}

main();
