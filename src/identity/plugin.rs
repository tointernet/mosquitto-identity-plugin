use super::configs::Configs;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub(crate) struct AuthRequest<'a> {
    client_id: &'a str,
    realm: &'a str,
    grant_type: &'a str,
    username: &'a str,
    password: &'a str,
    scope: &'a str,
    audience: &'a str,
}

pub struct IdentityPlugin<'i> {
    cfg: Option<Configs<'i>>,
}

impl<'i> IdentityPlugin<'i> {
    pub fn new() -> IdentityPlugin<'i> {
        std::env::set_var("RUST_LOG", "DEBUG");
        env_logger::init();
        IdentityPlugin { cfg: None }
    }

    pub fn configs(&mut self, opts: HashMap<&str, &'i str>) -> Result<(), &str> {
        self.cfg = Some(Configs::new(&opts)?);
        Ok(())
    }

    pub fn auth(&self, username: &str, password: &str) -> Result<(), &'i str> {
        let cfg = self.cfg.unwrap();
        let path = format!("{}{}", cfg.server_address, cfg.server_oauth_path);

        let body = AuthRequest {
            client_id: cfg.client_id,
            realm: cfg.realm,
            grant_type: cfg.grant_type,
            username,
            password,
            scope: cfg.scope,
            audience: cfg.audience,
        };

        match ureq::post(&path)
            .set("Content-Type", "application/json")
            .send_string(&serde_json::to_string(&body).unwrap())
        {
            Ok(_) => Ok(()),
            Err(ureq::Error::Status(code, res)) => {
                log::error!("status: {} ,res: {:?}", code, res.into_string());
                Err("identity server error")
            }
            Err(e) => {
                log::error!("{:?}", e.to_string());
                Err("unexpected error")
            }
        }
    }
}
