use itertools::Itertools;

use crate::VectorSpace;

pub mod cubic;
pub mod quintic;

/// A segment of a polynomial curve, used to hold precomputed coefficients for fast interpolation.
/// Can be evaluated as a parametric curve over the domain `[0, 1)`.
///
/// Segments can be chained together to form a longer compound curve.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PolynomialSegment<const ORDER: usize, P: VectorSpace> {
    /// Polynomial coefficients for the segment.
    pub coefficients: [P; ORDER],
}
impl<const ORDER: usize, P: VectorSpace> PolynomialSegment<ORDER, P> {
    /// Instantaneous position of a point at parametric value `t`.
    #[inline]
    pub fn position(&self, t: f32) -> P {
        self.coefficients
            .iter()
            .copied()
            .rfold(P::ZERO, |res, v| v + res * t)
    }

    /// Instantaneous velocity of a point at parametric value `t`.
    #[inline]
    pub fn velocity(&self, t: f32) -> P {
        self.coefficients
            .into_iter()
            .enumerate()
            .skip(1)
            .rfold(P::ZERO, |res, (i, v)| v * i as f32 + res * t)
    }

    /// Instantaneous acceleration of a point at parametric value `t`.
    #[inline]
    pub fn acceleration(&self, t: f32) -> P {
        self.coefficients
            .into_iter()
            .enumerate()
            .skip(2)
            .rfold(P::ZERO, |res, (i, v)| v * (i * (i - 1)) as f32 + res * t)
    }

    /// Calculate polynomial coefficients for the cubic curve using a characteristic matrix.
    #[inline]
    fn coefficients(p: [P; ORDER], char_matrix: [[f32; ORDER]; ORDER]) -> Self {
        let coefficients = char_matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .zip(p)
                    .map(|(cf, point)| point * cf)
                    .fold(P::ZERO, |sum, v| sum + v)
            })
            .collect::<Vec<P>>()
            .try_into()
            .unwrap();
        Self { coefficients }
    }
}
