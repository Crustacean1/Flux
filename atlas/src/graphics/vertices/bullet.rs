use super::{indices::PointGeometry, layouts::PVertex};

pub fn bullet() -> ([PVertex; 1], [PointGeometry; 1]) {
    ([PVertex([0.0, 0.0, 0.0])], [PointGeometry([0])])
}
