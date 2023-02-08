use std::error::Error;

use async_trait::async_trait;
use bluer::{adv::Advertisement, adv::AdvertisementHandle, Adapter, Session, Uuid};
use bytes::Bytes;

use super::{
    ble_manager::BleManager,
    ble_uuids::{MSG_SERVICE_UUID, SERVICE_UUID},
};

pub struct BleService {
    pub advertising_handles: Vec<AdvertisementHandle>,
    pub adapter: Adapter,
    pub session: Session,
}

impl BleService {
    /// Initialize a new BleService
    /// Gets default Bluetooth adapter and initializes a Bluer session
    pub async fn new() -> Result<BleService, Box<dyn Error>> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        Ok(BleService {
            advertising_handles: vec![],
            adapter,
            session,
        })
    }
}

#[async_trait]
impl BleManager for BleService {
    /// Starts the advertisement for the qaul Bluetooth service
    async fn advertise(
        &mut self,
        qaul_id: &Bytes,
        advertMode: Option<i16>,
    ) -> Result<(), Box<dyn Error>> {
        let le_advertisement = Advertisement {
            service_uuids: vec![Uuid::parse_str(SERVICE_UUID)?].into_iter().collect(),
            tx_power: advertMode,
            discoverable: Some(true),
            local_name: Some("qaul.net main".to_string()),
            ..Default::default()
        };
        let handle = self.adapter.advertise(le_advertisement).await?;

        self.advertising_handles.push(handle);

        let le_advertisement = Advertisement {
            service_uuids: vec![Uuid::parse_str(MSG_SERVICE_UUID)?]
                .into_iter()
                .collect(),
            tx_power: advertMode,
            discoverable: Some(true),
            local_name: Some("qaul.net messages".to_string()),
            ..Default::default()
        };
        let handle = self.adapter.advertise(le_advertisement).await?;

        self.advertising_handles.push(handle);

        Ok(())
    }
}
