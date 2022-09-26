use crate::{
    identity::IdentityPlugin,
    mosquitto::defs::{self, MOSQ_ERR_AUTH},
};

use super::defs::{mosquitto, MOSQ_PLUGIN_DEFER, MOSQ_SUCCESS};
use std::os::raw::{c_char, c_int, c_void};

///This function is OPTIONAL.  Only include this function in your plugin if you are making basic username/password checks.
///
/// Called by the broker when a username/password must be checked.
///
/// RETURN
///
/// MOSQ_ERR_SUCCESS if the user is authenticated.  MOSQ_ERR_AUTH if authentication failed.  MOSQ_ERR_UNKNOWN for an application specific error.  MOSQ_ERR_PLUGIN_DEFER if your plugin does not wish to handle this check.
#[no_mangle]
extern "C" fn mosquitto_auth_unpwd_check(
    user_data: *mut c_void,
    _client: *mut mosquitto,
    username: *const c_char,
    password: *const c_char,
) -> c_int {
    log::info!("auth unpdw check");

    let plugin = unsafe { &*(user_data as *mut IdentityPlugin) };

    let username = defs::cstr_to_ptr(username);
    if username.is_none() {
        return MOSQ_ERR_AUTH;
    }
    let username = username.unwrap();

    let password = defs::cstr_to_ptr(password);
    if password.is_none() {
        return MOSQ_ERR_AUTH;
    }
    let password = password.unwrap();

    match plugin.auth(username, password) {
        Ok(_) => MOSQ_SUCCESS,
        Err(_) => MOSQ_ERR_AUTH,
    }
}

/// This function is OPTIONAL.  Only include this function in your plugin if you are making TLS-PSK checks.
///
/// Called by the broker when a client connects to a listener using TLS/PSK.  This is used to retrieve the pre-shared-key associated with a client identity.
///
/// Examine hint and identity to determine the required PSK (which must be a hexadecimal string with no leading “0x”) and copy this string into key.
///
/// PARAMETERS
///
/// user_data:	the pointer provided in mosquitto_auth_plugin_init.
///
/// hint:	the psk_hint for the listener the client is connecting to.
///
/// identity:	the identity string provided by the client
///
/// key:	a string where the hex PSK should be copied
///
/// max_key_len:	the size of key
///
/// RETURN
///
/// Return 0 on success.  Return > 0 on failure.  Return MOSQ_ERR_PLUGIN_DEFER if your plugin does not wish to handle this check.
#[no_mangle]
extern "C" fn mosquitto_auth_psk_key_get(
    _user_data: *mut c_void,
    _client: *mut mosquitto,
    _hint: *const c_char,
    _identity: *const c_char,
    _key: *mut c_char,
    _max_key_len: c_int,
) -> c_int {
    MOSQ_PLUGIN_DEFER
}

/// This function is OPTIONAL.  Only include this function in your plugin if you are making extended authentication checks.
///
/// PARAMETERS
///
/// user_data:	the pointer provided in mosquitto_auth_plugin_init.
///
/// method:	the authentication method
///
/// reauth:	this is set to false if this is the first authentication attempt on a connection, set to true if the client is attempting to reauthenticate.
///
/// data_in:	pointer to authentication data, or NULL
///
/// data_in_len:	length of data_in, in bytes
///
/// data_out:	if your plugin wishes to send authentication data back to the client, allocate some memory using malloc or friends and set data_out.  The broker will free the memory after use.
///
/// data_out_len:	Set the length of data_out in bytes.
///
/// RETURN
///
/// Return MOSQ_ERR_SUCCESS if authentication was successful.  Return MOSQ_ERR_AUTH_CONTINUE if the authentication is a multi step process and can continue.  Return MOSQ_ERR_AUTH if authentication was valid but did not succeed.  Return any other relevant positive integer MOSQ_ERR_* to produce an error.
#[no_mangle]
extern "C" fn mosquitto_auth_start(
    _user_data: *mut c_void,
    _client: *mut mosquitto,
    _method: *const c_char,
    _reauth: bool,
    _data_in: *const c_void,
    _data_in_len: u16,
    _data_out: *mut *mut c_void,
    _data_out_len: *mut u16,
) -> c_int {
    log::debug!("mosquitto_auth_start");
    MOSQ_PLUGIN_DEFER
}

#[no_mangle]
extern "C" fn mosquitto_auth_continue(
    _user_data: *mut c_void,
    _client: *mut mosquitto,
    _method: *const c_char,
    _data_in: *const c_void,
    _data_in_len: u16,
    _data_out: *mut *mut c_void,
    _data_out_len: *mut u16,
) -> c_int {
    MOSQ_PLUGIN_DEFER
}
