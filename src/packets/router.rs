use crate::objects::player::{Player, PlayerList};
use crate::packets::rw::Reader;
use crate::web::server::RequestContext;
use std::sync::Arc;
use tokio::sync::RwLock;
use ntex::http::Response;


/// # Bancho Server
struct BanchoServer {
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

        self.handle_index().await
    }

    async fn handle_index(&self) -> Response {
        Response::from("cu.rs === The megachad bancho.")
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
