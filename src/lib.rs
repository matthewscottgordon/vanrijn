#![feature(external_doc, clamp)]
#![doc(include = "../README.md")]

pub mod accumulation_buffer;
mod camera;
pub mod colour;
pub mod image;
pub mod integrators;
pub mod materials;
pub mod math;
pub mod mesh;
pub mod raycasting;
pub mod realtype;
pub mod sampler;
pub mod scene;
pub mod util;

pub use camera::partial_render_scene;
