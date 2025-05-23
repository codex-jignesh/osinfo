mod os_release;

use log::trace;

use crate::OSInfo;

pub fn get_info() -> OSInfo {
    trace!("Linux::get_info is called");
    let info = os_release::get_os_data();
    trace!("Returning {:?}", info);
    info.unwrap_or_default()
}

