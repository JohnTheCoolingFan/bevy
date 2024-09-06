//! Polynomial splines of order 4

use super::PolynomialSegment;

/// A 4-th order polynomial curve segemnt
pub type CubicSegment<P> = PolynomialSegment<4, P>;
