use std::{error::Error, pin::Pin};

use async_trait::async_trait;
use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::{
        characteristic_control, service_control, Application, ApplicationHandle, Characteristic,
        CharacteristicControlHandle, CharacteristicRead, CharacteristicWrite,
        CharacteristicWriteMethod, Service, ServiceControlHandle,
    },
    Adapter, AdapterEvent, Address, Session, Uuid,
};
use bytes::Bytes;
use futures::StreamExt;

use crate::rpc::{
    self,
    proto_sys::{self, ble::Message, BleDeviceInfo, BleInfoResponse},
    utils::{send_ble_sys_msg, send_result_already_running},
};

use super::{
    ble_connect::{QaulBleAppEventRx, QaulBleConnect},
    ble_uuids::{MAIN_SERVICE_UUID, MSG_CHAR, MSG_SERVICE_UUID, READ_CHAR},
};

enum QaulBleState {
    Running,
    Idle,
    Error,
}

pub enum QaulBleHandle {
    AdvertisementHandle(AdvertisementHandle),
    AppHandle(ApplicationHandle),
    CharaHandle(CharacteristicControlHandle),
    ServiceHandle(ServiceControlHandle),
}

pub struct QaulBleService {
    pub ble_handles: Vec<QaulBleHandle>,
    pub adapter: Adapter,
    pub session: Session,
    pub device_block_list: Vec<Address>,
    pub state: QaulBleState,
}

impl QaulBleService {
    /// Initialize a new BleService
    /// Gets default Bluetooth adapter and initializes a Bluer session
    pub async fn new() -> Result<QaulBleService, Box<dyn Error>> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        Ok(QaulBleService {
            ble_handles: vec![],
            adapter,
            session,
            device_block_list: vec![],
            state: QaulBleState::Idle,
        })
    }
}

#[async_trait]
impl QaulBleConnect for QaulBleService {
    async fn get_device_info(&mut self) -> Result<(), Box<dyn Error>> {
        let has_multiple_adv_support = self
            .adapter
            .supported_advertising_features()
            .await?
            .unwrap()
            .contains(&bluer::adv::PlatformFeature::HardwareOffload);
        let max_adv_length = self
            .adapter
            .supported_advertising_capabilities()
            .await?
            .map(|caps| caps.max_advertisement_length)
            .unwrap_or(30);
        let this_device = BleDeviceInfo {
            ble_support: true,
            id: format!("{}", self.adapter.address().await?),
            name: self.adapter.name().into(),
            bluetooth_on: self.adapter.is_powered().await?,
            adv_extended: max_adv_length > 31,
            adv_extended_bytes: max_adv_length as u32,
            le_2m: false,                   // TODO: provide actual value
            le_coded: false,                // TODO: provide actual value
            le_audio: false,                // TODO: provide actual value
            le_periodic_adv_support: false, // TODO: provide actual value
            le_multiple_adv_support: has_multiple_adv_support,
            offload_filter_support: false, // TODO: provide actual value
            offload_scan_batching_support: false, // TODO: provide actual value
        };
        let response = BleInfoResponse {
            device: Some(this_device),
        };
        send_ble_sys_msg(Message::InfoResponse(response));
        Ok(())
    }

    async fn advertise_scan_listen(
        &mut self,
        qaul_id: Bytes,
        advert_mode: Option<i16>,
    ) -> Result<(), Box<dyn Error>> {
        if self.state == QaulBleState::Running {
            debug!("Received start request, but BLE service is already running!");
            send_result_already_running();
        }
        Ok(())
    }
}

// impl QaulBleConnect for QaulBleService {
//     /// Starts the advertisement for the qaul Bluetooth service
//     async fn advertise(&mut self, advert_mode: Option<i16>) -> Result<(), Box<dyn Error>> {
//         let main_adv = Advertisement {
//             service_uuids: vec![Uuid::parse_str(MAIN_SERVICE_UUID)?]
//                 .into_iter()
//                 .collect(),
//             tx_power: advert_mode,
//             discoverable: Some(true),
//             local_name: Some("qaul.net".to_string()),
//             ..Default::default()
//         };

//         self.ble_handles.push(QaulBleHandle::AdvertisementHandle(
//             self.adapter.advertise(main_adv).await?,
//         ));

//         debug!(
//             "Advertising qaul main BLE service at UUID {}",
//             MAIN_SERVICE_UUID
//         );

//         Ok(())
//     }

//     /// Sets up qaul's two BLE characteristics "main" and "message"
//     /// and returns their combined event streams
//     async fn start_ble_app(
//         &mut self,
//         qaul_id: &Bytes,
//     ) -> Result<QaulBleAppEventRx, Box<dyn Error>> {
//         let (_, main_service_handle) = service_control();
//         let (main_chara_ctrl, main_chara_handle) = characteristic_control();

//         let main_service = Service {
//             uuid: Uuid::parse_str(MAIN_SERVICE_UUID)?,
//             primary: true,
//             characteristics: vec![Characteristic {
//                 uuid: Uuid::parse_str(READ_CHAR)?,
//                 read: Some(CharacteristicRead {
//                     read: true,
//                     ..Default::default()
//                 }),
//                 control_handle: main_chara_handle,
//                 ..Default::default()
//             }],
//             control_handle: main_service_handle,
//             ..Default::default()
//         };

//         // TODO: set Qaul id

//         let (_, msg_service_handle) = service_control();
//         let (msg_chara_ctrl, msg_chara_handle) = characteristic_control();

//         let msg_service = Service {
//             uuid: Uuid::parse_str(MSG_SERVICE_UUID)?,
//             primary: true,
//             characteristics: vec![Characteristic {
//                 uuid: Uuid::parse_str(MSG_CHAR)?,
//                 write: Some(CharacteristicWrite {
//                     write: true,
//                     write_without_response: true,
//                     method: CharacteristicWriteMethod::Io,
//                     ..Default::default()
//                 }),
//                 control_handle: msg_chara_handle,
//                 ..Default::default()
//             }],
//             control_handle: msg_service_handle,
//             ..Default::default()
//         };

//         let app = Application {
//             services: vec![main_service, msg_service],
//             ..Default::default()
//         };

//         self.ble_handles.push(QaulBleHandle::AppHandle(
//             self.adapter.serve_gatt_application(app).await?,
//         ));

//         Ok(QaulBleAppEventRx {
//             msg_chara_events: msg_chara_ctrl,
//             main_chara_events: main_chara_ctrl,
//         })
//     }

//     async fn scan(
//         &mut self,
//     ) -> Result<Pin<Box<dyn futures::Stream<Item = Address>>>, Box<dyn Error>> {
//         let block_list = self.device_block_list.clone();
//         Ok(self
//             .adapter
//             .discover_devices_with_changes()
//             .await?
//             .filter_map(move |evt| match evt {
//                 AdapterEvent::DeviceAdded(device) => {
//                     if block_list.contains(&device) {
//                         std::future::ready(None)
//                     } else {
//                         std::future::ready(Some(device))
//                     }
//                 }
//                 _ => std::future::ready(None),
//             })
//             .boxed())
//     }

//     fn close(&self) {
//         self.ble_handles = vec![]
//     }
// }
