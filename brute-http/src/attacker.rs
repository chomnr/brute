use actix::Message;
use serde::Deserialize;

use crate::flags::Flags;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Attacker {
    /// PAM_USER
    username: String,

    /// PAM_AUTHTOK
    password: String,

    /// PAM_RHOST
    ip: String,

    /// PAM_SERVICE
    protocol: String,
}

impl Attacker {
    pub fn new(username: &str, password: &str, ip: &str, protocol: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
            ip: String::from(ip),
            protocol: String::from(protocol)
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AttackerRequest {
    payload: Attacker,
    flags: Flags
}

impl Message for AttackerRequest {
    type Result = ();
}

impl AttackerRequest {
    pub fn new(attacker: Attacker, flags: Flags) -> Self {
        Self {
            payload: attacker,
            flags,
        }
    }
}