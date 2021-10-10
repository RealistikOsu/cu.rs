// Internal module declarations.
mod web;
mod logger;

use std::sync::{Arc, RwLock};
use web::server::{start_server, Address};

#[ntex::main]
async fn main() {
    start_server(Address::IPAddress("127.0.0.1:1337")).await;
}
