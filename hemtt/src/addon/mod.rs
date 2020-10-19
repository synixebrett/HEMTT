use std::path::PathBuf;

mod location;
pub use location::AddonLocation;

use crate::HEMTTError;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Addon {
    pub name: String,
    pub location: AddonLocation,
}
impl Addon {
    pub fn new<S: Into<String>>(name: S, location: AddonLocation) -> Result<Self, HEMTTError> {
        Ok(Self {
            name: validate_name(name.into())?,
            location,
        })
    }

    pub fn locate<S: Into<String>>(name: S) -> Option<Self> {
        let name = name.into();
        for location in AddonLocation::first_class() {
            if location.exists() {
                let mut path = PathBuf::from(location);
                path.push(name.clone());
                if path.exists() {
                    return Some(Self { name, location });
                }
            }
        }
        None
    }

    /// Path to the addon folder
    pub fn source(&self) -> PathBuf {
        PathBuf::from(format!(
            "{}{}{}",
            self.location.to_string(),
            std::path::MAIN_SEPARATOR,
            self.name
        ))
    }

    /// Check if the addon exists
    pub fn exists(&self) -> bool {
        self.source().exists()
    }

    /// Filename of the PBO
    ///
    /// Arguments:
    /// * `prefix`: Prefix of the destination pbo
    ///             Some(prefix) => {prefix}_{self.name}.pbo
    ///             None => {self.name}.pbo
    pub fn pbo(&self, prefix: Option<&str>) -> String {
        if let Some(prefix) = prefix {
            format!("{}_{}.pbo", prefix, self.name)
        } else {
            format!("{}.pbo", self.name)
        }
    }

    /// Folder containing the released addon
    ///
    /// Arguments:
    /// * `destination_root`: root folder of the destination
    /// * `standalone`:
    ///                 Some(modname) - The destination should be it's own mod
    ///                 None - The destination is part of a larger mod
    pub fn destination_parent(
        &self,
        destination_root: &PathBuf,
        standalone: Option<&str>,
    ) -> PathBuf {
        let mut r = destination_root.clone();
        r.push(self.location.to_string());

        // Individual Mod
        if let Some(modname) = standalone {
            if self.location == AddonLocation::Addons {
                warn!("Standalone addons should be in optionals or compats");
            }
            r.push(&format!("@{}_{}", modname, self.name));
            r.push("addons");
        }

        r
    }

    /// File path of the released addon
    ///
    /// Arguments:
    /// * `destination_root`: root folder of the destination
    /// * `prefix`: Prefix of the destination pbo
    ///             Some(prefix) => {prefix}_{self.name}.pbo
    ///             None => {self.name}.pbo
    /// * `standalone`:
    ///                 Some(modname) - The destination should be it's own mod
    ///                 None - The destination is part of a larger mod
    pub fn destination(
        &self,
        destination_root: &PathBuf,
        prefix: Option<&str>,
        standalone: Option<&str>,
    ) -> PathBuf {
        let mut r = self.destination_parent(destination_root, standalone);
        r.push(self.pbo(prefix));
        r
    }
}

impl From<&Addon> for hemtt_handlebars::Variables {
    fn from(addon: &Addon) -> Self {
        use serde_json::{Map, Value};
        use std::collections::BTreeMap;
        Self({
            let mut map = BTreeMap::new();
            map.insert(
                String::from("addon"),
                Value::Object({
                    let mut map = Map::new();
                    map.insert(String::from("name"), Value::String(addon.name.clone()));
                    map.insert(
                        String::from("source"),
                        Value::String(addon.source().display().to_string()),
                    );
                    map
                }),
            );
            map
        })
    }
}

