#[derive(Debug, Clone)]
pub enum RpcError {
    Unknown { reason: Option<String> },
}
