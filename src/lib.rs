#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod osimp;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod osimp;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod osimp;


mod os_info;
mod version;
mod matcher;

pub use crate::{os_info::OSInfo, version::Version, matcher::Matcher};

/// Returns information about the current operating system (id, name, version, variant, edition, codename).
/// 
/// - codename on Windows will be display version string e.g 22H2, 23H1 etc and on Linux it will be the codename of the distribution.
///
/// - variant will be server / client 
///
/// # Examples
///
/// ```
/// use osinfo;
///
/// let info = osinfo::get();
///
/// // Print full information:
/// println!("OS information: {info}");
///
/// // Print information separately:
/// println!("ID: {}", info.get_id());
/// println!("Name: {}", info.get_name());
/// println!("Version: {}", info.get_version());
/// println!("Variant: {}", info.get_variant());
/// println!("Edition: {}", info.get_edition());
/// println!("Codename: {}", info.get_codename());
/// ```
pub fn get() -> OSInfo {
    osimp::get_info()
}
