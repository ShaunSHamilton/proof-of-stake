export const NAME = process.env.NAME;

export const nodeState = {
  name: NAME,
  isNextMiner: function () {
    return this.chain[this.chain.length - 1].next_miner === this.name;
  },
  isNextValidator: function () {
    return this.chain[this.chain.length - 1].next_validators.includes(
      this.name
    );
  },
  chain: [],
  clientSocks: [],
  nodeSocks: [],
  tasks: [],
  network: new Set(),
};
