use crate::objects::player::{Player, PlayerList};
use crate::packets::{rw::Reader, builders};
use crate::web::server::RequestContext;
use crate::logger;
use crate::consts::packet_ids;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, Mutex};
use ntex::http::Response;

use crate::events::{
    login,
    misc,
};

/// # Bancho Server
pub struct BanchoServer {
    online_players: PlayerList,
    all_players: PlayerList,
    uuid_store: Mutex<HashMap<String, i32>>,

    total_conns: u64,
}

impl BanchoServer {
    pub fn new() -> Self {
        Self {
            online_players: PlayerList::new(),
            all_players: PlayerList::new(),
            uuid_store: Mutex::new(HashMap::new()),
            total_conns: 0,
        }
    }

    pub async fn handle_conn(&mut self, req: RequestContext) -> Response {
        self.total_conns += 1;


        match req.header_value("User-Agent") {
            Some("osu!") => {
                self.handle_bancho(req).await
            },
            _ => {self.handle_index().await}
        }
    }

    async fn handle_index(&self) -> Response {
        Response::from("cu.rs === The megachad bancho.")
    }

    async fn handle_bancho(&self, mut req: RequestContext) -> Response {
        // If cho-token is not sent, this is a login request.
        let uuid: String;
        let packet_resp: Vec<u8>;

        match req.header_value("osu-token") {
            // Handle normal packets.
            Some(token) => {
                uuid = token.to_string();
                packet_resp = self.handle_packets(req, &uuid).await;
            },
            // Login request.
            _ => {
                let login_resp = login::login_handle(&mut req, &self).await;
                uuid = login_resp.0;
                packet_resp = login_resp.1
            }
        };

        // Build final response
        Response::Ok()
            .header("cho-token", uuid)
            .body(packet_resp)
    }

    // Handles a packet stream from osu.
    async fn handle_packets(&self, mut req: RequestContext, uuid: &String) -> Vec<u8> {
        // Fetch Player Obj
        let p: Arc<RwLock<Player>>;
        match self.player_from_uuid(uuid).await {
            Some(pl) => {p = pl;}
            _ => {return builders::server_restart(&0);}
        }
        // Create Packet Context
        let mut ctx = PacketContext {
            player: p,
            reader: Reader::new(req.read_body().await),
            server: &self
        };

        while !ctx.reader.empty() {
            let (p_id, p_len) = ctx.reader.read_headers();
            match p_id {
                // Handle individual packets.
                packet_ids::OSU_PING => {misc::handle_ping(&ctx).await}
                // If we do not have a handler for it, incr buffer.
                _ => {
                    logger::debug(format!("No handler for packet with id {}", p_id));
                    ctx.reader.incr_buffer(p_len as usize);
                }
            }
        }

        vec![]
    }

    /// # Player From UUID
    pub async fn player_from_uuid(&self, uuid: &String) -> Option<Arc<RwLock<Player>>> {
        if let Some(player_id) = self.uuid_store.lock().await.get(uuid) {
            self.online_players.get(player_id.clone()).await
        } else {
            None
        }
    }
}

/// A context struct provided to all packet events.
pub struct PacketContext<'a> {
    pub player: Arc<RwLock<Player>>,
    pub reader: Reader,
    pub server: &'a BanchoServer,
}

// This is really weird but i have a headache.
static mut bancho_server: Option<BanchoServer> = None;

pub async fn create_bancho_server() {
    unsafe {
        bancho_server = Some(BanchoServer::new());
    }
}

/// A really wacky, hacky solution to get back into the class.
pub async fn handle_bancho(req: RequestContext) -> Response {
    unsafe {
        return bancho_server.as_mut().unwrap().handle_conn(req).await
    };
}
