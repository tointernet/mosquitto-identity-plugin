use crate::mosquitto::defs::{
    PLUGIN_AUDIENCE_KEY, PLUGIN_CLIENT_ID_KEY, PLUGIN_GRANT_TYPE_KEY, PLUGIN_REALM_KEY,
    PLUGIN_SCOPE_KEY, PLUGIN_SERVER_ADDRESS_KEY, PLUGIN_SERVER_OAUTH_PATH_KEY,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Configs<'c> {
    pub server_address: &'c str,
    pub server_oauth_path: &'c str,
    pub client_id: &'c str,
    pub realm: &'c str,
    pub grant_type: &'c str,
    pub scope: &'c str,
    pub audience: &'c str,
}

impl<'c> Configs<'c> {
    pub fn new(map: &HashMap<&str, &'c str>) -> Result<Configs<'c>, &'c str> {
        let server_address = map.get(PLUGIN_SERVER_ADDRESS_KEY);
        if server_address.is_none() {
            return Err("missing server_address config");
        }
        let &server_address = server_address.unwrap();

        let server_oauth_path = map.get(PLUGIN_SERVER_OAUTH_PATH_KEY);
        if server_oauth_path.is_none() {
            return Err("missing server_oauth_path config");
        }
        let &server_oauth_path = server_oauth_path.unwrap();

        let client_id = map.get(PLUGIN_CLIENT_ID_KEY);
        if client_id.is_none() {
            return Err("missing client_id config");
        }
        let &client_id = client_id.unwrap();

        let realm = map.get(PLUGIN_REALM_KEY);
        if realm.is_none() {
            return Err("missing relam config");
        }
        let &realm = realm.unwrap();

        let grant_type = map.get(PLUGIN_GRANT_TYPE_KEY);
        if grant_type.is_none() {
            return Err("missing grant_type config");
        }
        let &grant_type = grant_type.unwrap();

        let scope = map.get(PLUGIN_SCOPE_KEY);
        if scope.is_none() {
            return Err("missing scope config");
        }
        let &scope = scope.unwrap();

        let audience = map.get(PLUGIN_AUDIENCE_KEY);
        if audience.is_none() {
            return Err("missing audience config");
        }
        let &audience = audience.unwrap();

        Ok(Configs {
            server_address,
            server_oauth_path,
            client_id,
            realm,
            grant_type,
            scope,
            audience,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config_correctly() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("identity_server_address", "address");
        map.insert("identity_server_oauth_path", "path");
        map.insert("identity_client_id", "id");
        map.insert("identity_realm", "realm");
        map.insert("identity_grant_type", "type");
        map.insert("identity_scope", "scope");
        map.insert("identity_audience", "audience");

        let cfg = Configs::new(&map);
        assert!(cfg.is_ok());
    }

    #[test]
    fn test_new_config_when_some_values_missing() {
        let mut map: HashMap<&str, &str> = HashMap::new();

        let cfg = Configs::new(&map);
        assert!(cfg.is_err());

        map.insert("identity_server_address", "address");
        let cfg = Configs::new(&map);
        assert!(cfg.is_err());

        map.insert("identity_server_oauth_path", "path");
        let cfg = Configs::new(&map);
        assert!(cfg.is_err());

        map.insert("identity_client_id", "id");
        let cfg = Configs::new(&map);
        assert!(cfg.is_err());

        map.insert("identity_realm", "realm");
        let cfg = Configs::new(&map);
        assert!(cfg.is_err());

        map.insert("identity_grant_type", "type");
        let cfg = Configs::new(&map);
        assert!(cfg.is_err());

        map.insert("identity_scope", "scope");
        let cfg = Configs::new(&map);
        assert!(cfg.is_err());
    }
}
