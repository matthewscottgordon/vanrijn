mod uniform_square;
pub use uniform_square::UniformSquare;

mod unit_disc;
pub use unit_disc::UnitDisc;

mod uniform_hemisphere;
pub use uniform_hemisphere::UniformHemisphere;

pub trait RandomDistribution<T> {
    fn value(&self) -> T;
}
