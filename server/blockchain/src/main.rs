#![allow(unused_variables, unused_imports)]
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
use std::time::Duration;
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpSocket,
    select, spawn,
    sync::{broadcast, mpsc},
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
// use request::parse_request_header;

use crate::request::{close_connection, Request};

const DIFFICULTY_PREFIX: &str = "00";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let node_name: String = std::env::var("NODE_NAME").unwrap_or_else(|_| "Camper".to_string());
    pretty_env_logger::init();
    info!("Starting as {:?}", node_name);
    info!("Peer ID: {:?}", PEER_ID);

    // Create and bind socket
    // let addr = "127.0.0.1:3000".parse().unwrap();
    // let socket = TcpSocket::new_v4()?;
    // socket.set_reuseport(true)?;
    // socket.bind(addr)?;
    // let listener = socket.listen(8)?;

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

    let mut swarm = SwarmBuilder::new(transport, behaviour, PEER_ID.clone())
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

    // let (tx, _rx) = broadcast::channel::<String>(8);

    let temp_vec = vec![];
    // Handle multiple sockets
    loop {
        let next_validators = match &swarm.behaviour().chain.get_latest_block() {
            Some(block) => &block.next_validators,
            None => &temp_vec,
        };

        let evt = {
            select! {
              // con = listener.accept(), if next_validators.contains(&node_name) => {
              //   match con {
              //     Ok((mut socket, addr)) => {
              //       let tx = tx.clone();
              //       let mut rx = tx.subscribe();
              //       info!("New connection from {}", addr);
                    // tokio::spawn(async move {
                    //     // Split socket
                    //     let (reader, mut writer) = socket.split();
                    //     // Create bufReader and request buffer
                    //     let mut reader = BufReader::new(reader);
                    //     let mut line = String::new();
                    //     // Loop through all async expressions
                    //     loop {
                    //         select! {
                    //           req = reader.read_line(&mut line) => {
                    //             if req.unwrap() == 0 {
                    //               break;
                    //             }
                    //             let msg = line.clone();
                    //             println!("{}", msg);
                    //             tx.send((msg, addr)).unwrap();
                    //             line.clear();
                    //           }
                    //           req = rx.recv() => {
                    //             let (msg, other_addr) = req.unwrap();
                    //             println!("{}", other_addr);
                    //             if other_addr != addr {
                    //               writer.write_all(msg.as_bytes()).await.unwrap();
                    //             }
                    //           }
                    //         }
                    //     }
                    // });
              //       None
              //     },
              //     Err(e) => {
              //       error!("{}", e);
              //       // Some(EventType::Request())
              //       None
              //     }
              //   }
              // },
            // req = stream.read(&mut msg) => {
              // req = listener.accept() => {
              //   match req {
              //     // Ok(_) => Some(EventType::Request(msg)),
              //     Ok((stream, _)) => {
              //       let mut reader = BufReader::new(stream);
              //       let mut msg = vec![0; 1024];
              //           // let mut line = String::new();
              //           // Read entire stream
              //           let n = reader.read(&mut msg).await?;
              //           msg.truncate(n);
              //           // Turn `request` into String
              //           let request_string = String::from_utf8(msg).expect("can convert to string");
              //           // Handle request http header
              //           let request_result = parse_request_header(request_string);
              //           if let Ok(request_body) = request_result {
              //             let response = format!("{}", "HTTP/1.1 200 OK\r\n\r\n");
              //             reader.write(response.as_bytes()).await?;
              //             close_connection(&mut reader).await;
              //             Some(EventType::Request(request_body))
              //           } else {
              //             let response = format!("{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n");
              //             reader.write(response.as_bytes()).await?;
              //             close_connection(&mut reader).await;
              //             None
              //           }
              //         },
              //         Err(e) => {
              //             error!("{}", e);
              //             continue;
              //         }
              //       }
              //   },

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
                EventType::Request(req) =>
                // {
                //   info!("Request: {:?}", req);
                // }
                {
                    match req {
                        Request::Get => {
                            info!("{:?}", req);
                        }
                        Request::GetBlock(_) => {
                            info!("{:?}", req);
                        }
                        Request::GetChain(_) => {
                            info!("{:?}", req);
                        }
                        Request::GetNodesList(_) => {
                            info!("{:?}", req);
                        }
                        Request::GetNodeInfo(_) => {
                            info!("{:?}", req);
                        }
                        Request::PostTask(_) => {
                            info!("{:?}", req);
                        }
                        Request::PostStake(_) => {
                            info!("{:?}", req);
                        }
                    }
                }
            }
        }
    }
}
