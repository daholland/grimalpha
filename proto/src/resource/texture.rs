use super::{ResourceNs, ResourceResult, ResourceId,ResourceError, Resource};
use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};
use std::fmt;
use std::fs::File;
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::{SrgbTexture2d,Texture2d, RawImage2d, PixelValue};


use ::util;
use ::uuid::Uuid;
use ::image;

pub type TextureId = u64;

pub struct TextureCache {
    textures: HashMap<TextureId, Texture>,
    pub loaded_ids: HashSet<TextureId>,
}

impl TextureCache {
    pub fn new() -> TextureCache {
        TextureCache {
            textures: HashMap::with_capacity(20),
            loaded_ids: HashSet::new(),
        }
    }

    pub fn add(&mut self, k: TextureId, v: Texture) -> ResourceResult<TextureId> {
        self.textures
            .insert(k, v)
            .ok_or_else(|| ResourceError::new(ResourceNs::Texture, "fail add texture".to_owned()))
            .map(|t| t.id)
    }

    pub fn get_keys(&self) -> Vec<TextureId> {
        let mut idvec = Vec::new();

        for &key in self.textures.keys() {
            idvec.push(key);
        }

        idvec
    }

    pub fn get(&self, tex_id: &TextureId) -> ResourceResult<&Texture> {
        let t = self.textures.get(tex_id);

        match t {
            Some(tex) => Ok(tex),
            None => {
                Err(ResourceError::new(ResourceNs::Texture,
                                       "error getting texture from cache".to_owned()))
            }
        }


    }
}


pub struct Texture {
    name: String,
    id: TextureId,
    data: SrgbTexture2d,
    size: usize,
    dimensions: (u32, u32),
    path: PathBuf,
    loaded: bool,
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture {{name: {}, id: {}", self.name, self.id)
    }
}

impl Resource for Texture {
    fn is_loaded(&self) -> bool {
        self.loaded
    }

    fn id(&self) -> ResourceId {
        self.id
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn size(&self) -> usize {
        self.size
    }
}


impl Texture {
    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn load(&mut self, display: &GlutinFacade) -> ResourceResult<TextureId> {

        use std::mem;
        use std::io::BufReader;
        use ::glium::texture::*;

        let tex_id = super::make_resource_id(ResourceNs::Texture, self.name.as_str());

        let fin = File::open(self.path.clone()).unwrap();
        let fin = BufReader::new(fin);

        let image = image::load(fin, image::ImageFormat::PNG).unwrap();
        println!("image: color: {:?}", image.color());

        let image = image.to_rgba();
        let dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), dimensions);


        self.data = SrgbTexture2d::new(display, image).unwrap();

        self.size = mem::size_of::<Self>();

        self.loaded = true;

        Ok(tex_id)
    }

    pub fn raw(&self) -> &SrgbTexture2d {
        &self.data
    }

    pub fn new(display: &GlutinFacade,
               name: &str,
               path: PathBuf,
               dimensions: (u32, u32))
               -> Texture {
        use std::mem;

        let tex_id = super::make_resource_id(ResourceNs::Texture, name);
        let size = mem::size_of::<Texture>();
        let image = ::glium::texture::SrgbTexture2d::empty(display, dimensions.0, dimensions.1)
            .unwrap();
        Texture {
            name: name.to_owned(),
            id: tex_id,
            data: image,
            size: size,
            dimensions: dimensions,
            path: path,
            loaded: false,
        }

    }
}
