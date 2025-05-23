//! This module defines the `OSInfo` struct and related methods for representing and querying
//! operating system information in a platform-agnostic way.

use std::fmt::{self, Display, Formatter};

use super::{Version};

/// Represents information about an operating system, such as its ID, name, version, variant, edition, and codename.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OSInfo {
    /// Operating system identification.
    pub(crate) id: Option<String>,
    /// Operating system name.
    /// This is the name of the operating system as it is known to the user. It may be a marketing
    /// name or a more technical name.
    pub(crate) name: Option<String>,
    /// Operating system version. See `Version` for details.
    pub(crate) version: Version,
    /// Operating system variant.
    /// This is the variant of the operating system, such as "Server", "client", "Embedded", etc.
    /// It may be `None` if the variant is not known or not applicable.
    pub(crate) variant: Option<String>,
    /// Operating system edition.
    pub(crate) edition: Option<String>,
    /// Operating system codename.
    pub(crate) codename: Option<String>,
}

impl OSInfo {
    /// Constructs an `OSInfo` instance representing an unknown operating system.
    ///
    /// This function sets the `id` field to `"Unknown"`, the `name` field to an empty string,
    /// the `version` field to `Version::Unknown`, and all other fields to `None`.
    ///
    /// # Examples
    /// ```
    /// use osinfo::{Version, OSInfo};
    /// let info = OSInfo::unknown();
    /// assert_eq!(String::from("Unknown"), info.get_id());
    /// assert_eq!(String::new(), info.get_name());
    /// assert_eq!(Version::Unknown, info.get_version());
    /// ```
    pub fn unknown() -> Self {
        Self {
            id: Some(String::from("Unknown")),
            name: Some(String::new()),
            version: Version::Unknown,
            variant: None,
            edition: None,
            codename: None,
        }
    }

    /// Returns the operating system ID as a `String`.
    /// If the ID is not set, returns an empty string.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::unknown();
    /// assert_eq!(info.get_id(), "Unknown");
    /// ```
    pub fn get_id(&self) -> String {
        self.id.clone().unwrap_or_default()
    }
    
    /// Returns the operating system name as a `String`.
    /// If the name is not set, returns an empty string.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::unknown();
    /// assert_eq!(info.get_name(), "");
    /// ```
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    /// Returns the operating system version as a `Version`.
    ///
    /// # Example
    /// ```
    /// use osinfo::{Version, OSInfo};
    /// let info = OSInfo::unknown();
    /// assert_eq!(info.get_version(), Version::Unknown);
    /// ```
    pub fn get_version(&self) -> Version {
        self.version.clone()
    }

    /// Returns the operating system variant as a `String`.
    /// If the variant is not set, returns an empty string.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::unknown();
    /// assert_eq!(info.get_variant(), "");
    /// ```
    pub fn get_variant(&self) -> String {
        self.variant.clone().unwrap_or_default()
    }

    /// Returns the operating system edition as a `String`.
    /// If the edition is not set, returns an empty string.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::unknown();
    /// assert_eq!(info.get_edition(), "");
    /// ```
    pub fn get_edition(&self) -> String {
        self.edition.clone().unwrap_or_default()
    }

    /// Returns the operating system codename as a `String`.
    /// If the codename is not set, returns an empty string.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::unknown();
    /// assert_eq!(info.get_codename(), "");
    /// ```
    pub fn get_codename(&self) -> String {
        self.codename.clone().unwrap_or_default()
    }

    /// Constructs an `OSInfo` instance with the specified ID.
    /// All other fields are set to their default values.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::with_id("linux".to_string());
    /// assert_eq!(info.get_id(), "linux");
    /// ```
    pub fn with_id(id: String) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    /// Constructs an `OSInfo` instance with the specified name.
    /// All other fields are set to their default values.
    ///
    /// # Example
    /// ```
    /// use osinfo::OSInfo;
    /// let info = OSInfo::with_name("Ubuntu".to_string());
    /// assert_eq!(info.get_name(), "Ubuntu");
    /// ```
    pub fn with_name(name: String) -> Self {
        Self {
            name: Some(name),
            ..Default::default()
        }
    }

}

impl Default for OSInfo {
    fn default() -> Self {
        Self::unknown()
    }
}

impl Display for OSInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get_id())?;
        
        if let Some(ref name) = self.name {
            write!(f, " ({name})")?;
        }
        if let Some(ref variant) = self.variant {
            write!(f, " ({variant})")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unknown() {
        let info = OSInfo::unknown();
        assert_eq!(String::from("Unknown"), info.get_id());
        assert_eq!(String::new(), info.get_name());
        assert_eq!(Version::Unknown, info.get_version());
        assert_eq!(String::new(), info.get_variant());
        assert_eq!(String::new(), info.get_edition());
        assert_eq!(String::new(), info.get_codename());
    }

    #[test]
    fn default() {
        assert_eq!(OSInfo::default(), OSInfo::unknown());
    }

    #[test]
    fn with_id_sets_id() {
        let info = OSInfo::with_id("test_id".to_string());
        assert_eq!(info.get_id(), "test_id");
        assert_eq!(info.get_name(), "");
    }

    #[test]
    fn with_name_sets_name() {
        let info = OSInfo::with_name("TestOS".to_string());
        assert_eq!(info.get_name(), "TestOS");
        assert_eq!(info.get_id(), "Unknown");
    }

    #[test]
    fn display_format() {
        let mut info = OSInfo::with_id("linux".to_string());
        info.name = Some("Ubuntu".to_string());
        info.variant = Some("Server".to_string());
        let display = format!("{}", info);
        assert!(display.contains("linux"));
        assert!(display.contains("Ubuntu"));
        assert!(display.contains("Server"));
    }
}
