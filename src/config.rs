use std::{fs::File, io::Read, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub editor_cmd: String,
    pub editor_args: Vec<String>,
    pub indexes: bool,
    pub graph: bool,
}

impl Config {
    pub fn default() -> Self {
        Config {
            name: "My Zettelkasten".to_owned(),
            author: "Me".to_owned(),
            editor_cmd: "vim".to_owned(),
            editor_args: vec![],
            indexes: true,
            graph: true,
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let ser = serde_yaml::to_string(self)?;

        Ok(ser)
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;

        let mut ser = vec![];
        file.read_to_end(&mut ser)?;

        let cfg = serde_yaml::from_slice(&ser)?;

        Ok(cfg)
    }
}
