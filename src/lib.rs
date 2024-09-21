use anyhow::{anyhow, Result};
use bezier_rs::{Bezier, Identifier, Subpath, TValueType};
use fj::{
    core::{
        objects::Region,
        operations::{build::BuildRegion, update::UpdateRegion},
        Core,
    },
    math::Winding,
};
use font::{Font, Glyph, Offset, Read};
use glam::{DAffine2, DVec2};

mod segment;

const DEFAULT_RESOLUTION: usize = 5;

pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}

pub enum VerticalAlignment {
    Middle,
    Bottom,
    Top,
}

pub struct GlyphRegionBuilder {
    glyph: Glyph,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    height: f64,
    translation_x: f64,
    translation_y: f64,
    resolution: usize,
    rotation_degrees: f64,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ID {}
impl Identifier for ID {
    fn new() -> Self {
        Self {}
    }
}

impl GlyphRegionBuilder {
    pub fn try_new<T: Read>(font: &mut Font<T>, character: char) -> Result<Self> {
        let glyph = font.glyph(character)?;
        match glyph {
            Some(glyph) => Ok(Self {
                glyph,
                horizontal_alignment: HorizontalAlignment::Left,
                vertical_alignment: VerticalAlignment::Bottom,
                height: 1.0,
                translation_x: 0.0,
                translation_y: 0.0,
                resolution: DEFAULT_RESOLUTION,
                rotation_degrees: 0.0,
            }),
            None => Err(anyhow!("Character not in font: {}", character)),
        }
    }

    pub fn rotate(mut self, rotation_degrees: f64) -> Self {
        self.rotation_degrees = rotation_degrees;
        self
    }

    pub fn resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.horizontal_alignment = HorizontalAlignment::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.horizontal_alignment = HorizontalAlignment::Right;
        self
    }

    pub fn align_left(mut self) -> Self {
        self.horizontal_alignment = HorizontalAlignment::Left;
        self
    }

    pub fn align_middle(mut self) -> Self {
        self.vertical_alignment = VerticalAlignment::Middle;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn translate_x(mut self, value: f64) -> Self {
        self.translation_x = value;
        self
    }

    pub fn translate_y(mut self, value: f64) -> Self {
        self.translation_y = value;
        self
    }

    pub fn build(self, core: &mut Core) -> Vec<Region> {
        let mut point_lists = vec![];
        let mut a = Offset::default();
        let mut y_max = f64::MIN;
        let mut y_min = f64::MAX;
        for contour in self.glyph.iter() {
            a += contour.offset;
            let mut beziers: Vec<Bezier> = vec![];
            for segment in contour.iter() {
                let (bezier, new_offset) = segment::to_bezier(segment, &mut a);
                beziers.push(bezier);
                a = new_offset;
            }
            let subpath: Subpath<ID> = Subpath::from_beziers(&beziers, true);
            let pivot_point = DVec2 { x: 0., y: 0. };
            let mut subpath = subpath.rotate_about_point(self.rotation_degrees.to_radians(), pivot_point);
            
            let x_shift = match self.horizontal_alignment {
                HorizontalAlignment::Center => (self.glyph.advance_width as f64) / 2.0,
                HorizontalAlignment::Left => 0.0,
                HorizontalAlignment::Right => self.glyph.advance_width as f64,
            };
            let (_, y0, _, y1) = self.glyph.bounding_box;
            let y_shift = match self.vertical_alignment {
                VerticalAlignment::Middle => 0.5*(y1 - y0) as f64,
                VerticalAlignment::Bottom => 0.0,
                VerticalAlignment::Top => 1.0*(y1 - y0) as f64,
            };
            subpath.apply_transform(DAffine2::from_translation(DVec2 { x: x_shift, y: y_shift }));


            let mut res: Vec<[f64; 2]> = vec![];
            for bezier in subpath.iter() {
                let x = bezier
                    .compute_lookup_table(Some(self.resolution), Some(TValueType::Euclidean));
                for p in x {
                    let x = p.x.round();
                    let y = p.y.round();
                    if !res.contains(&[x, y]) {
                        res.push([x, y]);
                        y_max = f64::max(y_max, y);
                        y_min = f64::min(y_min, y);
                    }
                }
            }
            point_lists.push(res);
        }

        let multiplier = self.height / (y_max - y_min);
        let point_lists: Vec<Vec<[f64; 2]>> = point_lists
            .into_iter()
            .map(|list| {
                list.into_iter()
                    .map(|p| [p[0] * multiplier + self.translation_x, p[1] * multiplier + self.translation_y])
                    .collect()
            })
            .collect();

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
