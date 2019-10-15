use super::Project;

use crate::HEMTTError;

impl Project {
    /// Should the private key be saved to disk and reused for future versions
    pub fn reuse_private_key(&self) -> bool {
        self.reuse_private_key.is_some() && self.reuse_private_key.unwrap()
    }

    /// Get the name for .bikey & .biprivatekey files
    pub fn get_key_name(&self) -> Result<String, HEMTTError> {
        Ok(if self.key_name.is_empty() {
            if self.reuse_private_key() {
                self.prefix.clone()
            } else if self.prefix.is_empty() {
                self.version()?
            } else {
                format!("{}_{}", &self.prefix, &self.version()?)
            }
        } else {
            self.render(&self.key_name, Some("project:key_name"))?
        })
    }

    /// Get the name for .bisign files
    pub fn get_sig_name(&self, pbo: &str) -> Result<String, HEMTTError> {
        Ok(if self.sig_name.is_empty() {
            format!("{}_{}.pbo.{}.bisign", &self.prefix, pbo, self.version()?)
        } else {
            format!(
                "{}_{}.pbo.{}.bisign",
                &self.prefix,
                pbo,
                self.render(&self.sig_name, Some("project:sig_name"))?
            )
        })
    }

    fn match_ver(&self, v: u8) -> armake2::BISignVersion {
        match v {
            3 => armake2::BISignVersion::V3,
            2 => armake2::BISignVersion::V2,
            _ => {
                warn!(format!("Invalid Sig Version `{}`", v));
                self.match_ver(crate::project::default_sig_version())
            }
        }
    }

    /// BISignVersion to use for signing
    pub fn get_sig_version(&self) -> armake2::BISignVersion {
        self.match_ver(self.sig_version)
    }
}
