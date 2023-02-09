use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait BleManager {
    async fn advertise(&mut self, advertMode: Option<i16>) -> Result<(), Box<dyn Error>>;
}
