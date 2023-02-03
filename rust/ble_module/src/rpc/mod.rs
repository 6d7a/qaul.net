// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Module
//!
//! Listens to incoming RPC messages on the `qaul.sys.ble` channel.
pub mod err;
mod proto_sys {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.sys.ble.rs");
}

use bytes::Bytes;
use state::Storage;
use tokio::sync::mpsc::{self, Receiver, Sender};

use self::err::RpcError;

/// receiver of the mpsc channel: ui ---> ble_module
static EXTERN_RECEIVE: Storage<Receiver<Bytes>> = Storage::new();
/// sender of the mpsc channel: ui ---> ble_module
static EXTERN_SEND: Storage<Sender<Bytes>> = Storage::new();
/// sender handle of the mpsc channel: ble_module ---> ui
static BLE_MODULE_SEND: Storage<Sender<Bytes>> = Storage::new();

/// Initialize RPC module
/// Create the sending and receiving channels and persist them across threads.
/// Return the receiver for the channel ui ---> ble_module
fn init() -> Receiver<Bytes> {
    // create channels
    let (ble_send, ui_rec) = mpsc::channel::<Bytes>(32);
    let (ui_send, ble_rec) = mpsc::channel::<Bytes>(32);

    // save to state
    EXTERN_RECEIVE.set(ui_rec);
    EXTERN_SEND.set(ui_send);
    BLE_MODULE_SEND.set(ble_send);

    // return ble receiver
    ble_rec
}

pub fn listener() -> Result<(), RpcError> {
    Ok(())
}
