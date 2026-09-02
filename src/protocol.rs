use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Set { key: String, value: String, ttl: Option<u64> },
    Get { key: String },
    Delete { key: String },
    List,
    Clear,
    Status,
    // 扩展功能
    Subscribe { topic: String },
    Publish { topic: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Success { message: String, data: Option<String> },
    Error { code: u16, message: String },
    Data { key: String, value: Option<String> },
    List { keys: Vec<String>, count: usize },
    Status { total_keys: usize, is_empty: bool, connections: usize },
    Notification { topic: String, message: String },
}

pub struct ProtocolParser;

impl ProtocolParser {
    pub fn serialize_request(req: &Request) -> serde_json::Result<String> {
        serde_json::to_string(req)
    }
    pub fn deserialize_request(data: &str) -> serde_json::Result<Request> {
        serde_json::from_str(data)
    }
    pub fn serialize_response(resp: &Response) -> serde_json::Result<String> {
        serde_json::to_string(resp)
    }
    pub fn deserialize_response(data: &str) -> serde_json::Result<Response> {
        serde_json::from_str(data)
    }
}