use std::ops::Deref;

use serde::{Deserialize, Serialize};

pub type CargoDeps = cargo_toml::DepsSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CargoToml(cargo_toml::Manifest);

impl Deref for CargoToml {
    type Target = cargo_toml::Manifest;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CargoToml {
    pub fn package_name(&self) -> &str {
        self.0
            .package
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or("app")
    }
}
