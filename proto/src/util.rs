extern crate toml;
extern crate rustc_serialize;

use std::fs::OpenOptions;
use std::io::prelude::*;
use self::toml::*;
use std::path::{Path, PathBuf};

pub fn get_curr_dir() -> Option<PathBuf> {
    use std::env;

    env::current_dir().ok()
}
pub fn get_home_dir() -> Option<PathBuf> {
    use std::env;

    env::home_dir()
}

pub fn read_config() -> Result<config::Config, Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("Proto.toml").unwrap();

    let mut sval = String::new();
    let _ = file.read_to_string(&mut sval);

    let mut parser  = toml::Parser::new(sval.as_str());

    let config = match parser.parse() {
        Some(value) => value,
        None => {
            println!("Errors in toml parser: {:?}", parser.errors);
            panic!();
        },
    };

    let configtable = config.get("app_config").unwrap();
    let videoconf = config.get("video_config").unwrap();
    let mut currdir = get_curr_dir().unwrap();


     let mut ac = config::AppConfig {
         root_path:     currdir.as_path().join(configtable.lookup("root_path").unwrap().as_str().unwrap()),
         user_home_dir:  currdir.as_path().join(configtable.lookup("user_home_dir").unwrap().as_str().unwrap()),
         resource_path:  currdir.as_path().join(configtable.lookup("resource_path").unwrap().as_str().unwrap()),
         game_config:    currdir.as_path().join(configtable.lookup("game_config").unwrap().as_str().unwrap()),
     };

    if let Some("$HOME") = ac.user_home_dir.as_path().file_name().unwrap().to_str() {
        ac.user_home_dir = get_home_dir().unwrap();
    }

    let vc = config::VideoConfig {
        resolution: (videoconf.lookup("resolution.x").unwrap().as_integer().unwrap(),
                     videoconf.lookup("resolution.y").unwrap().as_integer().unwrap())
    };
    println!("ac: {:?}\nvc: {:?}", ac, vc);
    

    Ok(config::Config {
        app_config: ac,
        video_config: vc
    })
    

    


}

pub mod config {
    use std::path::PathBuf;

    #[derive(RustcDecodable, Debug)]
    pub struct Config {
        pub video_config: VideoConfig,
        pub app_config: AppConfig
    }

    #[derive(RustcDecodable, Debug)]
    pub struct VideoConfig {
        pub resolution: (i64,i64)
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


