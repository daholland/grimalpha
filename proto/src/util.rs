extern crate toml;
extern crate rustc_serialize;

use std::fs::OpenOptions;
use std::io::prelude::*;
use self::toml::*;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
pub enum ConfigErrorKind {
    InvalidConfig
}

#[derive(Debug)]
pub struct ConfigError {
    kind: ConfigErrorKind,
    error: Box<Error + Send + Sync>
}

impl ConfigError {
    pub fn new<E>(kind: ConfigErrorKind, error: E) -> ConfigError
        where E: Into<Box<Error + Send + Sync>>
    {
        Self::_new(kind, error.into())
    }
    
    fn _new(kind: ConfigErrorKind, error: Box<Error + Send + Sync>) -> ConfigError {
        ConfigError {
            kind: kind,
            error: From::from(error),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ConfigErrorKind::InvalidConfig => write!(f, "Config error: {}", self.error.description()),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match self.kind {
            ConfigErrorKind::InvalidConfig => "error in config"
        }
    }

    fn cause(&self) -> Option<&Error> {
        self.error.cause()
    }
}



pub fn get_curr_dir() -> Option<PathBuf> {
    use std::env;

    env::current_dir().ok()
}
pub fn get_home_dir() -> Option<PathBuf> {
    use std::env;

    env::home_dir()
}

pub fn read_config() -> Result<config::Config, ConfigError> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("Proto.toml")
        .unwrap();

    let mut sval = String::new();
    let _ = file.read_to_string(&mut sval).unwrap();

    let mut parser = sval.as_str().parse::<Value>();

    let config = match parser {
        Ok(value) => value,
        Err(e) => {
            println!("Errors in toml parser: {:?}", e);
            panic!();
        }
    };

    let configtable = config.get("app_config").unwrap();
    let videoconf = config.get("video_config").unwrap();
    let currdir = get_curr_dir().unwrap();


    let mut ac = config::AppConfig {
        root_path: currdir.as_path()
            .join(configtable.get("root_path").unwrap().as_str().unwrap()),
        user_home_dir: currdir.as_path()
            .join(configtable.get("user_home_dir").unwrap().as_str().unwrap()),
        resource_path: currdir.as_path()
            .join(configtable.get("resource_path").unwrap().as_str().unwrap()),
        game_config: currdir.as_path()
            .join(configtable.get("game_config").unwrap().as_str().unwrap()),
    };

    if let Some("$HOME") = ac.user_home_dir.as_path().file_name().unwrap().to_str() {
        ac.user_home_dir = get_home_dir().unwrap();
    }

    let vc = config::VideoConfig {
        resolution: (videoconf.get("resolution").unwrap().get("x").unwrap().as_integer().unwrap(),
                     videoconf.get("resolution").unwrap().get("y").unwrap().as_integer().unwrap()),
    };
    println!("ac: {:?}\nvc: {:?}", ac, vc);


    Ok(config::Config {
        app_config: ac,
        video_config: vc,
    })
}

pub mod config {
    use std::path::PathBuf;

    #[derive(RustcDecodable, Debug)]
    pub struct Config {
        pub video_config: VideoConfig,
        pub app_config: AppConfig,
    }

    #[derive(RustcDecodable, Debug)]
    pub struct VideoConfig {
        pub resolution: (i64, i64),
    }

    impl VideoConfig {
        pub fn get_resolution(&self) -> (i64, i64) {
            self.resolution
        }
    }

    #[derive(RustcDecodable, Debug)]
    pub struct AppConfig {
        pub root_path: PathBuf,
        pub user_home_dir: PathBuf,
        pub resource_path: PathBuf,
        pub game_config: PathBuf,
    }
}
