use crate::consts::{
    privileges::Privileges,
    modes::{Mode, CustomMode},
};
use std::{
    collections::HashMap,
    sync::{Arc}
};
use tokio::sync::{RwLock, Mutex};


/// A structure representing a physical location of a user.
pub struct Geolocation {
    pub country: u8,
    pub location: (f32, f32),
    pub ip: String,
}

const BYTEQUEUE_CAPACITY: usize = 512;

/// A thread-safe, async-friendly queue of bytes.
pub struct ByteQueue {
    queue: Mutex<Vec<u8>>,
}

impl ByteQueue {
    /// Creates an empty instance of `ByteQueue`.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::with_capacity(BYTEQUEUE_CAPACITY)),
        }
    }

    /// Empties the ByteQueue, returning its previous contents prior to the
    /// clearing.
    pub async fn empty(&mut self) -> Vec<u8> {
        let mut q = self.queue.lock().await;
        let old_contents = q.clone();
        q.clear();
        old_contents
    }

    /// Enqueues bytes to the `ByteQueue`.
    #[inline(always)]
    pub async fn enqueue(&self, mut bytes: Vec<u8>) {
        self.queue.lock().await.append(&mut bytes);
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

    pub queue: ByteQueue,
}

/// A list of players, holding Arc + RwLock references and supporting
/// broadcasting efficiently.
pub struct PlayerList {
    players: HashMap<i32, Arc<RwLock<Player>>>,
}

impl PlayerList {
    /// Creates an empty player list.
    pub fn new() -> Self {
        Self { players: HashMap::new() }
    }

    /// Adds a player from a directly owner player structure.
    pub fn add_player(&mut self, p: Player) {
        let p_id = p.id.clone();
        let pl = Arc::from(RwLock::from(p));

        self.players.insert(p_id, pl);
    }

    /// # Broadcast
    /// Queues the given packet vector to all players in the list.
    pub async fn broadcast(&mut self, packet: Vec<u8>) {
        for player in self.players.values() {
            let p = player.read().await;

            p.queue.enqueue(packet.clone());
        }
    }
}
