use std::error::Error;

use bytes::Bytes;
use prost::Message;

use crate::{
    ble::ble_connect::QaulBleConnect,
    rpc::{proto_sys::ble::Message::*, SysRpcReceiver},
};

pub async fn listen_for_sys_msgs(
    mut rpc_receiver: Box<dyn SysRpcReceiver>,
    mut ble_service: Box<dyn QaulBleConnect>,
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
                    InfoRequest(req) => ble_service.get_device_info().await?,
                    StartRequest(req) => {
                        let qaul_id = Bytes::from(req.qaul_id);
                        ble_service.advertise_scan_listen(qaul_id, None).await?
                    }
                    StopRequest(req) => ble_service.close(),
                    DirectSend(req) => ble_service.send_directly().await?,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
