use std::error::Error;

use async_trait::async_trait;
use bluer::{
    adv::Advertisement, adv::AdvertisementHandle, gatt::local::CharacteristicControlHandle,
    Adapter, Session, Uuid,
};
use bytes::Bytes;

use super::{
    ble_manager::QaulBleManager,
    ble_uuids::{MSG_SERVICE_UUID, SERVICE_UUID},
};

pub struct QaulBleService {
    pub advertisement_handles: Vec<AdvertisementHandle>,
    pub adapter: Adapter,
    pub session: Session,
    pub msg_chara_handle: Option<CharacteristicControlHandle>,
    pub main_chara_handle: Option<CharacteristicControlHandle>,
}

impl QaulBleService {
    /// Initialize a new BleService
    /// Gets default Bluetooth adapter and initializes a Bluer session
    pub async fn new() -> Result<QaulBleService, Box<dyn Error>> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        Ok(QaulBleService {
            advertisement_handles: vec![],
            adapter,
            session,
            msg_chara_handle: None,
            main_chara_handle: None,
        })
    }
}

#[async_trait]
impl QaulBleManager for QaulBleService {
    /// Starts the advertisement for the qaul Bluetooth service
    async fn advertise(&mut self, advert_mode: Option<i16>) -> Result<(), Box<dyn Error>> {
        let main_adv = Advertisement {
            service_uuids: vec![Uuid::parse_str(SERVICE_UUID)?].into_iter().collect(),
            tx_power: advert_mode,
            discoverable: Some(true),
            local_name: Some("qaul.net main".to_string()),
            ..Default::default()
        };

        self.advertisement_handles
            .push(self.adapter.advertise(main_adv).await?);

        let msg_adv = Advertisement {
            service_uuids: vec![Uuid::parse_str(MSG_SERVICE_UUID)?]
                .into_iter()
                .collect(),
            tx_power: advert_mode,
            discoverable: Some(true),
            local_name: Some("qaul.net messages".to_string()),
            ..Default::default()
        };

        self.advertisement_handles
            .push(self.adapter.advertise(msg_adv).await?);

        Ok(())
    }
}
