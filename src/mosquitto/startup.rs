use super::defs::{mosquitto_opt, MOSQ_AUTH_PLUGIN_VERSION, MOSQ_SUCCESS};
use std::os::raw::{c_int, c_void};

///The broker will call this function immediately after loading the plugin to check it is a supported plugin version.  
/// Your code must simply return the version of the plugin interface you support, i.e.  4.
#[no_mangle]
extern "C" fn mosquitto_auth_plugin_version() -> c_int {
    println!("called_from_rust::mosquitto_auth_plugin_version");
    MOSQ_AUTH_PLUGIN_VERSION
}

/// Called after the plugin has been loaded and mosquitto_auth_plugin_version has been called.  This will only ever be called once and can
/// be used to initialise the plugin.
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
    MOSQ_SUCCESS
}

///Called when the broker is shutting down.  This will only ever be called once per plugin.  Note that mosquitto_auth_security_cleanup will
/// be called directly before this function.
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
    MOSQ_SUCCESS
}

/// This function is called in two scenarios:
///
/// 1.  When the broker starts up.  2. If the broker is requested to reload its configuration whilst running.  In this case,
/// mosquitto_auth_security_cleanup will be called first, then this function will be called.  In this situation, the reload parameter will
/// be true.
///
/// PARAMETERS:
///
/// user_data:	The pointer provided in mosquitto_auth_plugin_init.
///
/// opts:	Pointer to an array of struct mosquitto_opt, which provides the plugin options defined in the configuration file.
///
/// opt_count:	The number of elements in the opts array.
///
/// reload:	If set to false, this is the first time the function has been called.  If true, the broker has received a signal asking to
/// reload its configuration.
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
    MOSQ_SUCCESS
}

/// This function is called in two scenarios
///
/// 1.  When the broker is shutting down.  2. If the broker is requested to reload its configuration whilst running.  In this case, this
/// function will be called, followed by mosquitto_auth_security_init.  In this situation, the reload parameter will be true.
///
/// PARAMETERS:
///
/// user_data:	The pointer provided in mosquitto_auth_plugin_init.
///
/// opts:	Pointer to an array of struct mosquitto_opt, which provides the plugin options defined in the configuration file.
///
/// opt_count:	The number of elements in the opts array.
///
/// reload:	If set to false, this is the first time the function has been called.  If true, the broker has received a signal asking to
/// reload its configuration.
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
    MOSQ_SUCCESS
}
