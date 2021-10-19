use crate::web::server::RequestContext;
use crate::packets::router::BanchoServer;
use crate::logger;

/// A structure of the data provided in a login request.
struct LoginData {
    username: String,
    password_md5: String,
    osu_version: String,
    timezone: u8,
    allow_dms: bool,
    osu_hash: String,
    adapter_hash: String,
    uninstaller_hash: String,
    serial_hash: String,
}

impl LoginData {
    /// Attempts to parse the login data into a structure. On fail, returns
    /// `None`.
    pub fn from_body(body: String) -> Option<Self> {
        let body_parts: Vec<_> = body.split("\n").collect();

        if body_parts.len() != 4 {
            logger::debug("Received invalid main body size.");
            return None;
        }

        let username = body_parts[0];
        let password_md5 = body_parts[1];

        let s_data: Vec<_> = body_parts[2].split("|").collect();

        if s_data.len() != 5 {
            logger::debug("Received invalid s_data size.");
            return None;
        }

        let osu_version = s_data[0];
        let timezone: u8 = s_data[1].parse().unwrap_or(0);
        let allow_dms: bool = s_data[4].parse().unwrap_or(true);

        let client_hashes: Vec<_> = s_data[3].split(":").collect();

        if client_hashes.len() != 6 {
            logger::debug("Received invalid client_hashes size.");
            return None;
        }

        let osu_hash = client_hashes[0];
        let adapter_hash = client_hashes[2];
        let osu_uninst_hash = client_hashes[3];
        let serial_hash = client_hashes[4];

        Some(Self {
            username: username.to_string(),
            password_md5: password_md5.to_string(),
            osu_version: osu_version.to_string(),
            adapter_hash: adapter_hash.to_string(),
            allow_dms: allow_dms,
            osu_hash: osu_hash.to_string(),
            serial_hash: serial_hash.to_string(),
            timezone: timezone,
            uninstaller_hash: osu_uninst_hash.to_string(),
        })
    }
}

/// Handles the action of logging into the server
pub async fn login_handle(req: &mut RequestContext) -> (String, Vec<u8>) {
    let login_data = LoginData::from_body(req.read_string().await);
    ("no".to_string(), vec![])
}
