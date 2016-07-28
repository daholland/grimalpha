extern crate toml;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use self::toml::*;

pub fn test () {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("test.txt").unwrap();
    println!("file: {:?}", file);
    write!(&mut file, "Test write file").unwrap();
}

pub fn testoml() {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("Cargo.toml").unwrap();
    println!("file: {:?}", file);
    let mut sbuf = String::new();
    file.read_to_string(&mut sbuf);
    let value = toml::Parser::new(sbuf.as_str()).parse().unwrap();
    println!("{:?}", value);

}



