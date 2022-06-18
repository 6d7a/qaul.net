// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Run Libqaul in an own Thread
//! 
//! Start libqaul in an own thread and communicate
//! via a sync mpsc queues into and from this thread.
//! 
//! This setup is to decouple the GUI thread from 
//! libqaul. 
//! The communication will happen via protobuf rpc messages.

use crossbeam_channel::TryRecvError;
use futures::executor::block_on;
use std::{
    thread,
    path::PathBuf,
};
#[cfg(not(target_os = "macos"))]
use directories::ProjectDirs;

use crate::rpc::Rpc;
use crate::rpc::sys::Sys;

/// C API module
mod c;

/// android module
/// The module only compiled, when the compile target is android.
#[cfg(target_os = "android")]
mod android;

/// start libqaul in an own thread
/// 
/// Provide the location for storage, all data of qaul will be saved there.
pub fn start(storage_path: String) {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start(storage_path).await;
        }
    ));    
}

/// start libqaul on a desktop platform (Linux, Mac, Windows)
/// 
/// It will automatically define the path to the common OS specific
/// configuration and data location.
/// 
/// The locations are:
/// 
/// * Linux: /home/USERNAME/.config/qaul
/// * MacOS container directory: /Users/USERNAME/Library/Containers/net.qaul.qaulApp
/// * Windows: C:\Users\USERNAME\AppData\Roaming\qaul\qaul\config
pub fn start_desktop() {
    let path: PathBuf;

    log::info!("start_desktop");

    // create path for macos
    #[cfg(target_os = "macos")]
    {
        match std::env::current_dir() {
            Ok(dir) => path = dir,
            Err(e) => {
                log::error!("{}", e);
                return;
            },
        }
    }
    // create path on linux and windows
    #[cfg(not(target_os = "macos"))]
    {
        match ProjectDirs::from("net", "qaul", "qaul") {
            Some(proj_dirs) => path = proj_dirs.config_dir().to_path_buf(),
            None => {
                log::error!("Configuration path couldn't be created.");
                return;
            }
        }
    }

    log::info!("configuration path: {:?}", path.as_path());

    // check if path already exists
    if !path.exists() {
        log::info!("create path");

        // create path if it does not exist
        std::fs::create_dir_all(path.as_path()).unwrap();
    }

    log::info!("start libqaul");

    // start the library with the path
    match path.to_str() {
        Some(path_str) => {
            start(path_str.to_string());
        },
        None => log::error!("no path found"),
    }
}

/// start libqaul for android
/// here for debugging and testing
/// 
/// Hand over the path on the file system
/// where the app is allowed to store data.
pub fn start_android(storage_path: String) {
    // Spawn new thread
    thread::spawn(move|| block_on(
        async move {
            // start libqaul
            crate::start_android(storage_path).await;
        }
    ));
}

/// Check if libqaul finished initializing
/// 
/// The initialization of libqaul can take several seconds.
/// If you send any message before it finished initializing, libqaul will crash.
/// Wait therefore until this function returns true before sending anything to libqaul.
pub fn initialization_finished() -> bool {
    if let Some(_) = crate::INITIALIZED.try_get() {
        return true
    }
    
    false
}

/// send an RPC message to libqaul
pub fn send_rpc(binary_message: Vec<u8>) {
    Rpc::send_to_libqaul(binary_message);
}

/// receive a RPC message from libqaul
pub fn receive_rpc() -> Result<Vec<u8>, TryRecvError> {
    Rpc::receive_from_libqaul()
}

/// count of rpc messages to receive in the queue
pub fn receive_rpc_queued() -> usize {
    Rpc::receive_from_libqaul_queue_length()
}

/// count of sent rpc messages
pub fn send_rpc_count() -> i32 {
    Rpc::send_rpc_count()
}

/// send a SYS message to libqaul
pub fn send_sys(binary_message: Vec<u8>) {
    Sys::send_to_libqaul(binary_message);
}

/// receive a SYS message from libqaul
pub fn receive_sys() -> Result<Vec<u8>, TryRecvError> {
    Sys::receive_from_libqaul()
}

