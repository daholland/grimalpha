use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
pub fn test () {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("test.txt").unwrap();
    println!("file: {:?}", file);
    write!(&mut file, "Test write file").unwrap();
    
}

