use super::colour::Photon;
use super::raycasting::IntersectionInfo;
use super::sampler::Sampler;

mod whitted_integrator;
pub use whitted_integrator::*;

mod simple_random_integrator;
pub use simple_random_integrator::*;

pub trait Integrator {
    fn integrate(
        &self,
        sampler: &Sampler,
        info: &IntersectionInfo,
        photon: &Photon,
        recursion_limit: u16,
    ) -> Photon;
}
