use std::collections::HashMap;

pub(crate) struct Configs {
    server_address: String,
    server_oauth_path: String,
    client_id: String,
    realm: String,
    grant_type: String,
    scope: String,
    audience: String,
}

pub(crate) struct IdentityPlugin {
    cfg: Option<Configs>,
}

impl IdentityPlugin {
    pub fn new() -> IdentityPlugin {
        IdentityPlugin { cfg: None }
    }

    pub fn configs(&mut self, opts: HashMap<&str, &str>) -> Result<(), ()> {
        Ok(())
    }
}
