use super::{ResourceNs, ResourceResult, ResourceId,ResourceError, Resource};
use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fmt;
use std::fs::File;
use glium::backend::glutin_backend::GlutinFacade;
use glium::backend::Facade;

use std::rc::Rc;
use glium::program::{Program, Binary, ProgramCreationInput};


use ::util;

pub type ShaderId = u64;

pub struct ShaderCache {
    pub shaders: HashMap<ShaderId, Rc<Program>>,
    pub loaded_ids: HashSet<ShaderId>
}

impl ShaderCache {
    pub fn new() -> ShaderCache {
        ShaderCache {
            shaders: HashMap::with_capacity(20),
            loaded_ids: HashSet::new(),
        }

    }

    pub fn add(&mut self, k: ShaderId, v: Program) -> ResourceResult<ShaderId> {
        self.shaders
            .insert(k,Rc::new(v))
            .ok_or_else(|| ResourceError::new(ResourceNs::Shader, "fail add shader".to_owned()))
            .map(|t| k)
    }

    pub fn get_keys(&self) -> Vec<ShaderId> {
        let mut idvec = Vec::new();

        for &key in self.shaders.keys() {
            idvec.push(key);
        }

        idvec
    }

    pub fn get(&self, shader_id: &ShaderId) -> ResourceResult<&Program> {
        let s = self.shaders.get(shader_id);

        match s {
            Some(tex) => Ok(tex),
            None => {
                Err(ResourceError::new(ResourceNs::Shader, "error getting shader from cache".to_owned()))

            }
        }
    }
}


#[derive(Clone)]
pub struct Shader {
    name: String,
    id: ShaderId,
    size: usize,
    pub data: ShaderSource,
    path: ShaderPath,
    version: f32,
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
        self.path.vert.clone()
    }

    fn size(&self) -> usize {
        self.size
    }

}

#[derive(Clone)]
pub struct ShaderPath{
    vert: PathBuf,
    frag: PathBuf
}
#[derive(Clone)]
pub struct ShaderSource {
    pub vert: String,
    pub frag: String,
    pub geom: Option<String>
}

impl Shader {
    pub fn load(&mut self, display: &GlutinFacade) -> ResourceResult<ShaderId> {
        use std::mem;
        use std::io::BufReader;

        let shader_id = super::make_resource_id(ResourceNs::Shader, &self.name);

        self.size = mem::size_of::<Self>();
        self.loaded = true;

        Ok(shader_id)
    }

    pub fn raw(&self) -> &ShaderSource {
        
        &self.data
        

    }

    pub fn new(display: &GlutinFacade,
               name: &str,
               path: PathBuf) -> Shader {
        use std::mem;
        let shader_id = super::make_resource_id(ResourceNs::Shader, name);
        let size = mem::size_of::<Shader>();

        let mut pathvs = PathBuf::from(path.clone());
        pathvs.push("330.vert");
        let mut pathfs = PathBuf::from(path.clone());
        pathfs.push("330.frag");

let mut vs = String::new();
let mut vf = File::open(pathvs.clone()).unwrap();
vf.read_to_string(&mut vs).unwrap();

        let mut fs = String::new();
        let mut ff = File::open(pathfs.clone()).unwrap();
        ff.read_to_string(&mut fs).unwrap();

        

        Shader {
            name: name.to_owned(),
            id: shader_id,
            version: 3.3,
            data: ShaderSource{vert: vs, frag: fs,geom: None},
            size: size,
            path: ShaderPath {vert: pathvs, frag: pathfs},
            loaded: false
        }
    }
}
                                                                     

// pub trait Resource {
//     fn is_loaded(&self) -> bool;

//     fn id(&self) -> ResourceId;

//     fn path(&self) -> PathBuf;

//     fn size(&self) -> usize;
// }

