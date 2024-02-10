use anyhow::{anyhow, Result};
use bezier_rs::{Bezier, TValueType};
use fj::core::{objects::Region, operations::build::BuildRegion, services::Services};
use font::{glyph::Segment, Font, Glyph, Offset};

const DEFAULT_RESOLUTION: usize = 5;

pub struct GlyphRegionBuilder {
    glyph: Glyph,
}

impl GlyphRegionBuilder {
    pub fn try_new(font: &mut Font, character: char) -> Result<Self> {
        let glyph = font.glyph(character)?;
        match glyph {
            Some(glyph) => Ok(Self { glyph }),
            None => Err(anyhow!("Character not in font: {}", character)),
        }
    }

    pub fn build(self, services: &mut Services) -> Vec<Region> {
        let mut point_lists = vec![];
        let mut a = Offset::default();
        for contour in self.glyph.iter() {
            a += contour.offset;
            let mut beziers = vec![];
            for segment in contour.iter() {
                match *segment {
                    Segment::Linear(mut b) => {
                        b += a;
                        beziers.push(Bezier::from_linear_coordinates(
                            a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64,
                        ));
                        a = b;
                    }
                    Segment::Quadratic(mut b, mut c) => {
                        b += a;
                        c += b;
                        beziers.push(Bezier::from_quadratic_coordinates(
                            a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64, c.0 as f64, c.1 as f64,
                        ));
                        a = c;
                    }
                    Segment::Cubic(mut b, mut c, mut d) => {
                        b += a;
                        c += b;
                        d += c;
                        beziers.push(Bezier::from_cubic_coordinates(
                            a.0 as f64, a.1 as f64, b.0 as f64, b.1 as f64, c.0 as f64, c.1 as f64,
                            d.0 as f64, d.1 as f64,
                        ));
                        a = d;
                    }
                }
            }
            let mut res = vec![];
            for bezier in beziers.iter() {
                let x = bezier
                    .compute_lookup_table(Some(DEFAULT_RESOLUTION), Some(TValueType::Euclidean));
                for p in x {
                    if !res.contains(&[p.x, p.y]) {
                        res.push([p.x, p.y]);
                    }
                }
            }
            point_lists.push(res);
        }
        let mut max = f64::MIN;
        for point_list in &point_lists {
            for point in point_list {
                if point[1] > max {
                    max = point[1]
                }
            }
        }
        let point_lists: Vec<Vec<[f64; 2]>> = point_lists
            .iter()
            .map(|point_list| {
                point_list
                    .iter()
                    .map(|point| [point[0] / max, point[1] / max])
                    .collect()
            })
            .collect();
        let mut polygons = vec![];
        for mut region_points in point_lists {
            region_points.reverse();
            polygons.push(Region::polygon(region_points, services));
        }
        polygons
    }
}
