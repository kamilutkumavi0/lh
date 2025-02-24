//! reads user configiration for prints outpu handsomely
mod default;
use serde_derive::Deserialize;
use std::collections::HashMap;
use toml::from_str;
// use std::env;
use home::home_dir;
use std::fs;

/// Selection of the font format of output.
#[derive(Deserialize, Debug, Clone)]
pub enum FontFormat {
    Bold,
    Italic,
    Regular,
}

///Selection of the color of output.
#[derive(Deserialize, Debug, Clone)]
pub enum ColorFormat {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Color and symbol of the file
#[derive(Deserialize, Debug, Clone)]
pub struct FileTypeToml {
    // File type name
    pub name: String,
    // Symbol of the file .
    pub symbol: String,
    // Formated color of the file type.
    pub color: ColorFormat,
    // Formated background color of the file type.
    pub bg_color: Option<ColorFormat>,
    // Formated font (italic, bold, regular) of the file type/
    pub font: FontFormat,
    // Exention tracking for the file type.
    pub track: Vec<String>,
}

impl FileTypeToml {
    pub fn new(
        name: String,
        symbol: String,
        color: ColorFormat,
        bg_color: Option<ColorFormat>,
        font: FontFormat,
        track: Vec<String>,
    ) -> Self {
        Self {
            name,
            symbol,
            color,
            bg_color,
            font,
            track,
        }
    }
}
/// All config of user in progress
#[derive(Deserialize, Debug)]
pub struct Config {
    // General custom settings like wanna see logos or not
    // Configiration parser of the cli app.
    pub file_type: Vec<FileTypeToml>,
}

/// Tract conf to hash table for easy to use for filtering the output.
fn track_hash(config: &Config) -> HashMap<String, FileTypeToml> {
    // conf_hash is a hash table for detect file extentions.
    let mut conf_hash: HashMap<String, FileTypeToml> = default::creat_default();

    // All file type's exention as a key file format as value
    // in config parserer inserts in conf_hash.
    for file_types in &config.file_type {
        for tracks in &file_types.track {
            //println!("{tracks}");
            if let Some(config_type) = conf_hash.get_mut(tracks) {
                *config_type = file_types.clone();
            } else {
                conf_hash.insert(tracks.to_string(), file_types.clone());
            }
        }
    }
    conf_hash
}
///read lh.toml or uses the defaut toml file.
pub fn toml_read() -> HashMap<String, FileTypeToml> {
    let home_diroctory = home_dir();
    let config: Option<String> = match home_diroctory {
        Some(dir) => {
            let mut new_dir = dir.as_os_str().to_str().unwrap().to_string();
            new_dir.push_str("/.config/lh.toml");
            match fs::read_to_string(new_dir) {
                Ok(f) => Some(f),
                Err(_) => None,
            }
        }
        None => None,
    };
    match config {
        Some(conf_str) => match from_str(&conf_str) {
            Ok(config) => track_hash(&config),
            Err(_) => default::creat_default(),
        },
        None => default::creat_default(),
    }
}
