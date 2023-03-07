use std::error::Error;

use async_std::{channel::Sender, prelude::*};
use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::{local::*, CharacteristicReader},
    Adapter, AdapterEvent, Address, Device, Session,
};
use bytes::Bytes;
use futures::FutureExt;
use futures_concurrency::stream::Merge;

use crate::ble::ble_uuids::main_service_uuid;
use crate::ble::ble_uuids::msg_char;
use crate::ble::ble_uuids::msg_service_uuid;
use crate::ble::ble_uuids::read_char;
use crate::{
    ble::utils::mac_to_string,
    rpc::{proto_sys::*, utils::*},
};

pub enum QaulBleService {
    Idle(IdleBleService),
    Started(StartedBleService),
}

enum QaulBleHandle {
    AdvertisementHandle(AdvertisementHandle),
    AppHandle(ApplicationHandle),
    CharaHandle(CharacteristicControlHandle),
    ServiceHandle(ServiceControlHandle),
}

pub struct StartedBleService {
    ble_handles: Vec<QaulBleHandle>,
    adapter: Adapter,
    session: Session,
    device_block_list: Vec<Address>,
    stop_handle: Sender<bool>,
}

pub struct IdleBleService {
    ble_handles: Vec<QaulBleHandle>,
    adapter: Adapter,
    session: Session,
    device_block_list: Vec<Address>,
}

enum BleMainLoopEvent {
    Stop,
    MessageReceived((Vec<u8>, Address)),
    MainCharEvent(CharacteristicControlEvent),
    MsgCharEvent(CharacteristicControlEvent),
    DeviceDiscovered(Device),
}

impl IdleBleService {
    /// Initialize a new BleService
    /// Gets default Bluetooth adapter and initializes a Bluer session
    pub async fn new() -> Result<QaulBleService, Box<dyn Error>> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        Ok(QaulBleService::Idle(IdleBleService {
            ble_handles: vec![],
            adapter,
            session,
            device_block_list: vec![],
        }))
    }
}

