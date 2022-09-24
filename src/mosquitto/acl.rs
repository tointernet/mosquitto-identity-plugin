use super::defs::{mosquitto, mosquitto_acl_msg, MOSQ_SUCCESS};
use std::os::raw::{c_int, c_void};

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
/// MOSQ_ERR_SUCCESS: if access was granted.  MOSQ_ERR_ACL_DENIED: if access was not granted.  MOSQ_ERR_UNKNOWN: for an application specific
/// error.  MOSQ_ERR_PLUGIN_DEFER: if your plugin does not wish to handle this check.
#[no_mangle]
extern "C" fn mosquitto_auth_acl_check(
    _user_data: *mut c_void,
    _access: c_int,
    _client: *mut mosquitto,
    _msg: *const mosquitto_acl_msg,
) -> c_int {
    println!("called_from_rust::mosquitto_auth_acl_check");
    MOSQ_SUCCESS
}
