function connect(ws, data) {
  console.log("Connected...", data);
}

const clientEvents = {
  connect,
  // "get-chain": handleGetChain,
};

const nodeEvents = {
  connect,
  // "mine-block": handleMineBlock,
};

module.exports = {
  clientEvents,
  nodeEvents,
};
