use std::{error::Error, vec};

use bytes::Bytes;

use crate::{
    ble::{self, ble_manager::BleManager, ble_service::BleService},
    rpc::SysRpcReceiver,
};

pub async fn run_ble_connector_loop(mut rpc_receiver: Box<dyn SysRpcReceiver>) {
    let mut ble_service = BleService::new().await.unwrap();
    let qaul_id = Bytes::from(&b"Hello world"[..]);
    ble_service.advertise(&qaul_id, None).await.unwrap();
    loop {
        tokio::select! {
            sys_ble_msg = async {
                rpc_receiver.recv().await
            } => {
                if sys_ble_msg.is_none() { return }

            }
        };
    }
    ble_service.advertising_handles = vec![];
}
