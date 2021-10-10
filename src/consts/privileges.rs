// Realistik trying macros S1E1
macro_rules! privilege_check {
    ($priv_name: ident, $priv_const: ident) => {
        #[inline(always)]
        pub fn $priv_name(&self) -> bool {
            self.privs & Self::$priv_const > 0
        }
    };
}

pub struct Privileges {
    pub privs: u32
}

impl Privileges {
    // https://github.com/RealistikOsu/common/blob/master/constants/privileges.py
    // TODO: Maybe also make these part of the macro.
    const USER_PUBLIC               : u32 = 1;
    const USER_NORMAL               : u32 = 2 << 0;
    const USER_DONOR                : u32 = 2 << 1;
    const ADMIN_ACCESS_RAP          : u32 = 2 << 2;
    const ADMIN_MANAGE_USERS        : u32 = 2 << 3;
    const ADMIN_BAN_USERS           : u32 = 2 << 4;
    const ADMIN_SILENCE_USERS       : u32 = 2 << 5;
    const ADMIN_WIPE_USERS          : u32 = 2 << 6;
    const ADMIN_MANAGE_BEATMAPS     : u32 = 2 << 7;
    const ADMIN_MANAGE_SERVERS      : u32 = 2 << 8;
    const ADMIN_MANAGE_SETTINGS     : u32 = 2 << 9;
    const ADMIN_MANAGE_BETAKEYS     : u32 = 2 << 10;
    const ADMIN_MANAGE_REPORTS      : u32 = 2 << 11;
    const ADMIN_MANAGE_DOCS         : u32 = 2 << 12;
    const ADMIN_MANAGE_BADGES       : u32 = 2 << 13;
    const ADMIN_VIEW_RAP_LOGS       : u32 = 2 << 14;
    const ADMIN_MANAGE_PRIVILEGES   : u32 = 2 << 15;
    const ADMIN_SEND_ALERTS         : u32 = 2 << 16;
    const ADMIN_CHAT_MOD            : u32 = 2 << 17;
    const ADMIN_KICK_USERS          : u32 = 2 << 18;
    const USER_PENDING_VERIFICATION : u32 = 2 << 19;
    const USER_TOURNAMENT_STAFF     : u32 = 2 << 20;
    const ADMIN_CAKER               : u32 = 20 << 21;

    /// # Empty Privileges
    /// Creates a new instance of `Privileges` featuring no privileges.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            privs: 0
        }
    }

    /// # Privileges
    /// Creates an instance of privileges from an existing bitwise enum
    /// integer.
    #[inline(always)]
    pub fn from_bitwise(privs: u32) -> Self {
        Self {
            privs: privs
        }
    }

    // Individual delarations. TODO: do all constants
    privilege_check!(public, USER_PUBLIC);
    privilege_check!(normal, USER_NORMAL);
    privilege_check!(donor, USER_DONOR);
    privilege_check!(access_panel, ADMIN_ACCESS_RAP);
    privilege_check!(manage_users, ADMIN_MANAGE_USERS);
    privilege_check!(ban_users, ADMIN_BAN_USERS);
    privilege_check!(silence_users, ADMIN_SILENCE_USERS);
    privilege_check!(wipe_users, ADMIN_WIPE_USERS);
    privilege_check!(rank_beatmaps, ADMIN_MANAGE_BEATMAPS);
    privilege_check!(edit_server, ADMIN_MANAGE_SERVERS);
    privilege_check!(edit_settings, ADMIN_MANAGE_SETTINGS);
    privilege_check!(edit_privileges, ADMIN_MANAGE_PRIVILEGES);
    privilege_check!(send_alerts, ADMIN_SEND_ALERTS);
    privilege_check!(chat_mod, ADMIN_CHAT_MOD);
    privilege_check!(kick_user, ADMIN_KICK_USERS);
    privilege_check!(unverified, USER_PENDING_VERIFICATION);

    /// # Has Any
    /// Checks if the privilege features any of the given bitwise flags.
    pub fn has_any(&self, flag: u32) -> bool {
        self.privs & flag > 0
    }

    /// # Has All
    /// Checks if the privileges features all of a given bitwise flag.
    pub fn has_all(&self, flag: u32) -> bool {
        self.privs & flag == flag
    }

    /// # Reset Privilege
    /// Sets the saved privilege to a new value. Requires mutable privilege.
    pub fn reset_priv(&mut self, flag: u32) {
        self.privs = flag;
    }
}
