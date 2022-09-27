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
        std::env::set_var("RUST_LOG", "INFO");
        #[cfg(not(test))]
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
            .timeout(std::time::Duration::from_secs(1))
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

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn test_auth_correctly() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api")
                .header("content-type", "application/json");
            then.status(200)
                .header("content-type", "application/json")
                .body("");
        });

        let mut plugin = IdentityPlugin::new();
        let address = format!("http://{}", server.address().to_string());
        let map = new_config_opts(&address);
        let res = plugin.configs(map);
        assert!(res.is_ok());

        let res = plugin.auth("username", "password");

        assert!(res.is_ok());
        mock.assert();
    }

    #[test]
    fn test_auth_when_unauthorize() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api")
                .header("content-type", "application/json");
            then.status(401)
                .header("content-type", "application/json")
                .body("");
        });

        let mut plugin = IdentityPlugin::new();
        let address = format!("http://{}", server.address().to_string());
        let map = new_config_opts(&address);
        let res = plugin.configs(map);
        assert!(res.is_ok());

        let res = plugin.auth("username", "password");

        assert!(res.is_err());
        mock.assert();
    }

    fn new_config_opts<'n>(address: &'n str) -> HashMap<&'n str, &'n str> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("identity_server_address", address);
        map.insert("identity_server_oauth_path", "/api");
        map.insert("identity_client_id", "id");
        map.insert("identity_realm", "realm");
        map.insert("identity_grant_type", "type");
        map.insert("identity_scope", "scope");
        map.insert("identity_audience", "audience");

        map
    }
}
