use serde::Deserialize;
use serde_json;

pub fn parse_request_header(request_string: String) -> Result<Request, ()> {
    println!("{}", request_string);
    let body = request_string
        .split("\r\n\r\n")
        .next()
        .expect("body exists");
    if let Some(line) = request_string.lines().next() {
        match line {
            "GET / HTTP/1.1" => Ok(Request::Get),
            "GET /node-info HTTP/1.1" => {
                // Split request into header and body
                let request_body = deserialize(body);
                Ok(request_body)
            }
            "GET /block HTTP/1.1" => {
                let request_body = deserialize(body);
                Ok(request_body)
            }
            "GET /chain HTTP/1.1" => {
                let request_body = deserialize(body);
                Ok(request_body)
            }
            "POST /stakes HTTP/1.1" => {
                let request_body = deserialize(body);
                Ok(request_body)
            }
            "POST /task HTTP/1.1" => {
                let request_body = deserialize(body);
                Ok(request_body)
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

pub fn deserialize<'a, T>(request_string: &'a str) -> T
where
    T: Deserialize<'a>,
{
    let request_body: T = serde_json::from_str(request_string).expect("can deserialize");
    request_body
}

#[derive(Deserialize, Debug)]
pub enum Request {
    Get,
    GetNodeInfo,
    GetBlock,
    GetChain,
    GetNodesList,
    PostStake,
    PostTask,
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
