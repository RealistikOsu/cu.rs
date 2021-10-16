use crate::consts::{
    privileges::Privileges,
    modes::{Mode, CustomMode},
};

/// A structure representing a physical location of a user.
pub struct Geolocation {
    pub country: u8,
    pub location: (f32, f32),
    pub ip: String,
}

const BYTEQUEUE_CAPACITY: usize = 512;

pub struct ByteQueue {
    queue: Vec<u8>,
}

impl ByteQueue {
    /// Creates an empty instance of `ByteQueue`.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            queue: Vec::with_capacity(BYTEQUEUE_CAPACITY),
        }
    }

    /// Empties the ByteQueue, returning its previous contents prior to the
    /// clearing.
    pub fn empty(&mut self) -> Vec<u8> {
        let old_contents = self.queue.clone();
        self.queue.clear();
        old_contents
    }

    /// Enqueues bytes to the `ByteQueue`.
    #[inline(always)]
    pub fn enqueue(&mut self, mut bytes: Vec<u8>) {
        self.queue.append(&mut bytes);
    }
}

/// A structure representing a Player's in-game status.
pub struct Action {
    pub id: u8,
    pub text: String,
    pub bmap_md5: String,
    pub bmap_id: String,
    pub mods: u32, // TODO: Mods class.
}

/// A structure representing an in-game player.
pub struct Player {
    pub id: i32,
    pub name: String,
    pub safe_name: String,
    pub uuid: String,

    pub location: Geolocation,
    pub privileges: Privileges,
    pub action: Action,
    pub mode: Mode,
    pub c_mode: CustomMode,
}
