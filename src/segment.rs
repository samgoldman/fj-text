use bezier_rs::Bezier;
use font::{glyph::Segment, Offset};

pub fn to_bezier(segment: &Segment, a: &Offset) -> (Bezier, Offset) {
    match *segment {
        Segment::Linear(mut b) => {
            b += *a;
            return (
                Bezier::from_linear_coordinates(a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64),
                b,
            );
        }
        Segment::Quadratic(mut b, mut c) => {
            b += *a;
            c += b;
            return (
                Bezier::from_quadratic_coordinates(
                    a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64, c.0 as f64, c.1 as f64,
                ),
                c,
            );
        }
        Segment::Cubic(mut b, mut c, mut d) => {
            b += *a;
            c += b;
            d += c;
            return (
                Bezier::from_cubic_coordinates(
                    a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64, c.0 as f64, c.1 as f64,
                    d.0 as f64, d.1 as f64,
                ),
                d,
            );
        }
    }
}
