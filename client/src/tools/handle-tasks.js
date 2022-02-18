export function getTasks() {
  // Listen for tasks from peers

  // Assign tasks to respective servers

  // return tasks attached to node names
  return getTasksSample;
}

export function getNodeAccount(nodeOwner = "Camper") {
  // get account meta from blockchain

  // return account meta
  return getNodeAccountSample;
}

const sampleTask = {
  nodeOwner: "Camper",
  quiz: {
    question:
      "How much wood could a woodchuck chuck, if a woodchuck could chuck wood?", //
    options: [
      {
        code: "As much wood as a woodchuck could chuck",
        order: 0,
      },
      {
        code: "Enough wood to make a woodchuck chuck",
        order: 1,
      },
      {
        code: "Yes",
        order: 2,
      },
    ],
    // Do not send results to servers?
    results: {
      correct: [0],
      incorrect: [1],
      misbehaved: [2],
    },
  },
  id: 1, // ID === nonce solution?
};

const getTasksSample = [sampleTask];

const getNodeAccountSample = {
  nodeOwner: "Camper",
  tokens: 100,
  staked: 90,
  reputation: 8,
};
