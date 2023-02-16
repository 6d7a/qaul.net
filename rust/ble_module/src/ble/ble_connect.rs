use std::{error::Error, pin::Pin};

use async_trait::async_trait;
use bluer::{gatt::local::CharacteristicControl, Address};
use bytes::Bytes;

pub struct QaulBleAppEventRx {
    pub main_chara_events: CharacteristicControl,
    pub msg_chara_events: CharacteristicControl,
}

#[async_trait]
pub trait QaulBleConnect {
    async fn get_device_info(&mut self) -> Result<(), Box<dyn Error>>;
    async fn advertise_scan_listen(
        &mut self,
        advert_mode: Option<i16>,
    ) -> Result<(), Box<dyn Error>>;
    async fn start_ble_app(&mut self, qaul_id: &Bytes)
        -> Result<QaulBleAppEventRx, Box<dyn Error>>;
    async fn scan(
        &mut self,
    ) -> Result<Pin<Box<dyn futures::Stream<Item = Address>>>, Box<dyn Error>>;
    async fn send_directly(&mut self) -> Result<(), Box<dyn Error>>;
    fn close(&self);
}
