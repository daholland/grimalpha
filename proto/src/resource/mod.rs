use std::io;
use std::num;
use std::error;
use std::result;
use std::option;
use std::convert;
use std::string;
use std::hash::{Hash, Hasher, SipHasher};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs::File;

use ::util;
use ::image;

use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::{SrgbTexture2d, RawImage2d, PixelValue};
use glium::program::{Program, Binary, ProgramCreationInput};
use std::fmt;

pub type ResourceId = u64;
pub type TextureId = u64;

pub type ResourceResult<T> = Result<T, ResourceError>;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash)]
pub enum ResourceNs {
    Texture,
    Shader
}

pub mod texture;
pub mod shader;

impl fmt::Display for ResourceNs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResourceNs::Texture => write!(f, "texture/"),
            ResourceNs::Shader => write!(f, "texture/"),
        }

   }
}

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
                         ResourceNs::Shader => write!(f, "Shader Error: {}", self.error.description()),
            // _ => write!(f, "Unknown Error: {}", self.error.description()),
        }
    }
}

impl error::Error for ResourceError {
    fn description(&self) -> &str {
        match self.kind {
            ResourceNs::Texture => "error in texture",
                         ResourceNs::Shader => "error in shader"
        }

    }

    fn cause(&self) -> Option<&error::Error> {
        self.error.cause()

    }
}


pub fn make_resource_id(namespace: ResourceNs, name: &str) -> ResourceId {
    let mut h = SipHasher::new();
    //println!("h: ", h);
    namespace.hash(&mut h);
    //println!("h: ", h);
    name.hash(&mut h);
    //println!("h: ", h);

    h.finish()
}

pub trait Resource {
    fn is_loaded(&self) -> bool;

    fn id(&self) -> ResourceId;

    fn path(&self) -> PathBuf;

    fn size(&self) -> usize;
}

pub struct ResourceManager {
    pub textures: texture::TextureCache,
    pub shaders: shader::ShaderCache,
}


impl ResourceManager {
    pub fn init() -> ResourceManager {
        ResourceManager { textures: texture::TextureCache::new(),
        shaders: shader::ShaderCache::new()}
    }

    pub fn get_raw_texture(&self, name: &str) -> ResourceResult<&SrgbTexture2d> {
        let tid = make_resource_id(ResourceNs::Texture, name);

        self.textures.get(&tid).map(|t| t.raw())
    }

    pub fn get_shader(&self, name: &str) -> ResourceResult<&Program> {
        let sid = make_resource_id(ResourceNs::Shader, name);

        self.shaders.get(&sid).map(|s| s)
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

