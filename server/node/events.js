function connect(ws, data) {
  console.log("Connected...", data);
}

export const clientEvents = {
  test: async (name, data) => "test worked",
  stake: async (name, data) => "staked!",
  unstake: async (name, data) => "unstaked!",
  "get-node-account": async (name, data) => ({
    name,
    tokens: 100,
    staked: 90,
    reputation: 8,
  }),
  "get-node-accounts": async (name, data) => [
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
  "get-chain": async (name, data) => "",
  "submit-task": async (name, data) => "",
  ping: async (name, data) => "pong",
  connect: async (name, data) => "Welcome!",
};

export const nodeEvents = {
  connect: async (name, data) => "Welcome!",
  ping: async (name, data) => "pong",
  "get-chain": async (name, data) => "",
};
