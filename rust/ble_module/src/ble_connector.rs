use crate::{
    ble::ble_manager::QaulBleConnect,
    rpc::{proto_sys::ble::Message::*, SysRpcReceiver},
};

pub async fn run_ble_connector_loop(
    mut rpc_receiver: Box<dyn SysRpcReceiver>,
    mut ble_service: Box<dyn QaulBleConnect>,
) {
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
                    InfoRequest(req) => todo!(),
                    StartRequest(req) => todo!(),
                    StopRequest(req) => todo!(),
                    DirectSend(req) => todo!(),
                    _ => (),
                }
            }
        }
    }
}
