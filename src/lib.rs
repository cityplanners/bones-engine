#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

mod engine;
pub mod ecs;

pub mod prelude {
    pub use crate::{
        ecs::*,
        engine::model::{
            Model,
            Mesh,
            Material,
            Instance
        }
    };
    pub use cgmath::{ Vector3, Quaternion, Deg };
    pub use cgmath::prelude::*;
}