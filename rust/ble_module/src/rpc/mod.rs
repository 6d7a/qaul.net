// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Module
//!
//! Listens to incoming RPC messages on the `qaul.sys.ble` channel.
pub mod err;

// TODO: remove local import, import instead from libqaul
mod proto_sys {
    include!("./qaul.sys.ble.rs");
}

use bytes::Bytes;
use prost::Message;
use state::Storage;
use tokio::sync::mpsc;

use proto_sys::Ble;

/// receiver of the mpsc channel: ui ---> ble_module
static EXTERN_RECEIVE: Storage<crossbeam_channel::Receiver<Bytes>> = Storage::new();
/// sender of the mpsc channel: ui ---> ble_module
static EXTERN_SEND: Storage<tokio::sync::mpsc::Sender<Bytes>> = Storage::new();
/// sender handle of the mpsc channel: ble_module ---> ui
static BLE_MODULE_SEND: Storage<crossbeam_channel::Sender<Bytes>> = Storage::new();

/// Initialize RPC module
/// Create the sending and receiving channels and persist them across threads.
/// Return the receiver for the channel ui ---> ble_module
pub fn init() -> tokio::sync::mpsc::Receiver<Bytes> {
    // create channels
    let (ble_send, ui_rec) = crossbeam_channel::bounded::<Bytes>(32);
    let (ui_send, ble_rec) = mpsc::channel::<Bytes>(32);

    // save to state
    EXTERN_RECEIVE.set(ui_rec);
    EXTERN_SEND.set(ui_send);
    BLE_MODULE_SEND.set(ble_send);

    // return ble receiver
    ble_rec
}

/// send rpc message ui ---> ble_module
pub fn send_to_ble_module(binary_message: Bytes) {
    if let Err(err) = EXTERN_SEND.get().try_send(binary_message) {
        error!("{:?}", err);
    }
}

/// check whether there are new messages in
/// the receiving rpc channel ble_module ---> ui
pub fn receive_from_ble_module() -> Result<Bytes, crossbeam_channel::TryRecvError> {
    EXTERN_RECEIVE.get().try_recv()
}

/// get the number of messages in the receiving cue
pub fn queue_length_ble_to_ui() -> usize {
    BLE_MODULE_SEND.get().len()
}

/// send rpc message ble_module ---> ui
pub fn send_to_ui(binary_message: Bytes) {
    if let Err(err) = BLE_MODULE_SEND.get().try_send(binary_message) {
        error!("{:?}", err);
    }
}

/// Process received binary protobuf encoded RPC message
///
/// This function will decode the message from the binary
/// protobuf format to a rust struct and return it
pub fn process_received_message(data: Bytes) -> Option<Ble> {
    match Ble::decode(data.clone()) {
        Ok(ble) => Some(ble),
        Err(err) => {
            error!("{:#?}", err);
            None
        }
    }
}
