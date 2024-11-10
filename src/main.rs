mod error;
use error::{OsuToolError, Result};
use osu_db::listing::Listing;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs;
use std::env;

#[derive(Serialize, Deserialize)]
struct Config {
    osu_dir: String,
    lazer_dir: String,
}

impl Config {
    fn load() -> Result<Self> {
        let config_path = "config.toml";
        
        if !std::path::Path::new(config_path).exists() {
            let default_config = Config {
                osu_dir: String::new(),
                lazer_dir: String::new(),
            };
            let toml = toml::to_string_pretty(&default_config)?;
            fs::write(config_path, toml)?;
            return Ok(default_config);
        }

        let contents = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    fn save(&self) -> Result<()> {
        let toml = toml::to_string_pretty(self)?;
        fs::write("config.toml", toml)?;
        Ok(())
    }

    fn get_osu_stable_dir() -> String {
        if cfg!(target_os = "windows") {
            // Windows 路径
            if let Ok(appdata) = env::var("APPDATA") {
                return format!("{}\\Local\\osu!", appdata);
            }
        } else {
            // Linux/Unix 路径
            if let Ok(home) = env::var("HOME") {
                return format!("{}/.local/share/osu-wine/osu!", home);
            }
        }
        String::new()
    }

    fn get_osu_lazer_dir() -> String {
        if cfg!(target_os = "windows") {
            // Windows 路径
            if let Ok(appdata) = env::var("APPDATA") {
                return format!("{}\\osu", appdata);
            }
        } else {
            // Linux/Unix 路径
            if let Ok(home) = env::var("HOME") {
                return format!("{}/.local/share/osu", home);
            }
        }
        String::new()
    }
}

fn pause() {
    println!("按回车键继续...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
}

fn run() -> Result<()> {
    // Load config
    let mut config = Config::load()?;
    
    // Set default paths
    if config.osu_dir.is_empty() {
        config.osu_dir = Config::get_osu_stable_dir();
        if config.osu_dir.is_empty() {
            return Err(OsuToolError::ConfigError("无法获取默认 osu! 目录路径".into()));
        }
        println!("使用默认 osu! 路径: {}", config.osu_dir);
    }
    
    if config.lazer_dir.is_empty() {
        config.lazer_dir = Config::get_osu_lazer_dir();
        if config.lazer_dir.is_empty() {
            return Err(OsuToolError::ConfigError("无法获取默认 lazer 目录路径".into()));
        }
        println!("使用默认 lazer 路径: {}", config.lazer_dir);
    }
    
    // Check directories
    if !Path::new(&config.osu_dir).exists() {
        return Err(OsuToolError::DirectoryNotFound(config.osu_dir.clone()));
    }
    
    // Save config
    config.save()?;

    // Check osu!.db existence
    let osu_db_path = Path::new(&config.osu_dir).join("osu!.db");
    if !osu_db_path.exists() {
        return Err(OsuToolError::DatabaseNotFound(osu_db_path.display().to_string()));
    }

    // Load the listing
    let listing = Listing::from_file(&osu_db_path)
        .map_err(|e| OsuToolError::OsuDbError(e.to_string()))?;
    
    // Create output file
    let mut output_file = File::create("beatmap-hash-time.txt")?;
    
    // Write beatmap information
    for beatmap in listing.beatmaps.iter() {
        if let Some(hash) = &beatmap.hash {
            let formatted_time = beatmap.last_modified.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            writeln!(output_file, "{},{}", hash, formatted_time)?;
        }
    }

    println!("哈希值和时间已保存到 beatmap-hash-time.txt");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("错误: {}", err);
        match &err {
            OsuToolError::DatabaseNotFound(_) => {
                eprintln!("请确保 osu!.db 文件存在");
            }
            OsuToolError::DirectoryNotFound(_) => {
                eprintln!("请在 config.toml 中设置正确的目录路径");
            }
            _ => {}
        }
    }
    
    pause();
}