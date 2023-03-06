use std::error::Error;

use async_std::task::spawn;
use bytes::Bytes;


use crate::{
    ble::{ble_service::{get_device_info, QaulBleService}, self},
    rpc::{
        proto_sys::{ble::Message::*, BleDeviceInfo},
        SysRpcReceiver, utils::send_result_already_running,
    },
};

use super::{proto_sys::BleDirectSend, BleRpc};

pub async fn listen_for_sys_msgs(
    mut rpc_receiver: BleRpc,
    mut ble_service: QaulBleService,
) -> Result<(), Box<dyn Error>> {
    loop {
        let evt = rpc_receiver.recv().await;
        match evt {
            None => {
                info!("Qaul 'sys' message channel closed. Shutting down gracefully.");
                break;
            }
            Some(msg) => {
                debug!("Received 'sys' message: {:#?}", msg);
                if msg.message.is_none() {
                    continue;
                }
                match msg.message.unwrap() {
                    StartRequest(req) => {
                        match ble_service {
                            QaulBleService::Idle(svc) => {
                                let qaul_id = Bytes::from(req.qaul_id);
                                ble_service = svc
                                    .advertise_scan_listen(qaul_id, None)
                                    .await
                            },
                            QaulBleService::Started(_) => {
                                warn!("Received Start Request, but bluetooth service is already running!");
                                send_result_already_running()
                            },
                        }
                        
                    }
                    StopRequest(_) => {
                        
                    }
                    DirectSend(req) => {
                        
                    }
                    InfoRequest(_) => {
                        spawn(async {
                            get_device_info().await.unwrap_or_else(|err| {
                                error!("Error getting device info: {:#?}", &err)
                            })
                        });
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