fn validate_name(name: String) -> Result<String, HEMTTError> {
    const STANDARD_CHARACTERS: [char; 27] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_',
    ];
    const ALLOWED_CHARACTERS: [char; 27] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-',
    ];

    for c in name.chars() {
        if !STANDARD_CHARACTERS.contains(&c) && !ALLOWED_CHARACTERS.contains(&c) {
            return Err(HEMTTError::AddonInvalidName(name));
        }
        if ALLOWED_CHARACTERS.contains(&c) {
            warn!("Invalid character `{}` in addon `{}`", c, name);
        }
    }
    Ok(name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    fn get_addon() -> super::Addon {
        super::Addon {
            name: "my_addon".to_string(),
            location: super::AddonLocation::Addons,
        }
    }
    fn get_optional() -> super::Addon {
        super::Addon {
            name: "my_addon".to_string(),
            location: super::AddonLocation::Optionals,
        }
    }
    fn get_compat() -> super::Addon {
        super::Addon {
            name: "my_addon".to_string(),
            location: super::AddonLocation::Compats,
        }
    }
    // fn get_custom() -> super::Addon {
    //     super::Addon {
    //         name: "my_addon".to_string(),
    //         location: super::AddonLocation::Custom("custom".to_string()),
    //     }
    // }

    #[test]
    fn source() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let addons: Vec<PathBuf> = addons.iter().map(|a| a.source()).collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("addons/my_addon"),
                PathBuf::from("optionals/my_addon"),
                PathBuf::from("compats/my_addon"),
                // PathBuf::from("custom/my_addon"),
            ]
        );
    }

    #[test]
    fn pbo_no_prefix() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let addons: Vec<String> = addons.iter().map(|a| a.pbo(None)).collect();
        assert_eq!(
            addons,
            vec![
                String::from("my_addon.pbo"),
                String::from("my_addon.pbo"),
                String::from("my_addon.pbo"),
                // String::from("my_addon.pbo"),
            ]
        );
    }

    #[test]
    fn pbo_with_prefix() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let addons: Vec<String> = addons.iter().map(|a| a.pbo(Some("prefix"))).collect();
        assert_eq!(
            addons,
            vec![
                String::from("prefix_my_addon.pbo"),
                String::from("prefix_my_addon.pbo"),
                String::from("prefix_my_addon.pbo"),
                // String::from("prefix_my_addon.pbo"),
            ]
        );
    }

    #[test]
    fn destination_parent_no_standalone() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let root = PathBuf::from("root");
        let addons: Vec<PathBuf> = addons
            .iter()
            .map(|a| a.destination_parent(&root, None))
            .collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("root/addons"),
                PathBuf::from("root/optionals"),
                PathBuf::from("root/compats"),
                // PathBuf::from("root/custom"),
            ]
        );
    }

    #[test]
    fn destination_parent_with_standalone() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let root = PathBuf::from("root");
        let addons: Vec<PathBuf> = addons
            .iter()
            .map(|a| a.destination_parent(&root, Some("standalone")))
            .collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("root/addons/@standalone_my_addon/addons"),
                PathBuf::from("root/optionals/@standalone_my_addon/addons"),
                PathBuf::from("root/compats/@standalone_my_addon/addons"),
                // PathBuf::from("root/custom/@standalone_my_addon/addons"),
            ]
        );
    }

    #[test]
    fn destination_no_prefix_no_standalone() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let root = PathBuf::from("root");
        let addons: Vec<PathBuf> = addons
            .iter()
            .map(|a| a.destination(&root, None, None))
            .collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("root/addons/my_addon.pbo"),
                PathBuf::from("root/optionals/my_addon.pbo"),
                PathBuf::from("root/compats/my_addon.pbo"),
                // PathBuf::from("root/custom/my_addon.pbo"),
            ]
        );
    }

    #[test]
    fn destination_no_prefix_with_standalone() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let root = PathBuf::from("root");
        let addons: Vec<PathBuf> = addons
            .iter()
            .map(|a| a.destination(&root, None, Some("standalone")))
            .collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("root/addons/@standalone_my_addon/addons/my_addon.pbo"),
                PathBuf::from("root/optionals/@standalone_my_addon/addons/my_addon.pbo"),
                PathBuf::from("root/compats/@standalone_my_addon/addons/my_addon.pbo"),
                // PathBuf::from("root/custom/@standalone_my_addon/addons/my_addon.pbo"),
            ]
        );
    }

    #[test]
    fn destination_with_prefix_no_standalone() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let root = PathBuf::from("root");
        let addons: Vec<PathBuf> = addons
            .iter()
            .map(|a| a.destination(&root, Some("prefix"), None))
            .collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("root/addons/prefix_my_addon.pbo"),
                PathBuf::from("root/optionals/prefix_my_addon.pbo"),
                PathBuf::from("root/compats/prefix_my_addon.pbo"),
                // PathBuf::from("root/custom/prefix_my_addon.pbo"),
            ]
        );
    }

    #[test]
    fn destination_with_prefix_with_standalone() {
        let addons = vec![get_addon(), get_optional(), get_compat()]; //, get_custom()];
        let root = PathBuf::from("root");
        let addons: Vec<PathBuf> = addons
            .iter()
            .map(|a| a.destination(&root, Some("prefix"), Some("standalone")))
            .collect();
        assert_eq!(
            addons,
            vec![
                PathBuf::from("root/addons/@standalone_my_addon/addons/prefix_my_addon.pbo"),
                PathBuf::from("root/optionals/@standalone_my_addon/addons/prefix_my_addon.pbo"),
                PathBuf::from("root/compats/@standalone_my_addon/addons/prefix_my_addon.pbo"),
                // PathBuf::from("root/custom/@standalone_my_addon/addons/prefix_my_addon.pbo"),
            ]
        );
    }
}
