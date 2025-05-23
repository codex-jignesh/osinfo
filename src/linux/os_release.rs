// spell-checker:ignore sles, AOSCOS

use std::{fmt, fs::File, io::Read, path::Path};

use log::{trace, warn};

use crate::{matcher::Matcher, OSInfo, Version};

pub fn get_os_data() -> Option<OSInfo> {
    retrieve(&DISTRIBUTIONS, "/")
}

fn retrieve(distributions: &[ReleaseInfo], root: &str) -> Option<OSInfo> {
    for release_info in distributions {
        let path = Path::new(root).join(release_info.path);
        
        if !path.exists() {
            trace!("Path '{}' doesn't exist", release_info.path);
            continue;
        }

        let mut file = match File::open(&path) {
            Ok(val) => val,
            Err(e) => {
                warn!("Unable to open {:?} file: {:?}", &path, e);
                continue;
            }
        };

        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            warn!("Unable to read {:?} file: {:?}", &path, e);
            continue;
        }

        let id = (release_info.id)(&file_content);
        let name = (release_info.name)(&file_content);
        let variant = (release_info.variant)(&file_content);
        let version = (release_info.version)(&file_content);
        let codename = (release_info.codename)(&file_content);
        // If id is indeterminate, try the next release_info
        if id.is_none() {
            continue;
        }

        //let version = (release_info.version)(&file_content);

        return Some(OSInfo {
            // Unwrap is OK here because of the `id.is_none()` check above.
            id: id,
            name: name,
            variant: variant,
            version: version.unwrap_or(Version::Unknown),
            codename: codename,
            //bitness: Bitness::Unknown,
            ..Default::default()
        });
    }

    // Failed to determine os info
    None
}

/// Struct containing information on how to parse distribution info from a release file.
#[derive(Clone)]
struct ReleaseInfo<'a> {
    /// Relative path to the release file this struct corresponds to from root.
    path: &'a str,

    /// A closure that determines the os id from the release file contents.
    id: for<'b> fn(&'b str) -> Option<String>,
    /// A closure that determines the os name from the release file contents.
    name: for<'b> fn(&'b str) -> Option<String>,
    /// A closure that determines the os version from the release file contents.
    version: for<'b> fn(&'b str) -> Option<Version>,

    /// A closure that determines the os variant from the release file contents.
    variant: for<'b> fn(&'b str) -> Option<String>,
    /// A closure that determines the os codename from the release file contents.
    codename: for<'b> fn(&'b str) -> Option<String>,
}

impl fmt::Debug for ReleaseInfo<'_> {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReleaseInfo")
            .field("path", &self.path)
            .field("id", &(self.id as fn(&'a str) -> Option<String>))
            .field("name", &(self.name as fn(&'a str) -> Option<String>))
            .field("version", &(self.version as fn(&'a str) -> Option<Version>))
            .field("variant", &(self.variant as fn(&'a str) -> Option<String>))
            .field("codename", &(self.codename as fn(&'a str) -> Option<String>))
            .finish()
    }
}

/// List of all supported distributions and the information on how to parse their version from the
/// release file.
static DISTRIBUTIONS: [ReleaseInfo; 1] = [
    // Keep this first; most modern distributions have this file.
    ReleaseInfo {
        path: "etc/os-release",
        id: |release| {
            Matcher::KeyValue { key: "ID" }
                .find(release)
        },
        name: |name| {
            Matcher::KeyValue { key: "NAME" }
                .find(name)
        },
        version: |version| {
            Matcher::KeyValue { key: "VERSION_ID" }
                .find(version)
                .map(Version::from_string)
        },
        variant: |variant| {
            Matcher::KeyValue { key: "VARIANT_ID" }
                .find(variant)
                .map(|v| v.to_string())
                .or_else(|| {Some("client".to_string())})
        },
        codename: |codename| {
            let version_codename = Matcher::KeyValue { key: "VERSION_CODENAME" }
                .find(codename);
            match version_codename {
                Some(v) => Some(v.to_string()),
                None => {
                    let version = Matcher::KeyValue { key: "VERSION" }
                        .find(codename);
                    if let Some(v) = version {
                        Matcher::Between { start: '(', end: ')' }
                            .find(&v)
                            .map(|v| v.to_string())
                    } else {
                        None
                    }
                }
            } 
        },
    },
];

