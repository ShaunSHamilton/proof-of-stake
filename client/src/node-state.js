import { createContext } from "react";
import { debug, info } from "../../utils/websockets";

export const state = {
  sock: null,
  name: "Camper",
  chain: [],
  tasks: [],
};

export const NodeContext = createContext(state);

export function getSelf(state) {
  for (const block of state.chain.reverse()) {
    for (const node of block.data) {
      if (node.name === state.name) {
        return node;
      }
    }
  }
  return { tokens: 0, staked: 0, reputation: 0 };
}

// Broken
export function getOtherNodes(state) {
  const uniqueNodeSet = [];
  state.chain.reverse().forEach((node) => {
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
};
export const dispatchUnstake = (state) => {
  debug("Dispatching unstake");
  state.sock({}, "unstake");
};
export const dispatchSubmitTask = (state, task) => {
  info("Dispatching task", NodeContext);
  state.sock(task, "submit-task");
};
export const dispatchBuyRack = (state) => {
  debug("Dispatching buy rack");
  state.sock({}, "buy-rack");
};
