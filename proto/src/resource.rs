use std::io;
use std::num;
use std::error;
use std::result;
use std::option;
use std::convert;
use std::string;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs::File;

use ::util;
use ::uuid::Uuid;
use ::image;

use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::{Texture2d, RawImage2d, PixelValue};

use std::fmt;

pub type ResourceId = Uuid;
pub type TextureId = Uuid;

pub type ResourceResult<T> = Result<T, ResourceError>;

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum ResourceNs {
    Texture,
}

impl fmt::Display for ResourceNs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResourceNs::Texture => write!(f, "texture/"),
        }

    }
}

const NS_GRIMALPHA: &'static str = "89190388-3c77-57cb-b20e-a611d586005a";


#[derive(Debug)]
pub struct ResourceError {
    kind: ResourceNs,
    error: Box<error::Error + Send + Sync>,
}

impl ResourceError {
    pub fn new<E>(kind: ResourceNs, error: E) -> ResourceError
        where E: Into<Box<error::Error + Send + Sync>>
    {
        Self::_new(kind, error.into())
    }

    fn _new(kind: ResourceNs, error: Box<error::Error + Send + Sync>) -> ResourceError {
        ResourceError {
            kind: kind,
            error: From::from(error),
        }
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ResourceNs::Texture => write!(f, "Texture Error: {}", self.error.description()),
            // _ => write!(f, "Unknown Error: {}", self.error.description()),
        }
    }
}

impl error::Error for ResourceError {
    fn description(&self) -> &str {
        match self.kind {
            ResourceNs::Texture => "error in texture",
        }

    }

    fn cause(&self) -> Option<&error::Error> {
        self.error.cause()

    }
}


pub fn make_resource_id(namespace: ResourceNs, name: &str) -> Uuid {
    let ns_ga = Uuid::parse_str(NS_GRIMALPHA).unwrap();

    let mut retstr = namespace.to_string();
    retstr.push_str(name);

    Uuid::new_v5(&ns_ga, retstr.as_str())
}

pub trait Resource {
    fn is_loaded(&self) -> bool;

    fn id(&self) -> ResourceId;

    fn path(&self) -> PathBuf;

    fn size(&self) -> usize;
}

pub struct ResourceManager {
    pub textures: TextureCache,
}

impl ResourceManager {
    pub fn init() -> ResourceManager {
        ResourceManager { textures: TextureCache::new() }
    }

    pub fn get_raw_texture(&self, name: &str) -> ResourceResult<&Texture2d> {
        let tid = make_resource_id(ResourceNs::Texture, name);

        self.textures.get(&tid).map(|t| t.raw())

    }

    // pub fn get(&self, ns: ResourceNs, name: &str) -> Option<&Resource> {
    //     let tid = make_resource_id(ns, name);

    //     match ns {
    //         ResourceNs::Texture => {
    //             self.textures.get(&tid).ok()
    //         }
    //     }
    // }
}

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
    id: Uuid,
    data: Texture2d,
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

        let tex_id = make_resource_id(ResourceNs::Texture, self.name.as_str());

        let fin = File::open(self.path.clone()).unwrap();
        let fin = BufReader::new(fin);

        let image = image::load(fin, image::ImageFormat::PNG).unwrap().to_rgba();
        let dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), dimensions);


        self.data = Texture2d::new(display, image).unwrap();

        self.size = mem::size_of::<Self>();

        self.loaded = true;

        Ok(tex_id)
    }

    pub fn raw(&self) -> &Texture2d {
        &self.data
    }

    pub fn new(display: &GlutinFacade,
               name: &str,
               path: PathBuf,
               dimensions: (u32, u32))
               -> Texture {
        use std::mem;

        let tex_id = make_resource_id(ResourceNs::Texture, name);
        let size = mem::size_of::<Texture>();
        let image = ::glium::texture::Texture2d::empty(display, dimensions.0, dimensions.1)
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
