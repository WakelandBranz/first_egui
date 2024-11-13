/// All feature settings are stored in here. Menu state is stored by egui persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use crate::features::*;
use super::features::visuals::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub features: Features,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            features: Features::default(),
        }
    }
}

impl Config {
    const CONFIG_PATH: &'static str = "config.json";

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(Self::CONFIG_PATH, json)
            .context("Failed to write config file")?;

        log::debug!("Config saved successfully to {}", Self::CONFIG_PATH);
        Ok(())
    }

    pub fn load() -> Result<Self> {
        if !Path::new(Self::CONFIG_PATH).exists() {
            log::debug!("No config file found, creating default config");
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        let json = fs::read_to_string(Self::CONFIG_PATH)
            .context("Failed to read config file")?;

        let config = serde_json::from_str(&json)
            .context("Failed to deserialize config")?;

        log::debug!("Config loaded successfully from {}", Self::CONFIG_PATH);
        Ok(config)
    }
}