use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait QaulBleManager {
    async fn advertise(&mut self, advert_mode: Option<i16>) -> Result<(), Box<dyn Error>>;
}
