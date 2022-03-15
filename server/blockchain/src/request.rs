#![allow(dead_code)]
use serde::{de::DeserializeOwned, Deserialize};
use serde_json;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub fn parse_request_header(request_string: String) -> Result<Request, ()> {
    println!("{}", request_string);
    if let Some(line) = request_string.lines().next() {
        match handle_method(line) {
            Methods::Get => {
                let route = get_route(line);
                if let Some(ref params) = get_query_params(line) {
                    match route.as_str() {
                        "/node-info" => {
                            Ok(Request::GetNodeInfo(deserialize::<GetNodeInfo>(params)))
                        }
                        "/block" => Ok(Request::GetBlock(deserialize::<GetBlock>(params))),
                        "/chain" => Ok(Request::GetChain(deserialize::<GetChain>(params))),
                        _ => Err(()),
                    }
                } else {
                    match route.as_str() {
                        "/" => Ok(Request::Get),
                        _ => Err(()),
                    }
                }
            }
            Methods::Post => {
                let route = get_route(line);
                // let _params = get_query_params(line);
                if let Some(ref body) = get_body(request_string) {
                    match route.as_str() {
                        "/stake" => Ok(Request::PostStake(deserialize::<PostStake>(body))),
                        "/task" => Ok(Request::PostTask(deserialize::<PostTask>(body))),
                        _ => Err(()),
                    }
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

pub fn handle_method(s: &str) -> Methods {
    let mut iter = s.split_whitespace();
    match iter.next() {
        Some("GET") => Methods::Get,
        Some("POST") => Methods::Post,
        _ => Methods::Unknown, // Maybe panic?
    }
}

pub fn get_route(s: &str) -> String {
    let mut iter = s.split_whitespace();
    if let Some(route_with_params) = iter.nth(1) {
        route_with_params.split('?').next().unwrap().to_string()
    } else {
        iter.next().unwrap().to_string()
    }
}

pub fn get_query_params<T>(s: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    if let Some(i) = s.find('?') {
        let param_string = s[(i + 1)..]
            .split(' ')
            .next()
            .expect("can remove request version");
        let json_string = create_json_from_query(param_string);
        match serde_json::from_str(json_string.as_str()) {
            Ok(serd) => Some(serd),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn create_json_from_query<'a>(query: &'a str) -> String {
    let kv = query
        .split('&')
        .map(|q| {
            let mut x = q.split('=');
            let k = x.next().expect("can parse key");
            let v = x.next().expect("can parse value");
            if let Ok(v) = v.parse::<i32>() {
                format!("\"{k}\": {v}")
            } else {
                format!("\"{k}\": \"{v}\"")
            }
        })
        .reduce(|acc, curr| format!("{acc}, {curr}"))
        .expect("can join iter");
    format!("{{ {kv} }}")
}

pub fn get_body<'a>(s: String) -> Option<String> {
    if let Some(str_ref) = s.split("\r\n\r\n").nth(1) {
        Some(String::from(str_ref))
    } else {
        None
    }
}

pub fn deserialize<'a, T>(body: &'a String) -> T
where
    T: Deserialize<'a> + std::fmt::Debug,
{
    let request_body: T = serde_json::from_str(body).expect("can deserialize");
    request_body
}

#[allow(dead_code)]
pub async fn close_connection(stream: &mut BufReader<TcpStream>) {
    stream
        .get_mut()
        .shutdown()
        .await
        .expect("can shutdown connection");
}

pub enum Methods {
    Get,
    Post,
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum Request {
    Get,
    GetNodeInfo(GetNodeInfo),
    GetBlock(GetBlock),
    GetChain(GetChain),
    GetNodesList(GetNodesList),
    PostStake(PostStake),
    PostTask(PostTask),
}

#[derive(Deserialize, Debug)]
pub struct GetNodeInfo {
    pub stake: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetBlock {
    pub index: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetChain {
    pub length: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetNodesList {
    pub length: usize,
}

#[derive(Deserialize, Debug)]
pub struct PostTask {
    pub task: String,
}

#[derive(Deserialize, Debug)]
pub struct PostStake {
    pub stake: usize,
}
