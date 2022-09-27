use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_long, c_void},
};

pub const MOSQ_AUTH_PLUGIN_VERSION: c_int = 4;

pub const MOSQ_SUCCESS: c_int = 0;
// pub const MOSQ_ERR_UNKNOWN: c_int = 1;
pub const MOSQ_ERR_AUTH: c_int = 11;
// pub const MOSQ_ERR_ACL_DENIED: c_int = 12;
pub const MOSQ_PLUGIN_DEFER: c_int = 17;

// pub const MOSQ_ACL_READ: c_int = 1;
// pub const MOSQ_ACL_WRITE: c_int = 2;
// pub const MOSQ_ACL_SUBSCRIBE: c_int = 4;

pub const PLUGIN_SERVER_ADDRESS_KEY: &str = "identity_server_address";
pub const PLUGIN_SERVER_OAUTH_PATH_KEY: &str = "identity_server_oauth_path";
pub const PLUGIN_CLIENT_ID_KEY: &str = "identity_client_id";
pub const PLUGIN_REALM_KEY: &str = "identity_realm";
pub const PLUGIN_GRANT_TYPE_KEY: &str = "identity_grant_type";
pub const PLUGIN_SCOPE_KEY: &str = "identity_scope";
pub const PLUGIN_AUDIENCE_KEY: &str = "identity_audience";

#[repr(C)]
pub struct mosquitto_opt {
    pub key: *mut c_char,
    pub value: *mut c_char,
}

#[repr(C)]
pub struct mosquitto {
    _unused: [u8; 0],
}

pub struct mosquitto_acl_msg {
    pub topic: *const c_char,
    pub _payload: *const c_void,
    pub _payloadlen: c_long,
    pub _qos: c_int,
    pub _retain: bool,
}

pub fn cstr_to_ptr<'a>(cstr: *const c_char) -> Option<&'a str> {
    if cstr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(cstr) }.to_str().unwrap())
    }
}
