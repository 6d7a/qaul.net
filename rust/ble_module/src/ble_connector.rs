use std::error::Error;

use crate::{
    ble::{self, ble_manager::QaulBleConnect},
    rpc::{proto_sys::ble::Message::*, SysRpcReceiver},
};

pub async fn run_ble_connector_loop(
    mut rpc_receiver: Box<dyn SysRpcReceiver>,
    mut ble_service: Box<dyn QaulBleConnect>,
) -> Result<(), Box<dyn Error> {
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
                    StartRequest(req) => ble_service.advertise_scan_listen(None).await?,
                    StopRequest(req) => ble_service.close(),
                    DirectSend(req) => ble_service.send_directly().await?,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
