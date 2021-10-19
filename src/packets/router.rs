use crate::objects::player::{Player, PlayerList};
use crate::packets::rw::Reader;
use crate::web::server::RequestContext;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use ntex::http::Response;

use crate::events::{
    login
};

/// # Bancho Server
pub struct BanchoServer {
    online_players: PlayerList,
    all_players: PlayerList,

    total_conns: u64,
}

impl BanchoServer {
    pub fn new() -> Self {
        Self {
            online_players: PlayerList::new(),
            all_players: PlayerList::new(),
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
        /// If cho-token is not sent, this is a login request.
        let uuid: String;
        let packet_resp: Vec<u8>;

        match req.header_value("osu-token") {
            // Handle normal packets.
            Some(token) => {
                uuid = token.to_string();
                packet_resp = self.handle_packets(req).await;
            },
            // Login request.
            _ => {
                let login_resp = login::login_handle(&mut req).await;
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
    async fn handle_packets(&self, req: RequestContext) -> Vec<u8> {
        vec![]
    }
}

/// A context struct provided to all packet events.
struct PacketContext {
    player: Arc<RwLock<Player>>,
    reader: Reader,
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
