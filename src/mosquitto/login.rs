use super::defs::{mosquitto, MOSQ_PLUGIN_DEFER, MOSQ_SUCCESS};
use std::os::raw::{c_char, c_int, c_void};

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
extern "C" fn mosquitto_auth_unpwd_check(
    _user_data: *mut c_void,
    _client: *mut mosquitto,
    _username: *const c_char,
    _password: *const c_char,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_unpwd_check");
    MOSQ_SUCCESS
}

#[no_mangle]
extern "C" fn mosquitto_auth_psk_key_get(
    _user_data: *mut c_void,
    _client: *mut mosquitto,
    _hint: *const c_char,
    _identity: *const c_char,
    _key: *mut c_char,
    _max_key_len: c_int,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_psk_key_get");
    MOSQ_PLUGIN_DEFER
}

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
    println!("called_from_rust::mosquitto_auth_start");
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
    println!("called_from_rust::mosquitto_auth_continue");
    MOSQ_PLUGIN_DEFER
}
