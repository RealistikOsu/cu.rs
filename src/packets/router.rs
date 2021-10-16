use crate::objects::player::Player;
use crate::packets::rw::Reader;
use std::sync::{Arc, RwLock};

/// A context struct provided to all packet events.
struct PacketContext {
    player: Arc<RwLock<Player>>,
    reader: Reader,
}
