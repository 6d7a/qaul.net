use std::error::Error;

use bytes::Bytes;
use tokio::sync::mpsc::{channel, Sender};

use crate::{
    ble::ble_connect::QaulBleConnect,
    rpc::{proto_sys::ble::Message::*, SysRpcReceiver},
};

pub async fn listen_for_sys_msgs(
    mut rpc_receiver: Box<dyn SysRpcReceiver>,
    mut ble_service: Box<dyn QaulBleConnect>,
) -> Result<(), Box<dyn Error>> {
    let mut stop_sender: Option<Sender<bool>> = None;
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
                    InfoRequest(_) => ble_service.get_device_info().await?,
                    StartRequest(req) => {
                        let (tx, rx) = channel::<bool>(1);
                        stop_sender = Some(tx);
                        let qaul_id = Bytes::from(req.qaul_id);
                        ble_service.advertise_scan_listen(rx, qaul_id, None).await?
                    }
                    StopRequest(_) => {
                        if let Some(tx) = stop_sender.clone() {
                            let _ = tx.send(true);
                        }
                    }
                    DirectSend(req) => ble_service.send_directly().await?,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
