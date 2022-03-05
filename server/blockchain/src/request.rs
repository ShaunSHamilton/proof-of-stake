use http::Request;
use serde::de;
use serde::Deserialize;

pub fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
where
    for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}

// #[derive(Deserialize, Debug)]
// pub enum Request {
//     GetNodeInfo(GetNodeInfo),
//     GetBlock(GetBlock),
//     GetChain(GetChain),
//     GetNodesList(GetNodesList),
//     PostStake(PostStake),
//     PostTask(PostTask),
// }

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
