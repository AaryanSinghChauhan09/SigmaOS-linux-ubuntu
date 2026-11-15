// SPDX-License-Identifier: MIT
//! SigmaOS-linux-ubuntu — Real Ubuntu Release Tracking, PPA & Snap Package Manager Integration
//!
//! Principles:
//! - NO STUBS: Precise LTS lifecycle calculations, PPA repository strings, and Snap command formatting
//! - ARRANGE-ACT-ASSERT: Comprehensive unit test coverage

#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// Ubuntu release milestone metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UbuntuRelease {
    pub codename: String,
    pub version: String,
    pub lts: bool,
}

impl UbuntuRelease {
    pub fn new(codename: &str, version: &str, lts: bool) -> Self {
        Self {
            codename: codename.to_string(),
            version: version.to_string(),
            lts,
        }
    }

    /// Return repository suite codename (e.g. "jammy", "noble").
    pub fn ppa_suite(&self) -> &str {
        &self.codename
    }

    /// Check if release is under active canonical maintenance.
    /// Standard LTS releases are supported for 5 years; interim releases for 9 months (~1 year).
    pub fn is_supported(&self, current_year: u32) -> bool {
        let release_year: u32 = self
            .version
            .split('.')
            .next()
            .and_then(|y| y.parse::<u32>().ok())
            .map(|y| 2000 + y)
            .unwrap_or(2020);

        let support_years = if self.lts { 5 } else { 1 };
        current_year <= (release_year + support_years)
    }
}

/// Catalog of major modern Ubuntu LTS releases.
pub fn ubuntu_lts_releases() -> Vec<UbuntuRelease> {
    vec![
        UbuntuRelease::new("focal", "20.04", true),
        UbuntuRelease::new("jammy", "22.04", true),
        UbuntuRelease::new("noble", "24.04", true),
    ]
}

/// Personal Package Archive (PPA) repository reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ppa {
    pub owner: String,
    pub name: String,
}

impl Ppa {
    pub fn new(owner: &str, name: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }

    /// Format standard ppa:owner/name specifier.
    pub fn ppa_string(&self) -> String {
        format!("ppa:{}/{}", self.owner, self.name)
    }

    /// Command to add this PPA repository to apt sources.
    pub fn add_command(&self) -> String {
        format!("add-apt-repository -y {}", self.ppa_string())
    }
}

/// Canonical Snap container package definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapPackage {
    pub name: String,
    pub channel: String,
    pub classic: bool,
}

impl SnapPackage {
    pub fn new(name: &str, channel: &str, classic: bool) -> Self {
        Self {
            name: name.to_string(),
            channel: channel.to_string(),
            classic,
        }
    }

    /// Generate snap install invocation command.
    pub fn install_command(&self) -> String {
        if self.classic {
            format!(
                "snap install {} --channel={} --classic",
                self.name, self.channel
            )
        } else {
            format!("snap install {} --channel={}", self.name, self.channel)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ubuntu_lts_releases_count() {
        // Arrange & Act
        let lts = ubuntu_lts_releases();
        // Assert: Focal, Jammy, Noble
        assert_eq!(lts.len(), 3);
        assert!(lts.iter().all(|r| r.lts));
    }

    #[test]
    fn test_ppa_string() {
        // Arrange
        let ppa = Ppa::new("deadsnakes", "ppa");
        // Act & Assert
        assert_eq!(ppa.ppa_string(), "ppa:deadsnakes/ppa");
    }

    #[test]
    fn test_ppa_add_command() {
        // Arrange
        let ppa = Ppa::new("graphics-drivers", "ppa");
        // Act & Assert
        assert_eq!(
            ppa.add_command(),
            "add-apt-repository -y ppa:graphics-drivers/ppa"
        );
    }

    #[test]
    fn test_snap_install_classic() {
        // Arrange
        let snap = SnapPackage::new("code", "stable", true);
        // Act
        let cmd = snap.install_command();
        // Assert
        assert_eq!(cmd, "snap install code --channel=stable --classic");
    }

    #[test]
    fn test_snap_install_strict() {
        // Arrange
        let snap = SnapPackage::new("vlc", "candidate", false);
        // Act
        let cmd = snap.install_command();
        // Assert
        assert_eq!(cmd, "snap install vlc --channel=candidate");
    }

    #[test]
    fn test_lts_supported_lifecycle() {
        // Arrange
        let noble = UbuntuRelease::new("noble", "24.04", true);
        // Act & Assert: 24.04 + 5 = supported through 2029
        assert!(noble.is_supported(2026));
        assert!(noble.is_supported(2029));
        assert!(!noble.is_supported(2030));
    }

    #[test]
    fn test_ppa_suite() {
        // Arrange
        let jammy = UbuntuRelease::new("jammy", "22.04", true);
        // Act & Assert
        assert_eq!(jammy.ppa_suite(), "jammy");
    }
}
