#![allow(unsafe_code)]
use crate::{OSInfo, Version};
use winreg::{RegKey, enums::*};

pub fn get_os_data() -> OSInfo {
    current_version_from_reg()
}


fn current_version_from_reg() -> OSInfo {
    let mut os_info = OSInfo {
        id: Some(String::from("windows")),
        ..Default::default()
    };
    let current_version = get_registry(RegKey::predef(HKEY_LOCAL_MACHINE), "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    match current_version {
        Ok(current_version) => {
            os_info.version = get_version(&current_version);

            os_info.name = get_registry_value(&current_version, "ProductName");
            os_info.variant = get_registry_value(&current_version, "InstallationType");
            os_info.edition = get_registry_value(&current_version, "EditionID");
            os_info.codename = get_registry_value(&current_version, "DisplayVersion");
        },

        Err(e) => {
            log::error!("Failed to get registry key: {}", e);
        }
        
    }
    
    os_info
}

fn get_registry(reg_root: RegKey, path: &str) -> std::io::Result<RegKey> {
    reg_root.open_subkey(path)
}
fn get_version(reg_key: &RegKey) -> Version {
    let major = reg_key.get_value::<u32, _>("CurrentMajorVersionNumber").unwrap_or_default();
    let minor = reg_key.get_value::<u32, _>("CurrentMinorVersionNumber").unwrap_or_default();
    let build = reg_key.get_value::<String, _>("CurrentBuildNumber").unwrap_or_default().parse::<u32>().unwrap_or_default();
    let ubr = reg_key.get_value::<u32, _>("UBR").unwrap_or_default();

    Version::Semantic(
        major,
        minor,
        build,
        ubr,
    )
}

fn get_registry_value(reg_key: &RegKey, name: &str) -> Option<String> {
    if let Ok(value) = reg_key.get_value::<String, _>(name) {
        Some(value)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn windows() {
        let info = get_os_data();
        assert_eq!(String::from("windows"), info.get_id());
        assert!(info.get_name().contains("Windows"));
    }
}