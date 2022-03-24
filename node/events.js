import { nodeState } from "./state.js";

// TODO: If chain is updated, broadcast

function broadcast({ data, name, type }) {
  nodeState.nodeSocks.forEach((sock) => {
    sock.send(parse({ data, name, type }));
  });
}

function handleRustResult() {}

export const clientEvents = {
  // GET EVENTS: Return information
  ping: async (data, name) => "pong",
  connect: async (data, name) => "Welcome!",
  "get-node-account": async (data, name) => ({
    name,
    tokens: 100,
    staked: 90,
    reputation: 8,
  }),
  "get-node-accounts": async (data, name) => [
    {
      name,
      tokens: 100,
      staked: 90,
      reputation: 8,
    },
    {
      name: "Tom",
      tokens: 10,
      staked: 9,
      reputation: 2,
    },
  ],
  "get-chain": async (data, name) => "",
  // POST EVENTS: Return "Request Received"
  "buy-rack": async (data, name) => {
    if (nodeState.isNextMiner) {
      handle_buy_rack(name);
    } else {
      broadcast(data, name);
    }
  },
  stake: async (data, name) => "staked!",
  unstake: async (data, name) => "unstaked!",
  "submit-task": async (data, name) => "",

  // OTHER EVENTS:
  test: async (data, name) => "test worked",
};

export const nodeEvents = {
  // UPDATE EVENTS: Return latest blockchain
  connect: async (data, name) => "Welcome!",

  // BLOCKCHAIN EVENTS: Mine and broadcast
  "block-mined": async (data, name) => {
    // If isNextValidator, then validate, and emit "block-validated"
  },
  "block-validated": async (data, name) => {
    // Emitted event from next_validators. Contains most up-to-date chain.
  },
  // OTHER EVENTS:
  ping: async (data, name) => "pong",
};

export async function handleClientEvent({ data, name, type }) {
  if (clientEvents[type]) {
    return clientEvents[type](data, name);
  }
  return "Invalid event type sent";
}

export async function handleNodeEvent({ data, name, type }) {
  return await nodeEvents[type]?.(data, name);
}
