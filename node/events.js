import { nodeState, NAME } from "./state.js";
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
import { debug, error, info, parse } from "../utils/websockets/index.js";

import fs from "fs";
const quiz = JSON.parse(fs.readFileSync("../assets/quiz.json", "utf8"));

function broadcast({ data, name, type }) {
  nodeState.nodeSocks.forEach((sock) => {
    debug(JSON.stringify({ data, type, name }, null, 2));
    try {
      sock.send(parse({ data, name, type }));
    } catch (err) {
      error(err);
    }
  });
}

function handleSubmitTask({ data: { task, orderNumberSelected }, name }) {
  // Check if task is correct.
  const quizQ = quiz.find((q) => q.question === task.question);
  const result = Object.entries(quizQ.results).find(([key, val]) =>
    val.includes(orderNumberSelected)
  )?.[0];
  // Randomly decide if task has been correctly, incorrectly, or misbehavingly completed.
  const willCatchMisbehaviour = Math.random() > 0.2;
  // If correct, single reward
  // If incorrect, single punishement
  // If misbehaved and !willCatchMisbehaviour, reward
  // If misbehaved and willCatchMisbehaviour, punishment
  return {
    isShouldReward:
      result === "correct" ||
      (result === "misbehaved" && !willCatchMisbehaviour),
    isShouldPunish:
      (result === "misbehaved" && willCatchMisbehaviour) ||
      result === "incorrect",
  };
}

export function addTaskToState(task) {
  nodeState.tasks.push(task);
}

export function getRandomTask() {
  const randomIndex = Math.floor(Math.random() * quiz.length);
  const task = quiz[randomIndex];
  return { question: task.question, options: task.options };
}

function handleProposedBlock(proposedChain) {
  nodeState.chain = proposedChain;
  broadcast({
    data: { chain: proposedChain },
    name: NAME,
    type: "block-mined",
  });
}

export const clientEvents = {
  // GET EVENTS: Return information
  ping: async (data, name) => "pong",
  "buy-rack": async (data, name) => {
    if (nodeState.isNextMiner()) {
      const { chain: proposedChain } = handle_buy_rack(
        { chain: nodeState.chain, network: Array.from(nodeState.network) },
        name
      );
      handleProposedBlock(proposedChain);
    } else {
      broadcast({ data, name, type: "buy-rack" });
    }
  },
  stake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      const { chain: proposedChain } = handle_stake(
        { chain: nodeState.chain, network: Array.from(nodeState.network) },
        name
      );
      handleProposedBlock(proposedChain);
    } else {
      broadcast({ data, name, type: "stake" });
    }
  },
  unstake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      const { chain: proposedChain } = handle_unstake(
        { chain: nodeState.chain, network: Array.from(nodeState.network) },
        name
      );
      handleProposedBlock(proposedChain);
    } else {
      broadcast({ data, name, type: "unstake" });
    }
  },
  "submit-task": async (data, name) => {
    // Remove task from state
    nodeState.tasks = nodeState.tasks.filter(
      (task) => data.task.question !== task.question
    );
    // Current node should never be validator for own task
    broadcast({ data, name, type: "submit-task" });
  },
};

export const nodeEvents = {
  "update-chain": async (data, name) => {
    if (nodeState.chain.length < data.chain.length) {
      nodeState.chain = data.chain;
    }
    const { chain: proposedChain } = handle_connection(
      { chain: nodeState.chain, network: Array.from(nodeState.network) },
      name
    );
    info("Proposing new chain...", proposedChain);
    handleProposedBlock(proposedChain);
    nodeState.network.add(name);
    nodeState.clientSocks.forEach((sock) => {
      sock.send(
        parse({
          data: { chain: nodeState.chain, tasks: nodeState.tasks },
          name,
          type: "update-chain",
        })
      );
    });
  },
  // ALL CLIENT EVENTS: Without broadcast, to prevent infinite loop
  "buy-rack": async (data, name) => {
    if (nodeState.isNextMiner()) {
      const { chain: proposedChain } = handle_buy_rack(
        { chain: nodeState.chain, network: Array.from(nodeState.network) },
        name
      );
      handleProposedBlock(proposedChain);
    }
  },
  stake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      const { chain: proposedChain } = handle_stake(
        { chain: nodeState.chain, network: Array.from(nodeState.network) },
        name
      );
      handleProposedBlock(proposedChain);
    }
  },
  unstake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      const { chain: proposedChain } = handle_unstake(
        { chain: nodeState.chain, network: Array.from(nodeState.network) },
        name
      );
      handleProposedBlock(proposedChain);
    }
  },
  "submit-task": async (data, name) => {
    if (nodeState.isNextMiner()) {
      // DO NOTHING
    }
    if (nodeState.isNextValidator()) {
      const { isShouldReward, isShouldPunish } = handleSubmitTask({
        data,
        name,
      });
      broadcast({
        data: { isShouldPunish, isShouldReward },
        name,
        type: "task-validated",
      });
    }
  },
  "block-mined": async (data, name) => {
    // If isNextValidator, then validate, and emit "block-validated"
    try {
      debug("Validating block...", nodeState.isNextValidator());
    } catch (err) {
      error(err);
    }
    if (nodeState.isNextValidator()) {
      const isValid = handle_validate({
        chain: data.chain,
        network: Array.from(nodeState.network),
      });
      debug("Block is valid: ", isValid);
      if (isValid) {
        nodeState.chain = data.chain;
        broadcast({ data, name, type: "block-validated" });
      } else {
        handle_punish(
          { chain: nodeState.chain, network: Array.from(nodeState.network) },
          name
        );
      }
    }
  },
  "block-validated": async (data, name) => {
    // Emitted event from next_validators. Contains most up-to-date chain.
    nodeState.chain = data.chain;

    if (nodeState.isNextMiner()) {
      addTaskToState(getRandomTask());
    }
    // Send client updated chain
    nodeState.clientSocks.forEach((sock) => {
      sock.send(
        parse({
          data: { chain: nodeState.chain, tasks: nodeState.tasks },
          name,
          type: "update-chain",
        })
      );
    });
  },
  "task-validated": async (data, name) => {
    if (nodeState.isNextMiner()) {
      if (data.isShouldPunish) {
        const { chain: proposedChain } = handle_punish(
          { chain: nodeState.chain, network: Array.from(nodeState.network) },
          name
        );
        handleProposedBlock(proposedChain);
      } else if (data.isShouldReward) {
        const { chain: proposedChain } = handle_reward(
          { chain: nodeState.chain, network: Array.from(nodeState.network) },
          name
        );
        handleProposedBlock(proposedChain);
      }
    }
  },
  // OTHER EVENTS:
  ping: async (data, name) => "pong",
};

export async function handleClientEvent({ data, name, type }) {
  if (clientEvents[type]) {
    let parsed = data;
    try {
      // debug(`${name} sent ${data}`);
      parsed = JSON.parse(data);
    } catch (e) {
      // debug(`Error parsing JSON: ${data}`);
    }
    try {
      const res = await clientEvents[type](parsed, name);
      return res ?? 200;
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
      // debug(`${name} sent '${data}'`);
      parsed = JSON.parse(data);
    } catch (e) {
      // debug(e);
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
