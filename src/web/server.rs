// The cu.rs web server wrapper.
use ntex::http::{Request, Response, Payload, HttpService, Method, HeaderMap};
use ntex::server::Server;
use futures::StreamExt;
use crate::logger;
use crate::packets::router::handle_bancho;

/// The cu.rs request object wrapper, offering common functions.
pub struct RequestContext {
    req: Request,
}

impl RequestContext {
    pub fn from_req(req: Request) -> Self {
        Self {
            req: req
        }
    }

    /// Fetches the value of the given header as a string, if failed or cannot
    /// convert to string, returns None.
    pub fn header_value(&self, header: &str) -> Option<&str> {
        
        if let Some(header_val) = self.req.headers().get(header) {
            if let Ok(h) = header_val.to_str() {
                return Some(h);
            }
        }
        None
    }

    /// # Get IP
    /// Fetches the IP of the requester using the Nginx header `X-Real-IP`. If
    /// not present, returns `127.0.0.1`.
    pub fn get_ip(&self) -> &str {
        self.header_value("X-Real-IP").unwrap_or("127.0.0.1")
    }

    /// # Read Body Bytes
    /// Reads the request body as bytes, creating and returning a vector of
    /// u8s.
    pub async fn read_body(&mut self) -> Vec<u8> {
        let mut body: Vec<u8> = Vec::new();
        let payload: &mut Payload = self.req.payload();

        loop {
            if let Some(b) = payload.next().await {
                if let Ok(byte) = b {
                    body.extend_from_slice(&byte);
                }
            }
            else {break;}
        }

        body
    }

    /// # Read Body String
    /// Reads the body bytes and converts them into a string, returning an empty
    /// string on fail.
    #[inline(always)]
    pub async fn read_string(&mut self) -> String {
        let body = self.read_body().await;
        String::from_utf8(body).unwrap_or(String::new())
    }
}

pub enum Address {
    UNIXSock(String),
    IPAddress(String),
}

// Web server

/// # Server Start
/// Starts listening on an address.
async fn start_server_ip(addr: String) -> std::io::Result<()> {
    logger::info(format!("Starting cu.rs on http://{}", addr));

    Server::build()
    .bind("cu.rs", addr, || {
        HttpService::build().finish(handle_conn).tcp()
    })?
    .run()
    .await
}

pub async fn start_server(addr: Address) -> std::io::Result<()> {
    match addr {
        Address::IPAddress(address) => {
            start_server_ip(address).await
        },
        Address::UNIXSock(_) => {
            panic!("Unix sockets are not yet implemented!");
        }
    }
}

async fn handle_conn(req: Request) -> Result<Response, std::io::Error> {
    let req_ctx = RequestContext::from_req(req);

    Ok(handle_bancho(req_ctx).await)
}