impl IdleBleService {
    pub async fn advertise_scan_listen(
        mut self,
        qaul_id: Bytes,
        advert_mode: Option<i16>,
    ) -> QaulBleService {
        // ==================================================================================
        // ------------------------- SET UP ADVERTISEMENT -----------------------------------
        // ==================================================================================

        let advertisement = Advertisement {
            service_uuids: vec![main_service_uuid()].into_iter().collect(),
            tx_power: advert_mode,
            discoverable: Some(true),
            local_name: Some("qaul.net".to_string()),
            ..Default::default()
        };

        match self.adapter.advertise(advertisement).await {
            Ok(handle) => self
                .ble_handles
                .push(QaulBleHandle::AdvertisementHandle(handle)),
            Err(err) => {
                error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        debug!(
            "Advertising qaul main BLE service at UUID {}",
            main_service_uuid()
        );

        // ==================================================================================
        // ------------------------- SET UP APPLICATION -------------------------------------
        // ==================================================================================

        let (_, main_service_handle) = service_control();
        let (mut main_chara_ctrl, main_chara_handle) = characteristic_control();

        let main_service = Service {
            uuid: main_service_uuid(),
            primary: true,
            characteristics: vec![Characteristic {
                uuid: read_char(),
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        let value = qaul_id.clone();
                        async move {
                            debug!("Read request {:?} with value {:x?}", &req, &value);
                            Ok(value.to_vec())
                        }
                        .boxed()
                    }),
                    ..Default::default()
                }),
                control_handle: main_chara_handle,
                ..Default::default()
            }],
            control_handle: main_service_handle,
            ..Default::default()
        };

        let (_, msg_service_handle) = service_control();
        let (mut msg_chara_ctrl, msg_chara_handle) = characteristic_control();

        let msg_service = Service {
            uuid: msg_service_uuid(),
            primary: true,
            characteristics: vec![Characteristic {
                uuid: msg_char(),
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),
                control_handle: msg_chara_handle,
                ..Default::default()
            }],
            control_handle: msg_service_handle,
            ..Default::default()
        };

        let app = Application {
            services: vec![main_service, msg_service],
            ..Default::default()
        };

        match self.adapter.serve_gatt_application(app).await {
            Ok(handle) => self.ble_handles.push(QaulBleHandle::AppHandle(handle)),
            Err(err) => {
                error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        // ==================================================================================
        // --------------------------------- SCAN -------------------------------------------
        // ==================================================================================

        let device_stream = match self.adapter.discover_devices().await {
            Ok(addr_stream) => addr_stream.filter_map(|evt| match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    if self.device_block_list.contains(&addr) {
                        return None;
                    }
                    match self.adapter.device(addr) {
                        Ok(device) => Some(BleMainLoopEvent::DeviceDiscovered(device)),
                        Err(_) => None,
                    }
                }
                _ => None,
            }),
            Err(err) => {
                error!("{:#?}", err);
                return QaulBleService::Idle(self);
            }
        };

        // ==================================================================================
        // --------------------------------- MAIN BLE LOOP ----------------------------------
        // ==================================================================================

        let (tx, rx) = async_std::channel::bounded::<bool>(1);

        let stop_stream = rx.map(|_| BleMainLoopEvent::Stop);
        let main_evt_stream = main_chara_ctrl.map(BleMainLoopEvent::MainCharEvent);
        let msg_evt_stream = msg_chara_ctrl.map(BleMainLoopEvent::MsgCharEvent);

        let mut merged_ble_streams =
            (stop_stream, main_evt_stream, msg_evt_stream, device_stream).merge();

        debug!("Set up advertisement and scan filter, entering BLE main loop.");
        send_start_successful();

        let mut msg_receivers: Vec<CharacteristicReader> = vec![];

        while let Some(evt) = merged_ble_streams.next().await {
            match evt {
                BleMainLoopEvent::Stop => {
                    info!("Received stop signal, stopping advertising, scanning, and listening.");
                    break;
                }
                BleMainLoopEvent::MessageReceived(e) => todo!(),
                BleMainLoopEvent::MainCharEvent(e) => todo!(),
                BleMainLoopEvent::MsgCharEvent(e) => todo!(),
                BleMainLoopEvent::DeviceDiscovered(device) => {
                    let stringified_addr = mac_to_string(&device.address());
                    let uuids = device.uuids().await.ok().flatten().unwrap_or_default();
                    trace!(
                        "Discovered device {} with service UUIDs {:?}",
                        &stringified_addr,
                        &uuids
                    );

                    if !uuids.contains(&main_service_uuid()) {
                        continue;
                    }
                    debug!("Discovered qaul bluetooth device {}", &stringified_addr);

                    if !device.is_connected().await.unwrap_or(false) {
                        device.connect().await;
                        info!("Connected to device {}", &stringified_addr);
                    }

                    for service in device.services().await.unwrap() {
                        let service_uuid = service.uuid().await.unwrap_or_default();
                        if service_uuid != main_service_uuid() {
                            continue;
                        }
                        for char in service.characteristics().await.unwrap() {
                            let flags = char.flags().await.unwrap();
                            if flags.notify || flags.indicate {
                                msg_receivers.push(char.notify_io().await.unwrap());
                                info!(
                                    "Setting up notification for characteristic {} of device {}",
                                    char.uuid().await.unwrap(),
                                    &stringified_addr
                                );
                            } else if flags.read && char.uuid().await.unwrap() == read_char() {
                                let remote_qaul_id = char.read().await.unwrap();
                                let rssi = device.rssi().await.ok().flatten().unwrap_or(999) as i32;
                                send_device_found(remote_qaul_id, rssi)
                            }
                        }
                    }
                }
            }
        }

        // loop {
        //     tokio::select! {
        //         cmd = rpc_receiver.recv() => {
        //             if cmd.is_none() {
        //                 // Stop advertising, scanning, and listening and return
        //                 break;
        //             } else {

        //             }
        //         },
        //         Some((Ok(data), from)) = async {
        //             let mut futures = FuturesUnordered::from_iter(
        //                 msg_receivers.iter()
        //                     .map(|n| {
        //                         n.recv().map(|msg| (msg, n.device_address()))
        //                     })
        //             );
        //             futures.next().await
        //         }, if !msg_receivers.is_empty() => {
        //             info!("Received {} bytes of data from {}", data.len(), mac_to_string(&from));
        //             send_direct_received(from.0.to_vec(), data)
        //         },
        //         Some(_main_event) = main_chara_ctrl.next() => {
        //             // TODO: should something be reported to the UI?
        //         },
        //         Some(msg_event) = msg_chara_ctrl.next() => {
        //             match msg_event {
        //                 CharacteristicControlEvent::Write(write) => {
        //                     msg_receivers.push(write.accept()?)
        //                 },
        //                 // TODO: should the notifiy handle do something?
        //                 CharacteristicControlEvent::Notify(_) => (),
        //             }
        //         },
        //         Some(addr) = device_stream.next() => {
        //             let stringified_addr = mac_to_string(&addr);
        //             let device = self.adapter.device(addr)?;
        //             let uuids = device.uuids().await?.unwrap_or_default();
        //             trace!("Discovered device {} with service UUIDs {:?}", &stringified_addr, &uuids);

        //             if !uuids.contains(&main_service_uuid) { continue; }
        //             debug!("Discovered qaul bluetooth device {}", &stringified_addr);

        //             if !device.is_connected().await? {
        //                 device.connect().await?;
        //                 info!("Connected to device {}", &stringified_addr);
        //             }

        //             for service in device.services().await? {
        //                 let service_uuid = service.uuid().await?;
        //                 if service_uuid != main_service_uuid { continue; }
        //                 for char in service.characteristics().await? {
        //                     let flags = char.flags().await?;
        //                     if flags.notify || flags.indicate {
        //                         msg_receivers.push(char.notify_io().await?);
        //                         info!(
        //                             "Setting up notification for characteristic {} of device {}",
        //                             char.uuid().await?,
        //                             &stringified_addr);
        //                     } else if flags.read && char.uuid().await? == read_char_uuid {
        //                         let remote_qaul_id = char.read().await?;
        //                         let rssi = device.rssi().await?.unwrap_or(999) as i32;
        //                         send_device_found(remote_qaul_id, rssi)
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        QaulBleService::Started(StartedBleService {
            ble_handles: self.ble_handles,
            adapter: self.adapter,
            session: self.session,
            device_block_list: self.device_block_list,
            stop_handle: tx,
        })
    }
}

pub async fn get_device_info() -> Result<(), Box<dyn Error>> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    let has_multiple_adv_support = adapter
        .supported_advertising_features()
        .await?
        .unwrap()
        .contains(&bluer::adv::PlatformFeature::HardwareOffload);
    let max_adv_length = adapter
        .supported_advertising_capabilities()
        .await?
        .map(|caps| caps.max_advertisement_length)
        .unwrap_or(30);
    let this_device = BleDeviceInfo {
        ble_support: true,
        id: format!("{}", adapter.address().await?),
        name: adapter.name().into(),
        bluetooth_on: adapter.is_powered().await?,
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
    send_ble_sys_msg(ble::Message::InfoResponse(response));
    Ok(())
}
