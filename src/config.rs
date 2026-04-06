use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled, settings::Style};

use crate::OutputFormat;

const VALID_KEYS: &[&str] = &["api-key", "wallet-address", "chain", "format"];
const VALID_FORMATS: &[&str] = &["json", "text"];

fn config_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("bebop-cli")
}

fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "api-key", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "wallet-address", default, skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string(config_path()) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let _ = fs::create_dir_all(config_dir());
        let data = serde_json::to_string_pretty(self).expect("failed to serialize config");
        let _ = fs::write(config_path(), data);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match key {
            "api-key" => self.api_key.as_deref(),
            "wallet-address" => self.wallet_address.as_deref(),
            "chain" => self.chain.as_deref(),
            "format" => self.format.as_deref(),
            _ => None,
        }
    }

    fn set_value(&mut self, key: &str, value: String) {
        match key {
            "api-key" => self.api_key = Some(value),
            "wallet-address" => self.wallet_address = Some(value),
            "chain" => self.chain = Some(value),
            "format" => {
                if !VALID_FORMATS.contains(&value.as_str()) {
                    eprintln!("error: invalid format '{}', must be one of: {}", value, VALID_FORMATS.join(", "));
                    std::process::exit(1);
                }
                self.format = Some(value);
            }
            _ => unreachable!(),
        }
    }
}

fn validate_key(key: &str) {
    if !VALID_KEYS.contains(&key) {
        eprintln!("error: unknown config key '{}'", key);
        eprintln!("valid keys: {}", VALID_KEYS.join(", "));
        std::process::exit(1);
    }
}

pub fn show(key: Option<&str>, output: &OutputFormat) {
    let config = Config::load();

    if let Some(key) = key {
        validate_key(key);
        match config.get(key) {
            Some(value) => match output {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(
                        &serde_json::json!({ key: value })
                    ).unwrap());
                }
                OutputFormat::Text => println!("{}", value),
            },
            None => {
                eprintln!("error: '{}' is not configured", key);
                std::process::exit(1);
            }
        }
        return;
    }

    match output {
        OutputFormat::Json => {
            let out = serde_json::json!({
                "api-key": config.api_key,
                "wallet-address": config.wallet_address,
                "chain": config.chain,
                "format": config.format,
            });
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        OutputFormat::Text => {
            #[derive(Tabled)]
            struct Row {
                key: String,
                value: String,
            }
            let rows: Vec<Row> = VALID_KEYS.iter().map(|&k| Row {
                key: k.to_string(),
                value: config.get(k).unwrap_or("-").to_string(),
            }).collect();
            let mut table = Table::new(rows);
            table.with(Style::rounded());
            println!("{}", table);
        }
    }
}

pub fn set(key: &str, value: &str) {
    validate_key(key);
    let mut config = Config::load();
    config.set_value(key, value.to_string());
    config.save();
}

pub fn reset() {
    let config = Config::default();
    config.save();
}
