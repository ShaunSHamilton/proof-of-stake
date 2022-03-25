import { nodeState } from "./state.js";
import {
  handle_buy_rack,
  handle_connection,
  handle_get_node_by_name,
  handle_get_nodes,
  handle_punish,
  handle_reward,
  handle_stake,
  handle_unstake,
  handle_validate,
} from "../blockchain/pkg/blockchain.js";
import { debug } from "../utils/websockets/index.js";

// TODO: If chain is updated, broadcast

function broadcast({ data, name, type }) {
  nodeState.nodeSocks.forEach((sock) => {
    sock.send(parse({ data, name, type }));
  });
}

export const clientEvents = {
  // GET EVENTS: Return information
  ping: async (data, name) => "pong",
  connect: async (data, name) => "Welcome!",
  "get-node-account": async (data, name) => {
    const nodeAccount = handle_get_node_by_name(
      { chain: nodeState.chain },
      data.name
    );
    return nodeAccount;
  },
  "get-node-accounts": async (data, name) => {
    const nodeAccounts = handle_get_nodes({ chain: nodeState.chain });
    return nodeAccounts;
  },
  "get-chain": async (data, name) => {
    return nodeState.chain;
  },
  // POST EVENTS: Return "Request Received"
  "buy-rack": async (data, name) => {
    if (nodeState.isNextMiner) {
      handle_buy_rack({ chain: nodeState.chain }, name);
    } else {
      broadcast({ data, name, type: "buy-rack" });
    }
  },
  stake: async (data, name) => "staked!",
  unstake: async (data, name) => "unstaked!",
  "submit-task": async (data, name) => {},
};

export const nodeEvents = {
  // UPDATE EVENTS: Return latest blockchain

  // BLOCKCHAIN EVENTS: Mine and broadcast
  connect: async (data, name) => {
    handle_connection(name);
  },
  "block-mined": async (data, name) => {
    // If isNextValidator, then validate, and emit "block-validated"
    if (nodeState.isNextValidator) {
      const isValid = handle_validate({ chain: nodeState.chain });
      if (isValid) {
        broadcast({ data, name, type: "block-validated" });
      } else {
        handle_punish({ chain: nodeState.chain }, name);
      }
    }
  },
  "block-validated": async (data, name) => {
    // Emitted event from next_validators. Contains most up-to-date chain.
  },
  // OTHER EVENTS:
  ping: async (data, name) => "pong",
  res: async (data, name) => {
    Object.entries(data).forEach(([key, value]) => {
      nodeState[key] = value;
    });
  },
};

export async function handleClientEvent({ data, name, type }) {
  if (clientEvents[type]) {
    let parsed = data;
    try {
      parsed = JSON.parse(data);
    } catch (e) {
      debug(e);
    }
    try {
      const res = await clientEvents[type](parsed, name);
      return res;
    } catch (e) {
      return e;
    }
  }
  return "Invalid event type sent";
}

export async function handleNodeEvent({ data, name, type }) {
  if (nodeEvents[type]) {
    let parsed = data;
    try {
      parsed = JSON.parse(data);
    } catch (e) {
      debug(e);
    }
    try {
      const res = await nodeEvents[type](parsed, name);
      return res;
    } catch (e) {
      return e;
    }
  }
  return "Invalid event type sent";
}
