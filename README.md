# freeCodeCamp - Proof of Stake Prototype

Prototype Proof of Stake application for Web3 curriculum

### Prerequisites

- Intermediate JavaScript
- Basic HTML and CSS

- Cryptography
  - Hashing Functions
  - Signing
  - Verifying
  - Programatically Solving Hashes
- Blockchains
  - Linked Lists
  - Blocks
  - Tokens
- Wallets
  - Addresses
  - Private Keys
  - Public Keys
- Smart Contracts
  - Creation
  - Testing
  - Deployment

### Learning Outcomes

- Validating
- Minting
- Staking
- Security
  - 51% Attacks
- Rust

## File Structure

- node
  - An instance of the server
- client
  - React App
- server
  - app.js
- test

## Project Ideas

- Accumulation of resources increases probability of being chosen to do work
- If chosen to do work, you have the option of doing what is requested, or _misbehaving_
- Completing work gets you a reward
- _Misbehaving_ might get you more of a reward
- _Misbehaving_ might get you a punishment
- Punishment is reduction of resources
- Whether you get punished or not is dependent on the number of _players_ watching you
- If you own `>50%` of the resources, you can _misbehave_ without punishment, provided there is only _one_ player watching you

Intern at freeCodeCamp in our server room. Help debug this VM. Improve it, or introduce more bugs. If you introduce **any** jQuery, you have _definitely_ misbehaved

- Resources
  - Money to purchase skillsets. Buy Nodejs Course... Buy jQuery course to misbehave
  - Reputation - trust - watched less closely by nodes. Higher rep === more work
- Mechanics
  - Walk to VM to do tasks
- Tasks
  - Quiz
  - Solve algorithm
- Text based, MMO, or simple boxes/rooms to click

---

## Proof of Stake

### Glossary

**Node** - An instance of a server running the blockchain protocol
**Client** - A browser that connects to the blockchain
**Validator** - A node ensuring validity of the blockchain
**Miner** - A node chosen to mine the next block

### Hamilton Basic Protocol (HBP)

The _miner_ forges the next _miner_, upon validation.

The _miner_ is determined by weight of reputation and staking.

A _validator_ is determined by weight of reputation.

The number of _validators_ is determined by some weight of the _miner's_ reputation.

At least two _nodes_ are requested to validate a _view_ of the blockchain by a _client_

**Specification**

1. Genesis block is forged by initial _node_
2. Block predetermines the _miner_ and _validator(s)_
3. Next _validator(s)_ are responsible for distributing reward
4. Last _validator(s)_ are responsible for distributing blockchain to _client_
5. Pool is sorted by _weight_
6. Reward is `+1` token, and `+1` reputation
7. Incorrect block yields no reward
8. Misbehaved block yields punishment of `-1` token, and `-1` reputation
<!-- 9. Last _validator_ cannot be next _miner_ -->

#### Algorithms

**Terms**

$$
n_i^t = \text{number of tokens of }i^{th} \text{ node}\\
n_i^r = \text{number of reputation of }i^{th} \text{ node}\\
n_i^s = \text{number of staked tokens of }i^{th} \text{ node}\\
n_i^i = \text{node index in sorted array}\\
\;\\
N_n = \text{total number of nodes}\\
N_t = \text{total number of tokens}\\
N_r = \text{total number of reputation}\\
\;\\
w_i = \text{weight of }i^{th} \text{ node}\\
W_n = \text{total weight of nodes}\\
$$

**Weight**

$$
w_i = n_i^s + n_i^r\\
W_n = \sum_{i=0}^{N_n - 1} w_i
$$

**Validator**

$$
\text{cumulative } v_i = \sum_{x=0}^{i} \frac{n_x^r}{N_r}\\
$$

```
// List of elements sorted by weight
[ele1, ele2, ele3]

// Example list
=> [0.66, 0.16, 0.16]

// Cumulative weight summation
=> c_v = [0.66, 0.82, 1.0]

random_number = generate_random_number(0, 1)
for ele, index in enumerate(c_v):
    if random_number < ele:
        return index // Index of element interested in
```

**Miner**

$$
i = \frac{w_i}{W_n}\\
$$

### Chain

```json
["Block"]
```

### Block

Genesis block will contain all data for initial nodes.
A Node joining the network will create a new block.

```json
{
  "id": 0,
  "hash": "0x0",
  "previous_hash": "genesis",
  "timestamp": 1496314658000,
  "data": [
    {
      "name": "Camper",
      "staked": 0,
      "tokens": 1,
      "reputation": 1
    }
  ],
  "nonce": 0,
  "next_miner": "Camper",
  "next_validators": ["Tom", "Mrugesh", "Quincy"]
}
```

### Node

```json
{
  "name": "Camper",
  "peer_id": "0x1234567890123456789012345678901234567890",
  "staked": 0,
  "tokens": 1,
  "reputation": 1
}
```

### Validator

Same as Node

## Data Handling

Client request streams are not necessarily persisted, as mining could take _too long_. Instead, response from network is always just result of connection.

Always pass `chain` to and from node/blockchain.

### Network Structure

- `node_1` starts
  - Tries to connect to `peers`
    - Fails, because no other peers are available
  - Listens for connections from `clients`
- `node_2` starts
  - Tries to connect to `peers`
    - Succeeds, because `node_1` is available
  - Listens for connections from `clients`

1. Initialise `chain`
2. Wait for at least 3 nodes on the network
3. Intial `node` mines genesis block (pass in `chain`, get `chain` back)
4.

## Security

### Known Holes and Potential Patches

**Hole**: Currently, any node can alter the blockchain code, and still mine a valid block.
**Patch**: Blockchain (Rust code) should be compiled with a checksum, and if the checksum fails, the block is invalid.

**Hole**: If low rep validator is chosen, perhaps its validations should be validated.

## Notes

- Reputation is chance-based
- Story: 25% chance to earn rep on first, 50% chance on second
- Rep still determines how many server-racks
- Maybe buy rack with tokens - spend tokens
- unlock ability to buy rack, by gaining rep

- Camper starts with `10 tokens`, `0 rep`
- Must spend `x tokens` to buy `rep` (rack)

Camper is getting hacked. You notice just in time to save 1 rack (10 tokens).

1. `c1` requests `n1` - `get_node_by_name`

- `n1` is validator: `n1` gets info, sends `c1` response
- `n1` is not validator: `n1` broadcasts `c1` request

2. `n2` receives broadcast from `n1` - `get_node_by_name`

- `n2` is validator: `n2` gets info, sends `n1` response, `n1` sends `c1` response
- `n2` is not validator: `n2` ignores message

3. `c2` requests `n2` - `stake`

- `n2` is miner: `n2` mines block, broadcasts new chain

  - `n1` receives broadcast from `n2` with new chain
    - `n1` is validator: `n1` validates block, sends `n2` response:
      - `n2` receives `true`: `n1` broadcasts validated chain
      - `n2` receives `false`:
    - `n1` is not validator: `n1` ignores message

- `n2` is not miner: `n2` broadcasts `c2` request
