mod api;

use log::trace;

use crate::OSInfo;

pub fn get_info() -> OSInfo {
    trace!("windows::get_info is called");
    let info = api::get_os_data();
    trace!("Returning {:?}", info);
    info
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn windows() {
        let info = api::get_os_data();
        assert_eq!(String::from("windows"), info.get_id());
        assert!(info.get_name().contains("Windows"));
    }
}
