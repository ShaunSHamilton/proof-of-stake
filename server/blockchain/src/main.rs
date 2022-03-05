use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{error, info};
use serde_json::Value;
use std::time::Duration;
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncReadExt, BufReader},
    net::TcpListener,
    select, spawn,
    sync::mpsc,
    time::sleep,
};

mod arg;
mod block;
mod chain;
mod mine;
mod node;
mod p2p;
mod request;
mod validator;

use chain::Chain;
use p2p::{
    get_list_peers, handle_create_block, handle_print_chain, handle_print_peers, ChainBehaviour,
    EventType, LocalChainRequest, CHAIN_TOPIC, KEYS, PEER_ID,
};
use request::deserialize;

const DIFFICULTY_PREFIX: &str = "00";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     info!("Server started and connected")
    // }

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

    let mut swarm = SwarmBuilder::new(transport, behaviour, *PEER_ID)
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
              list = listener.accept() => {
                  let request_body = match list {
                    Ok((stream, _)) => {
                      let mut reader = BufReader::new(stream);
                      // let mut line = String::new();
                      let mut msg = vec![0; 1024];
                        // Read entire stream
                      let n = reader.read(&mut msg).await?;
                      msg.truncate(n);
                      msg
                    },
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                  };

                  // TODO: Look into `serde_json::StreamDeserializer`
                  let serde_stream = deserialize(request_body);
                  for val in serde_stream {
                    println!("{:?}", val);
                  }
                  // Skip first 203 bytes, because that is the HTTP headers
                  let request: Request = match serde_json::from_slice(&request_body[203..]) {
                      Ok(request) => request,
                      Err(e) => {
                          error!("{}", e);
                          continue;
                      }
                  };
                  Some(EventType::Request(request))
              },


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
                    cmd => info!("{}", cmd),
                },
                // From Client
                EventType::Request(req) => match req {
                    Request::GetBlock(block) => {
                        unimplemented!();
                    }
                    Request::GetChain(chain) => {
                        unimplemented!();
                    }
                    Request::GetNodesList(nodes) => {
                        unimplemented!();
                    }
                    Request::GetNodeInfo(node) => {
                        unimplemented!();
                    }
                    Request::PostTask(task) => {
                        unimplemented!();
                    }
                    Request::PostStake(stake) => {
                        unimplemented!();
                    }
                },
            }
        }
    }
}
