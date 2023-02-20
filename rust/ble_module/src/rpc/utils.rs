use super::{
    proto_sys::{self, BleError, BleStartResult},
    send_to_ui,
};

pub fn send_ble_sys_msg(msg: proto_sys::ble::Message) {
    let mut buf = Vec::with_capacity(msg.encoded_len());
    msg.encode(&mut buf);
    send_to_ui(buf);
}

pub fn send_result_already_running() {
    send_ble_sys_msg(BleStartResult {
        success: true,
        error_reason: BleError::UnknownError,
        error_message: "Advertisement already Started".into(),
    });
}
