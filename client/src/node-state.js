import { createContext } from "react";
import { debug } from "../../utils/websockets";

export const state = {
  sock: null,
  name: "Camper",
  chain: [],
  tasks: [],
};

export function getSelf() {
  return state.chain.reverse().find((node) => node.name === state.name);
}

export function getOtherNodes() {
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

export const dispatchStake = () => {
  debug("Dispatching stake");
  state.sock({}, "stake");
};
export const dispatchUnstake = () => {
  debug("Dispatching unstake");
  state.sock({}, "unstake");
};
export const dispatchSubmitTask = (task) => {
  debug("Dispatching task");
  state.sock(task, "submit-task");
};
export const dispatchBuyRack = () => {
  debug("Dispatching buy rack");
  state.sock({}, "buy-rack");
};

export const NodeContext = createContext(state);
