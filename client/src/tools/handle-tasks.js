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
    question: "I am a question", //
    options: [
      {
        code: "Option 1",
        order: 0,
      },
      {
        code: "option 2",
        order: 1,
      },
      {
        code: "option 3",
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
  tokens: 30,
  staked: 27,
  reputation: 3,
};
