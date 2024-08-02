//////////
// SSH //
////////
//////////////////////////////////////////////////////////////////////////////
// https://github.com/Eugeny/russh/blob/main/russh/examples/sftp_server.rs //
////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::env::var;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use async_trait::async_trait;
use russh::server::{Auth, Msg, Server as _, Session};
use russh::{Channel, ChannelId};
use russh_keys::key::KeyPair;
use tokio::sync::Mutex;

use crate::endpoint::post_add_attacker;

pub struct Server;

impl russh::server::Server for Server {
    type Handler = SshSession;

    fn new_client(&mut self, ip: Option<SocketAddr>) -> Self::Handler {
        let mut ssh = SshSession::default();
        ssh.ip = ip;
        ssh
    }
}

#[derive(Debug)]
pub struct SshSession {
    clients: Arc<Mutex<HashMap<ChannelId, Channel<Msg>>>>,
    ip: Option<SocketAddr>
}

impl Default for SshSession {
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            ip: None
        }
    }
}

impl SshSession {
    pub async fn get_channel(&mut self, channel_id: ChannelId) -> Channel<Msg> {
        let mut clients = self.clients.lock().await;
        clients.remove(&channel_id).unwrap()
    }
}

#[async_trait]
impl russh::server::Handler for SshSession {
    type Error = anyhow::Error;
    async fn auth_password(&mut self, user: &str, password: &str) -> Result<Auth, Self::Error> {
        let admin_username = var("SSH_ADMIN_USERNAME").unwrap();
        let admin_password = var("SSH_ADMIN_PASSWORD").unwrap();
        if user.eq(&admin_username) && password.eq(&admin_password) {
            return Ok(Auth::Accept);
        }
        match self.ip {
            Some(ip) => {
                post_add_attacker(user, password, &ip.ip().to_string(), "SSH").await.unwrap()
            },
            None => {},
        }
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn auth_publickey(
        &mut self,
        user: &str,
        public_key: &russh_keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::UnsupportedMethod)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        mut session: &mut Session,
    ) -> Result<bool, Self::Error> {
        {
            println!("session opened");
            let mut clients = self.clients.lock().await;
            clients.insert(channel.id(), channel);
        }
        Ok(true)
    }

    async fn subsystem_request(
        &mut self,
        channel_id: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        // You can add sftp here only sftp not ftp.
        Ok(())
    }
}

pub async fn listen_on_ssh() -> anyhow::Result<()> {
    let config = russh::server::Config {
        auth_rejection_time: Duration::from_secs(3),
        auth_rejection_time_initial: Some(Duration::from_secs(0)),
        keys: vec![KeyPair::generate_ed25519().unwrap()],
        ..Default::default()
    };

    let mut server = Server;

    server
        .run_on_address(Arc::new(config), ("0.0.0.0", "22".parse().unwrap()))
        .await?;
    Ok(())
}
