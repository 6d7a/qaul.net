use std::error::Error;

use crate::rpc::SysRpcReceiver;

pub async fn run_ble_connector_loop(mut rpc_receiver: Box<dyn SysRpcReceiver>) {
    loop {
        tokio::select! {
            sys_ble_msg = async {
                rpc_receiver.recv().await
            } => {
                if sys_ble_msg.is_none() { return }

            }
        };
    }
}
