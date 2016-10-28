use super::{ResourceNs, ResourceResult, ResourceId,ResourceError, Resource};
use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};
use std::fmt;
use std::fs::File;

use ::util;

pub type ShaderId = u64;

pub type ShaderCache {
    shaders: HashMap<ShaderId, Shader>,
    pub loaded_ids: HashSet<ShaderId>
}

impl ShaderCache {
    pub fn new() -> ShaderCache {
        ShaderCache {
            shaders: Hashmap::with_capacity(20),
            loaded_ids: HashSet::new(),
        }

    }

    pub fn add(&mut self, k: ShaderId, v: ShaderId) -> ResourceResult<ShaderId> {
        self.shaders
            .insert(k, v)
            .ok_or_else(|| ResourceError::new(ResourceNs::Texture, "fail add shader".to_owned()))
            .map(|t| t.id)
    }

    pub fn get_keys(&self) -> Vec<ShaderId> {
        let mut idvec = Vec::new();

        for &key in self.shaders.keys() {
            idvec.push(key);
        }

        idvec
    }

    pub fn get(&self, shader_id: &ShaderId) -> ResourceResult<&Resource> {
        let s = self.shaders.get(shader_id);

        match t {
            Some(tex) => Ok(tex),
            None => {
                Err(ResourceError::new(ResourceNs::Shader, "error getting shader from cache".to_owned()))

            }
        }
    }
}

pub struct Shader {
    name: String,
    id: ShaderId,
    size: usize,
    data: (),
    path: PathBuf,
    loaded: bool
}


impl fmt::Debug for Shader {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "Shader {{name: {}, id: {}", self.name, self.id)
     }
}

impl Resource for Shader {
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

impl Shader {
    pub fn load() -> ResourceResult<ShaderId> {
        
    }
}

// pub trait Resource {
//     fn is_loaded(&self) -> bool;

//     fn id(&self) -> ResourceId;

//     fn path(&self) -> PathBuf;

//     fn size(&self) -> usize;
// }

