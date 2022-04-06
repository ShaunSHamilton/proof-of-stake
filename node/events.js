import { nodeState, NAME } from "./state.js";
import { handle_mine, handle_validate } from "../blockchain/pkg/blockchain.js";
import { parse } from "../utils/websockets/index.js";
import { debug, error, info } from "../utils/logger.js";

import fs from "fs";
const quiz = JSON.parse(fs.readFileSync("../assets/quiz.json", "utf8"));

function broadcast({ data, name, type }) {
  nodeState.nodeSocks.forEach((sock) => {
    // debug(JSON.stringify({ data, type, name }, null, 2));
    try {
      sock.send(parse({ data, name, type }));
    } catch (err) {
      error(err);
    }
  });
  // Send to self?
  debug(data, name, type);
  handleNodeEvent({ data, name, type });
}

function handleSubmitTask({ data: { task, orderNumberSelected }, name }) {
  // Check if task is correct.
  const quizQ = quiz.find((q) => q.question === task.question);
  const result = Object.entries(quizQ.results).find(([key, val]) =>
    val.includes(orderNumberSelected)
  )?.[0];
  // Randomly decide if task has been correctly, incorrectly, or misbehavingly completed.
  const willCatchMisbehaviour = Math.random() > 0.2;
  return {
    taskValid: result === "correct" || !willCatchMisbehaviour,
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
  // nodeState.chain = proposedChain;
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
      // Add transaction to pool
      nodeState.transactionPool.push({
        event: "BuyRack",
        name: nodeState.name,
      });
    } else {
      broadcast({ data, name, type: "buy-rack" });
    }
  },
  stake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      // Add transaction to pool
      nodeState.transactionPool.push({
        event: "Stake",
        name: nodeState.name,
      });
    } else {
      broadcast({ data, name, type: "stake" });
    }
  },
  unstake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      // Add transaction to pool
      nodeState.transactionPool.push({
        event: "Unstake",
        name: nodeState.name,
      });
    } else {
      broadcast({ data, name, type: "unstake" });
    }
  },
  "submit-task": async (data, name) => {
    // Remove task from state
    nodeState.tasks = nodeState.tasks.filter(
      (task) => data.task.question !== task.question
    );
    // Broadcast for validation
    broadcast({ data, name, type: "submit-task" });
  },
};

export const nodeEvents = {
  "update-chain": async (data, name) => {
    if (nodeState.chain.length < data.chain.length) {
      nodeState.chain = data.chain;
    }
    nodeState.transactionPool.push({
      event: "UpdateChain",
      name,
    });
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
  "buy-rack": async (data, name) => {
    if (nodeState.isNextMiner()) {
      // Add transaction to pool
      nodeState.transactionPool.push({
        event: "BuyRack",
        name,
      });
    }
  },
  stake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      // Add transaction to pool
      nodeState.transactionPool.push({
        event: "Stake",
        name,
      });
    }
  },
  unstake: async (data, name) => {
    if (nodeState.isNextMiner()) {
      // Add transaction to pool
      nodeState.transactionPool.push({
        event: "Unstake",
        name,
      });
    }
  },
  "submit-task": async (data, name) => {
    if (nodeState.isNextMiner()) {
    }
    if (nodeState.isNextValidator()) {
      debug("Validating: ", data, name);
      const { taskValid } = handleSubmitTask({
        data,
        name,
      });
      nodeState.transactionPool.push({
        name,
        event: "SubmitTask",
      });
      broadcast({
        data: { taskValid },
        name,
        type: "task-validated",
      });
    }
  },
  "block-mined": async (data, name) => {
    // If isNextValidator, then validate, and emit "block-validated"
    if (nodeState.isNextValidator()) {
      const isValid = handle_validate({
        chain: data.chain,
        network: Array.from(nodeState.network),
      });
      if (isValid) {
        nodeState.chain = data.chain;
        nodeState.clientSocks.forEach((sock) => {
          sock.send(
            parse({
              data: { chain: nodeState.chain, tasks: nodeState.tasks },
              name,
              type: "update-chain",
            })
          );
        });
        broadcast({ data, name, type: "block-validated" });
      } else {
        broadcast({ data, name, type: "block-invalidated" });
      }
    }
  },
  "block-invalidated": async (data, name) => {
    // If next miner, add punishment to transaction pool
    if (nodeState.isNextMiner()) {
      nodeState.transactionPool.push({
        event: "BlockInvalidated",
        name,
      });
      // Try mine again?
      const [{ chain: proposedChain }, errors] = handle_mine({
        chain: {
          chain: nodeState.chain,
          network: Array.from(nodeState.network),
        },
        transactions: nodeState.transactionPool,
        task_valid: false, // Mine again, but with punishment
      });
      debug("Proposed chain: ", proposedChain, errors);
      // Clear transaction pool
      nodeState.transactionPool = [];
      // Handle proposed chain
      handleProposedBlock(proposedChain);
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
    const [{ chain: proposedChain }, errors] = handle_mine({
      chain: { chain: nodeState.chain, network: Array.from(nodeState.network) },
      transactions: nodeState.transactionPool,
      task_valid: data.taskValid,
    });
    debug("Proposed chain: ", proposedChain, errors);
    // Clear transaction pool
    nodeState.transactionPool = [];
    // Handle proposed chain
    handleProposedBlock(proposedChain);
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
      // debug("ee: ", e);
    }
    try {
      const res = await nodeEvents[type](parsed, name);
      return res;
    } catch (e) {
      // debug("Error handling event: ", e);
      return e;
    }
  }
  return "Invalid event type sent";
}
