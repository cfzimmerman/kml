use num_traits::Float;

use crate::types::geometry::Geometry;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct MultiGeometry<T: Float = f64>(pub Vec<Geometry<T>>);
