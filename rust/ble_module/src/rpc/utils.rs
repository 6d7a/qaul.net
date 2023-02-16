use super::{proto_sys, send_to_ui};

pub fn send_ble_sys_msg(msg: proto_sys::ble::Message) {
    let mut buf = Vec::with_capacity(msg.encoded_len());
    msg.encode(&mut buf);
    send_to_ui(buf);
}
