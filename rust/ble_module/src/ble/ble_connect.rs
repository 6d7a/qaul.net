use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait QaulBleConnect {
    async fn get_device_info(&mut self) -> Result<(), Box<dyn Error>>;
    async fn advertise_scan_listen(
        &mut self,
        mut terminator: Receiver<bool>,
        qaul_id: Bytes,
        advert_mode: Option<i16>,
    ) -> Result<(), Box<dyn Error>>;
    async fn send_directly(&mut self) -> Result<(), Box<dyn Error>>;
}
