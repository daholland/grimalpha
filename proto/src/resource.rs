use std::io::Cursor;
use std::path::{Path, PathBuf};
use ::util as util;
use std::collections::{HashMap, HashSet};
use ::uuid::Uuid;
use ::image as image;
use std::fs::File;

const NS_GRIMALPHA: &'static str = "89190388-3c77-57cb-b20e-a611d586005a";

pub struct ResourceManager {
    pub textures: TextureCache
}

pub type TextureId = Uuid;

pub fn new_resource_id(namespace: &str, name: &str) -> Uuid {
    let ns_ga = Uuid::parse_str(NS_GRIMALPHA).unwrap();

    let mut retstr = String::from(namespace);
    retstr.push_str(name);

    Uuid::new_v5(&ns_ga, retstr.as_str())
}



pub struct TextureCache {
    pub textures: HashMap<TextureId, Texture>,
    pub loaded_ids: HashSet<TextureId>
}

use glium::texture::{RawImage2d, PixelValue};

use std::fmt;

pub struct Texture {
    name: &'static str,
    id: Uuid,
    image: image::RgbaImage,
    size: usize
}
impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture {{name: {}, id: {}, image.len: {} }}", self.name, self.id, self.image.len())
    }
}
impl Texture {
    pub fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }

    pub fn into_glium_tex(&self) -> ::glium::texture::RawImage2d<u8> {
        ::glium::texture::RawImage2d::from_raw_rgba_reversed(self.image.clone().into_raw(), self.dimensions())
    }
}




use std::io::Error;

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            textures: HashMap::with_capacity(20),
            loaded_ids: HashSet::new()
        }
    }
    pub fn get_keys(&self) -> Vec<TextureId> {
        let mut idvec = Vec::new();

        for &key in self.textures.keys() {
            idvec.push(key);
        }

        idvec
    }

    pub fn get(&self, tex_id: &TextureId) -> Option<&Texture> {
        self.textures.get(&tex_id)
    }

    // grimalpha.nameless-software.com v5  89190388-3c77-57cb-b20e-a611d586005a
    //TODO: needs to be a Result, really
    pub fn load_image(&mut self, path: PathBuf) -> Option<TextureId> {
        use std::io::{Read, Seek, BufReader, Cursor};
        use ::glium::texture::*; 
        
        println!("path: {:?}", path);
        let name = "test";
        let tex_id = new_resource_id("textures/", name);
        let fin = File::open(path).unwrap();
        let fin = BufReader::new(fin);

        let image = image::load(fin, image::ImageFormat::PNG).unwrap().to_rgba();
        println!("image.len: {:?}", image.len());

        let texture = Texture {
            name: name,
            id: tex_id,
            image: image,
            size: 0

        };

        self.loaded_ids.insert(texture.id);
        self.textures.insert(texture.id, texture);
        println!("TextureCache: {:?}", self.textures);
        Some(tex_id)
    }
}

