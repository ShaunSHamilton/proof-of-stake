const nodeState = {
  isNextMiner: false,
  isNextValidator: false,
};

function broadcast({ data, name, type }) {
  nodeState.nodeSocks.forEach((sock) => {
    sock.send(parse({ data, name, type }));
  });
}

function handleRustResult() {}

export const clientEvents = {
  "buy-rack": async (data, name) => {
    if (nodeState.isNextMiner) {
      handle_buy_rack(name);
    } else {
      broadcast(data, name);
    }
  },
  // Tests
  test: async (data, name) => "test worked",
  stake: async (data, name) => "staked!",
  unstake: async (data, name) => "unstaked!",
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
  "submit-task": async (data, name) => "",
  ping: async (data, name) => "pong",
  connect: async (data, name) => "Welcome!",
};

export const nodeEvents = {
  connect: async (data, name) => "Welcome!",
  ping: async (data, name) => "pong",
  "get-chain": async (data, name) => {
    return nodeState.chain;
  },
};

export async function handleClientEvent({ data, name, type }) {
  if (clientEvents[type]) {
    return clientEvents[type](data, name);
  }
  return "no event";
}

export async function handleNodeEvent({ data, name, type }) {
  return await nodeEvents[type]?.(data, name);
}
