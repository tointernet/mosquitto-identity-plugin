// mosquitto_auth_plugin_version    >>>	The broker will call this function immediately after loading the plugin to check it is a supported plugin version.
// mosquitto_auth_plugin_init	      >>> Called after the plugin has been loaded and mosquitto_auth_plugin_version has been called.
// mosquitto_auth_plugin_cleanup    >>>	Called when the broker is shutting down.
// mosquitto_auth_security_init	    >>> 1.
// mosquitto_auth_security_cleanup	>>> 1.
// mosquitto_auth_acl_check         >>>	Called by the broker when topic access must be checked.

use std::os::raw::{c_char, c_int, c_long, c_void};

const MOSQ_AUTH_PLUGIN_VERSION: c_int = 4;

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

///The broker will call this function immediately after loading the plugin to check it is a supported plugin version.  
/// Your code must simply return the version of the plugin interface you support, i.e.  4.
#[no_mangle]
extern "C" fn mosquitto_auth_plugin_version() -> c_int {
    println!("called_from_rust::mosquitto_auth_plugin_version");
    MOSQ_AUTH_PLUGIN_VERSION
}

/// Called after the plugin has been loaded and mosquitto_auth_plugin_version has been called.  This will only ever be called once and can be used to initialise the plugin.
///
/// PARAMETERS:
///
/// user_data:	The pointer set here will be passed to the other plugin functions.  Use to hold connection information for example.
///
/// opts:	Pointer to an array of struct mosquitto_opt, which provides the plugin options defined in the configuration file.
///
/// opt_count:	The number of elements in the opts array.
///
/// RETURN:
///
/// Return 0 on success Return > 0 on failure.
#[no_mangle]
extern "C" fn mosquitto_auth_plugin_init(
    _user_data: *mut *mut c_void,
    _opts: *mut mosquitto_opt,
    _opt_count: c_int,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_plugin_init");
    0
}

///Called when the broker is shutting down.  This will only ever be called once per plugin.  Note that mosquitto_auth_security_cleanup will be called directly before this function.
///
/// PARAMETERS:
///
/// user_data:	The pointer provided in mosquitto_auth_plugin_init.
///
/// opts:	Pointer to an array of struct mosquitto_opt, which provides the plugin options defined in the configuration file.
///
/// opt_count:	The number of elements in the opts array.
///
/// RETURN
///
/// Return 0 on success Return > 0 on failure.
#[no_mangle]
extern "C" fn mosquitto_auth_plugin_cleanup(
    _user_data: *mut c_void,
    _opts: *mut mosquitto_opt,
    _opt_count: c_int,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_plugin_cleanup");
    0
}

/// This function is called in two scenarios:
///
/// 1.  When the broker starts up.  2. If the broker is requested to reload its configuration whilst running.  In this case, mosquitto_auth_security_cleanup will be called first, then this function will be called.  In this situation, the reload parameter will be true.
///
/// PARAMETERS:
///
/// user_data:	The pointer provided in mosquitto_auth_plugin_init.
///
/// opts:	Pointer to an array of struct mosquitto_opt, which provides the plugin options defined in the configuration file.
///
/// opt_count:	The number of elements in the opts array.
///
/// reload:	If set to false, this is the first time the function has been called.  If true, the broker has received a signal asking to reload its configuration.
///
/// RETURN:
///
/// Return 0 on success Return > 0 on failure.
#[no_mangle]
extern "C" fn mosquitto_auth_security_init(
    _user_data: *mut c_void,
    _opts: *mut mosquitto_opt,
    _opt_count: c_int,
    _reload: bool,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_security_init");
    0
}

/// This function is called in two scenarios
///
/// 1.  When the broker is shutting down.  2. If the broker is requested to reload its configuration whilst running.  In this case, this function will be called, followed by mosquitto_auth_security_init.  In this situation, the reload parameter will be true.
///
/// PARAMETERS:
///
/// user_data:	The pointer provided in mosquitto_auth_plugin_init.
///
/// opts:	Pointer to an array of struct mosquitto_opt, which provides the plugin options defined in the configuration file.
///
/// opt_count:	The number of elements in the opts array.
///
/// reload:	If set to false, this is the first time the function has been called.  If true, the broker has received a signal asking to reload its configuration./
///
/// RETURN
///
/// Return 0 on success Return > 0 on failure.
#[no_mangle]
extern "C" fn mosquitto_auth_security_cleanup(
    _user_data: *mut c_void,
    _opts: *mut mosquitto_opt,
    _opt_count: c_int,
    _reload: bool,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_security_cleanup");
    0
}

/// Called by the broker when topic access must be checked.
/// Access will be one of: MOSQ_ACL_SUBSCRIBE when a client is asking to subscribe to a topic string.  
/// This differs from MOSQ_ACL_READ in that it allows you to deny access to topic strings rather than by pattern.  
/// For example, you may use MOSQ_ACL_SUBSCRIBE to deny subscriptions to ‘#’, but allow all topics in MOSQ_ACL_READ.  
/// This allows clients to subscribe to any topic they want, but not discover what topics are in use on the server.  
/// MOSQ_ACL_READ when a message is about to be sent to a client (i.e. whether it can read that topic or not).  
/// MOSQ_ACL_WRITE when a message has been received from a client (i.e. whether it can write to that topic or not).
///
/// RETURN:
///
/// MOSQ_ERR_SUCCESS: if access was granted.  MOSQ_ERR_ACL_DENIED: if access was not granted.  MOSQ_ERR_UNKNOWN: for an application specific error.  MOSQ_ERR_PLUGIN_DEFER: if your plugin does not wish to handle this check.
#[no_mangle]
extern "C" fn mosquitto_auth_acl_check(
    _user_data: *mut c_void,
    _access: c_int,
    _client: *mut mosquitto,
    _msg: *const mosquitto_acl_msg,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_acl_check");
    0
}
