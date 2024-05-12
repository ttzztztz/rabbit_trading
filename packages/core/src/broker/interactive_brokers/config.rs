use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IBConfig {
    pub symbol_to_conid: HashMap<String, i64>,
}

impl IBConfig {
    pub fn new(path: String) -> Result<Self, Error> {
        let file = File::open(path.clone())?;
        serde_yaml_ng::from_reader(file)
            .with_context(|| format!("Error when reading config from path {}", path))
    }
}
