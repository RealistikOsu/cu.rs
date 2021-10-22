// cu.rs packet builders.
use crate::packets::rw::Writer;
use crate::consts::packet_ids;

/// Writes a Server Restart packet.
pub fn server_restart(time: &u32) -> Vec<u8> {
    let mut w = Writer::new(packet_ids::SRV_RESTART);
    w.write_int(time);
    w.build()
}

pub fn ping() -> Vec<u8> {
    Writer::new(packet_ids::SRV_PONG).build()
}

pub fn login_reply(user_id: &i32) -> Vec<u8> {
    let mut w = Writer::new(packet_ids::SRV_USER_ID);
    w.write_int(user_id);
    w.build()
}
