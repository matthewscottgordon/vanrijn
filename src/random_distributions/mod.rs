mod uniform_square;
pub use uniform_square::UniformSquare;

pub trait RandomDistribution<T> {
    fn value(&self) -> T;
}
