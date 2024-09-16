use anyhow::{anyhow, Result};
use bezier_rs::{Bezier, TValueType};
use fj::{
    core::{
        objects::Region,
        operations::{build::BuildRegion, update::UpdateRegion},
        Core,
    },
    math::Winding,
};
use font::{Font, Glyph, Offset, Read};

mod segment;

const DEFAULT_RESOLUTION: usize = 5;

pub enum Alignment {
    Center,
    Left,
    Right,
}

pub struct GlyphRegionBuilder {
    glyph: Glyph,
    alignment: Alignment,
    height: f64,
}

impl GlyphRegionBuilder {
    pub fn try_new<T: Read>(font: &mut Font<T>, character: char) -> Result<Self> {
        let glyph = font.glyph(character)?;
        match glyph {
            Some(glyph) => Ok(Self {
                glyph,
                alignment: Alignment::Left,
                height: 1.0,
            }),
            None => Err(anyhow!("Character not in font: {}", character)),
        }
    }

    pub fn align_center(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn build(self, core: &mut Core) -> Vec<Region> {
        let mut point_lists = vec![];
        let mut a = Offset::default();
        for contour in self.glyph.iter() {
            a += contour.offset;
            let mut beziers: Vec<Bezier> = vec![];
            for segment in contour.iter() {
                let (bezier, new_offset) = segment::to_bezier(segment, &mut a);
                beziers.push(bezier);
                a = new_offset;
            }
            let mut res: Vec<[f64; 2]> = vec![];
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

        let mut polygons: Vec<Region> = vec![];
        for mut region_points in point_lists {
            region_points.reverse();
            let region = Region::polygon(region_points, core);
            if region.exterior().winding(&core.layers.geometry) == Winding::Cw {
                let last_polygon = polygons.remove(0);
                let new_region = last_polygon.add_interiors([region.exterior().clone()], core);
                polygons.insert(0, new_region);
            } else {
                polygons.push(region);
            }
        }
        polygons
    }
}
