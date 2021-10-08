mod uniform_square;
pub use uniform_square::UniformSquare;

mod unit_disc;
pub use unit_disc::UnitDisc;

mod uniform_hemisphere;
pub use uniform_hemisphere::UniformHemisphere;

mod cosine_weighted_hemisphere;
pub use cosine_weighted_hemisphere::CosineWeightedHemisphere;

mod linear_weighted;
pub use linear_weighted::LinearWeighted;

mod sky_light_pdf;
pub use sky_light_pdf::SkyLightPdf;

pub trait RandomDistribution<T> {
    fn value(&self) -> T;
    fn pdf(&self, value: T) -> f64;
}
