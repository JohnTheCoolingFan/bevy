use super::PolynomialSegment;
use crate::VectorSpace;

pub struct QuinticHermite<P> {
    points: Vec<(P, P, P)>,
}

impl<P: VectorSpace> QuinticHermite<P> {
    const CHAR_MATRIX: [[f32; 6]; 6] = [
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0, 0.0, 0.0],
        [-10.0, -6.0, -1.5, 10.0, -4.0, 0.5],
        [15.0, 8.0, 1.5, -15.0, 7.0, -1.0],
        [-6.0, -3.0, -0.5, 6.0, -3.0, 0.5],
    ];

    pub fn into_segments(self) -> Option<Vec<PolynomialSegment<6, P>>> {
        if self.points.len() < 2 {
            None
        } else {
            Some(
                self.points
                    .windows(2)
                    .map(|pairs| {
                        let (a, b, c) = pairs[0];
                        let (d, e, f) = pairs[1];
                        [a, b, c, d, e, f]
                    })
                    .map(|points| PolynomialSegment::coefficients(points, Self::CHAR_MATRIX))
                    .collect(),
            )
        }
    }
}
