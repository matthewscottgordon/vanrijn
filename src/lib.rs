#![feature(external_doc)]
#![doc(include = "../README.md")]

mod camera;
pub mod colour;
pub mod image;
pub mod integrators;
pub mod materials;
pub mod mesh;
/// Core raycasting and geometry primitives
pub mod raycasting;
pub mod realtype;
pub mod sampler;
pub mod scene;
pub mod util;

pub use camera::partial_render_scene;

use realtype::Real;

