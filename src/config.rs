use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomButton {
    pub label: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub dark_mode: bool,
    pub scale_factor: f32,
    pub cmd_template: String,
    pub window_width: f32,
    pub window_height: f32,
    pub font_size: f32,
    pub show_icons: bool,
    pub language: String,
    pub custom_buttons: Vec<CustomButton>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dark_mode: false,
            scale_factor: 1.0,
            cmd_template: "gnome-control-center {panel}".to_string(),
            window_width: 800.0,
            window_height: 480.0,
            font_size: 16.0,
            show_icons: true,
            language: "zh".to_string(),
            custom_buttons: vec![],
        }
    }
}

pub fn load_config(path: &PathBuf) -> AppConfig {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
            return config;
        }
    }
    AppConfig::default()
}

pub fn save_config(path: &PathBuf, config: &AppConfig) {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, json);
    }
}