import { createContext } from "react";
import { debug, info } from "../../utils/logger";

export const state = {
  sock: null,
  name: "Camper",
  chain: [],
  tasks: [],
  transactionPool: [],
};

export const NodeContext = createContext(state);

export function getSelf(state) {
  for (const block of state.chain) {
    for (const node of block.data) {
      if (node.name === state.name) {
        return node;
      }
    }
  }
  return { tokens: 0, staked: 0, reputation: 0, racks: 0 };
}

// Broken
export function getOtherNodes(state) {
  const uniqueNodeSet = [];
  state.chain.forEach((node) => {
    if (
      node.name !== state.name &&
      !uniqueNodeSet.find((n) => n.name === node.name)
    ) {
      uniqueNodeSet.push(node);
    }
  });
  return uniqueNodeSet;
}

export const dispatchStake = (state) => {
  info("Dispatching stake");
  state.sock({}, "stake");
  if (state.setTutorialState) {
    state.setTutorialState((prev) => {
      const newState = { ...prev };
      newState.chain[0].data[0].staked += 1;
      return newState;
    });
  }
};
export const dispatchUnstake = (state) => {
  debug("Dispatching unstake");
  state.sock({}, "unstake");
  if (state.setTutorialState) {
    state.setTutorialState((prev) => {
      const newState = { ...prev };
      newState.chain[0].data[0].staked -= 1;
      return newState;
    });
  }
};
export const dispatchSubmitTask = (state, task) => {
  info("Dispatching task", NodeContext);
  state.sock(task, "submit-task");
  if (state.setTutorialState) {
    state.setTutorialState((prev) => {
      const newState = { ...prev };
      newState.tasks = [];
      return newState;
    });
  }
};
export const dispatchBuyRack = (state) => {
  debug("Dispatching buy rack");
  state.sock({}, "buy-rack");
  if (state.setTutorialState) {
    state.setTutorialState((prev) => {
      const newState = { ...prev };
      newState.chain[0].data[0].racks += 1;
      newState.chain[0].data[0].tokens -= 10;
      return newState;
    });
  }
};
