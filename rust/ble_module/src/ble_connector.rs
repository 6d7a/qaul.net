use std::{error::Error, vec};

use bluer::{gatt::local::CharacteristicControlEvent, AdapterEvent};
use bytes::Bytes;
use futures::{pin_mut, StreamExt};
use prost::encoding::BytesAdapter;

use crate::{
    ble::{
        self,
        ble_manager::{QaulBleAppEventRx, QaulBleManager},
        ble_service::QaulBleService,
    },
    rpc::SysRpcReceiver,
};

pub async fn run_ble_connector_loop(mut rpc_receiver: Box<dyn SysRpcReceiver>) {
    let mut ble_service = QaulBleService::new().await.unwrap();
    ble_service.advertise(None).await.unwrap();
    let qaul_id = Bytes::from(&b"hello world"[..]);
    let mut event_rx = ble_service.start_ble_app(&qaul_id).await.unwrap();
    let mut adapter_events = ble_service.scan().await.unwrap();
    loop {
        tokio::select! {
            sys_ble_msg = async {
                rpc_receiver.recv().await
            } => {
                if sys_ble_msg.is_none() { return }
            }
            Some(evt) = event_rx.msg_chara_events.next() => {
                match evt {
                    CharacteristicControlEvent::Write(req) => {
                        println!("Accepting write event with MTU {} from {}", req.mtu(), req.device_address());
                    },
                    CharacteristicControlEvent::Notify(notifier) => {
                        println!("Accepting notify request event with MTU {} from {}", notifier.mtu(), notifier.device_address());
                    }
                }
            }
            Some(address) = adapter_events.next() => {
                    println!("Device added: {:?}", address);
            }
        }
    }
    ble_service.ble_handles = vec![];
}
