use std::{error::Error, vec};

use bytes::Bytes;

use crate::{
    ble::{ble_manager::QaulBleManager, ble_service::QaulBleService},
    rpc::SysRpcReceiver,
};

pub async fn run_ble_connector_loop(mut rpc_receiver: Box<dyn SysRpcReceiver>) {
    let mut ble_service = QaulBleService::new().await.unwrap();
    ble_service.advertise(None).await.unwrap();
    loop {
        tokio::select! {
            sys_ble_msg = async {
                rpc_receiver.recv().await
            } => {
                if sys_ble_msg.is_none() { return }

            }
        };
    }
    ble_service.advertisement_handles = vec![];
}
