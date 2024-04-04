use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use toml_edit::{value, DocumentMut};

use super::{Config, ParseAvaliable};

pub const FILENAME: &'static str = "pyproject.toml";

#[derive(Debug)]
pub struct Property {
    pub filepath: PathBuf,
    pub doc: DocumentMut,
}

impl ParseAvaliable for Property {
    fn new(root: &PathBuf) -> Result<Self> {
        let filepath = root.join(FILENAME);
        if !filepath.exists() {
            return Err(anyhow!("Configuration file is not found."));
        }
        let source = read_to_string(&filepath);
        if source.is_err() {
            return Err(anyhow!("Configuration file cannot access."));
        }
        let doc = source.unwrap().parse::<DocumentMut>();
        if doc.is_err() {
            return Err(anyhow!("Configuration is not valid TOML."));
        }

        Ok(Property {
            filepath,
            doc: doc.unwrap(),
        })
    }

    fn get_config(&self) -> Result<Config> {
        let mut item = self.doc.as_item();
        for k in ["tool", "age"] {
            let child = item.get(k);
            if child.is_none() || !child.unwrap().is_table() {
                return Err(anyhow!("It does not have valid values."));
            }
            item = child.unwrap();
        }
        let config = toml::from_str::<Config>(item.to_string().as_str());
        if config.is_err() {
            return Err(anyhow!(config.unwrap_err()));
        }
        Ok(config.unwrap())
    }

    fn update_version(&mut self, version: &semver::Version) -> Result<()> {
        self.doc["tool"]["age"]["current_version"] = value(version.to_string());
        let mut out = File::create(&self.filepath)?;
        let _ = out.write(self.doc.to_string().as_bytes());
        Ok(())
    }
}
