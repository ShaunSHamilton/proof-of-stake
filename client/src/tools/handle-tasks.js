export async function getTasks() {
  // Listen for tasks from peers

  // Assign tasks to respective servers
  const quizzes = await (await fetch("/quiz.json")).json();
  const getTasksSample = quizzes.map((quiz, index) => {
    return {
      nodeOwner: "Camper",
      quiz,
      id: index,
    };
  });
  // return tasks attached to node names
  return getTasksSample;
}

export async function getNodeAccount(nodeOwner = "Camper") {
  // get account meta from blockchain

  // return account meta
  return getNodeAccountSample;
}

export async function putStaking(isStake, nodeOwner = "Camper") {
  // put staking to blockchain
  if (isStake) {
    getNodeAccountSample.staked += 1;
  } else {
    getNodeAccountSample.staked -= 1;
  }
}

const getNodeAccountSample = {
  nodeOwner: "Camper",
  tokens: 100,
  staked: 90,
  reputation: 8,
};
