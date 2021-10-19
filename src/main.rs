// Internal module declarations.
mod web;
mod logger;
mod consts;
mod objects;
mod packets;
mod config;
mod events;

use web::server::{start_server, Address};
use packets::router::create_bancho_server;
use config::ensure_config;

#[ntex::main]
async fn main() {
    let conf = ensure_config();
    create_bancho_server().await;
    start_server(Address::IPAddress(conf.http_ip)).await;
}
