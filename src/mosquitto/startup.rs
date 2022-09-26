use crate::identity::IdentityPlugin;

use super::defs::{mosquitto_opt, MOSQ_AUTH_PLUGIN_VERSION, MOSQ_SUCCESS};
use std::ffi::CStr;
use std::os::raw::{c_int, c_void};

///The broker will call this function immediately after loading the plugin to check it is a supported plugin version.  
/// Your code must simply return the version of the plugin interface you support, i.e.  4.
#[no_mangle]
extern "C" fn mosquitto_auth_plugin_version() -> c_int {
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
    user_data: *mut *mut c_void,
    _opts: *mut mosquitto_opt,
    _opt_count: c_int,
) -> c_int {
    let plugin = IdentityPlugin::new();

    log::debug!("mosquitto_auth_plugin_init :: identity plugin init");

    unsafe {
        *user_data = Box::into_raw(Box::new(plugin)) as *mut c_void;
    }

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
    user_data: *mut c_void,
    _opts: *mut mosquitto_opt,
    _opt_count: c_int,
) -> c_int {
    log::debug!("identity plugin cleanup");

    unsafe {
        Box::from_raw(user_data as *mut IdentityPlugin);
    }

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
    user_data: *mut c_void,
    opts: *mut mosquitto_opt,
    opt_count: c_int,
    _reload: bool,
) -> c_int {
    log::debug!("identity plugin security init");
    log::info!("mosquitto_auth_security_init :: get configurations...");

    let opts = unsafe { std::slice::from_raw_parts(opts, opt_count as usize) }
        .iter()
        .map(|option| {
            (
                unsafe { CStr::from_ptr(option.key) }.to_str().unwrap(),
                unsafe { CStr::from_ptr(option.value) }.to_str().unwrap(),
            )
        })
        .collect();

    let plugin = unsafe { &mut *(user_data as *mut IdentityPlugin) };

    match plugin.configs(opts) {
        Ok(_) => {
            log::info!("mosquitto_auth_security_init :: identity plugin configured")
        }
        Err(e) => {
            log::error!("{:?}", e);
        }
    }

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
    log::debug!("identity plugin security cleanup");
    MOSQ_SUCCESS
}
