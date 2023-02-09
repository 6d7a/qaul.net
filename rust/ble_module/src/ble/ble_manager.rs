use std::error::Error;

use async_trait::async_trait;
use bluer::gatt::local::CharacteristicControl;
use bytes::Bytes;

pub struct QaulBleAppEventRx {
    pub main_chara_events: CharacteristicControl,
    pub msg_chara_events: CharacteristicControl,
}

#[async_trait]
pub trait QaulBleManager {
    async fn advertise(&mut self, advert_mode: Option<i16>) -> Result<(), Box<dyn Error>>;
    async fn start_ble_app(&mut self, qaul_id: &Bytes)
        -> Result<QaulBleAppEventRx, Box<dyn Error>>;
}
