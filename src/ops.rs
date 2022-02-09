use crate::VersionIncrement;
use semver::Version;

// todo: unit tests
pub fn format_version(version: &Version) -> String {
    format!("{}.{}.{}", version.major, version.minor, version.patch)
}

pub fn increment_version_string(text: String, increment: VersionIncrement) -> Version {
    let version = match Version::parse(&text) {
        Ok(version) => version,
        Err(err) => panic!("error parsing invalid cargo version: {}", err),
    };
    increment_version(version, increment)
}

pub fn increment_version(prev_version: Version, increment: VersionIncrement) -> Version {
    let mut version = prev_version;
    match increment {
        VersionIncrement::Major => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
        VersionIncrement::Minor => {
            version.minor += 1;
            version.patch = 0;
        }
        VersionIncrement::Patch => {
            version.patch += 1;
        }
    };
    version
}

#[cfg(test)]
mod tests {
    use super::{increment_version, VersionIncrement};
    use semver::Version;

    #[test]
    fn increment_major() {
        let old_version = Version::new(1, 0, 1);
        let new_version = Version::new(2, 0, 0);
        assert_eq!(
            new_version,
            increment_version(old_version, VersionIncrement::Major)
        );
    }

    #[test]
    fn increment_minor() {
        let old_version = Version::new(1, 0, 1);
        let new_version = Version::new(1, 1, 0);
        assert_eq!(
            new_version,
            increment_version(old_version, VersionIncrement::Minor)
        );
    }

    #[test]
    fn increment_patch() {
        let old_version = Version::new(1, 0, 1);
        let new_version = Version::new(1, 0, 2);
        assert_eq!(
            new_version,
            increment_version(old_version, VersionIncrement::Patch)
        );
    }
}
