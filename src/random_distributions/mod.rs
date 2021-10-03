mod uniform_square;
pub use uniform_square::UniformSquare;

mod unit_disc;
pub use unit_disc::UnitDisc;

pub trait RandomDistribution<T> {
    fn value(&self) -> T;
}
