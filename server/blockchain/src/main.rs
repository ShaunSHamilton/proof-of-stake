use chrono::prelude::*;
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Duration;
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::mpsc,
    time::sleep,
};

const DIFFICULTY_PREFIX: &str = "00";

mod node;
use node::Node;
mod p2p;
use p2p::{
    get_list_peers, handle_create_block, handle_print_chain, handle_print_peers, ChainBehaviour,
    EventType, LocalChainRequest, CHAIN_TOPIC, KEYS, PEER_ID,
};
mod validator;
use validator::Validator;

pub struct Chain {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
    // pub node: Node,
    // pub validators: Vec<Validator>,
}

fn hash_to_bin_rep(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

impl Chain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            // node: Node::new("Camper".to_string()),
            // validators:
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("could not add block - invalid!");
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !hash_to_bin_rep(&hex::decode(&block.hash).expect("can decode from hex"))
            .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hex::encode(calc_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exit");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    // Always choose longest valid chain
    fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("local and remote chains are both invalid!!!");
        }
    }
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);
        Self {
            id,
            hash,
            // Should this be re-computed after mining?
            timestamp: now.timestamp(),
            previous_hash,
            data,
            nonce,
        }
    }
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");
    let mut nonce = 0;

    loop {
        if nonce % 100_000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = calc_hash(id, timestamp, previous_hash, data, nonce);
        let bin_hash = hash_to_bin_rep(&hash);
        if bin_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mined! nonce: {}, hash: {}, bin hash: {}",
                nonce,
                hex::encode(&hash),
                bin_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

fn calc_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
      "id": id,
      "previous_hash": previous_hash,
      "data": data,
      "timestamp": timestamp,
      "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Peer id: {}", PEER_ID.clone());
    let (response_sender, mut response_receiver) = mpsc::unbounded_channel();
    let (init_sender, mut init_receiver) = mpsc::unbounded_channel();

    let auth_keys = Keypair::<X25519>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");

    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let behaviour = ChainBehaviour::new(Chain::new(), response_sender, init_sender.clone()).await;

    let mut swarm = SwarmBuilder::new(transport, behaviour, *p2p::PEER_ID)
        .executor(Box::new(|fut| {
            spawn(fut);
        }))
        .build();

    let mut stdin = BufReader::new(stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0".parse().expect("can get local socket"),
    )
    .expect("swarm can be started");

    spawn(async move {
        sleep(Duration::from_secs(1)).await;
        info!("sending init event");
        init_sender.send(true).expect("can send init event");
    });

    loop {
        let evt = {
            select! {
              line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
              response = response_receiver.recv() => {
                Some(EventType::LocalChainResponse(response.expect("response exists")))
              },
              _init = init_receiver.recv() => {
                Some(EventType::Init)
              }
              event = swarm.select_next_some() => {
                info!("Unhandled Swarm Event: {:?}", event);
                None
              },
            }
        };

        if let Some(event) = evt {
            match event {
                EventType::Init => {
                    let peers = get_list_peers(&swarm);
                    swarm.behaviour_mut().chain.genesis();

                    info!("Connected Nodes: {}", peers.len());
                    if !peers.is_empty() {
                        let req = LocalChainRequest {
                            from_peer_id: peers
                                .iter()
                                .last()
                                .expect("at least one peer")
                                .to_string(),
                        };

                        let json = serde_json::to_string(&req).expect("can stringify json request");
                        swarm
                            .behaviour_mut()
                            .floodsub
                            .publish(CHAIN_TOPIC.clone(), json.as_bytes());
                    }
                }
                EventType::LocalChainResponse(resp) => {
                    let json = serde_json::to_string(&resp).expect("can stringify json response");
                    swarm
                        .behaviour_mut()
                        .floodsub
                        .publish(CHAIN_TOPIC.clone(), json.as_bytes());
                }
                EventType::Input(line) => match line.as_str() {
                    "ls p" => handle_print_peers(&swarm),
                    cmd if cmd.starts_with("ls c") => handle_print_chain(&swarm),
                    cmd if cmd.starts_with("create b") => handle_create_block(cmd, &mut swarm),
                    _ => error!("unknown command"),
                },
            }
        }
    }
}
