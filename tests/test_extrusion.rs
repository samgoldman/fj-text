use fj::{
    core::{
        objects::{Region, Sketch, Solid},
        operations::{
            build::{BuildSketch, BuildSolid},
            insert::Insert,
            merge::Merge,
            sweep::SweepSketch,
            update::UpdateSketch,
        },
    },
    math::Vector,
};
use fj_text::GlyphRegionBuilder;

use test_case::test_case;

fn extrude(glyph_regions: Vec<Region>, core: &mut fj::core::Core) -> Solid {
    let mut solids = vec![];
    for region in glyph_regions {
        let bottom_surface = core.layers.objects.surfaces.xy_plane();
        let sweep_path: Vector<3> = Vector::from([0., 0., 0.1]);

        let polygon = region.insert(core);
        solids.push(
            Sketch::empty()
                .add_regions(vec![polygon], core)
                .sweep_sketch(bottom_surface, sweep_path, core),
        );
    }

    let mut all = Solid::empty();

    for solid in solids {
        all = all.merge(&solid, core);
    }

    all
}

#[test_case("tests/fonts/AllertaStencil/AllertaStencil-Regular.ttf")]
#[test_case("tests/fonts/NotoSans/NotoSans-Regular.ttf")]
fn test_font(font_file: &str) {
    let fj = fj::Instance::new();
    let mut core = fj.core;
    let mut file = font::File::open(font_file).unwrap();
    let mut font = &mut file[0];
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars() {
        let glyph_regions = GlyphRegionBuilder::try_new(&mut font, c)
            .unwrap()
            .build(&mut core);
        let solid = extrude(glyph_regions, &mut core);
        assert!(solid.shells().len() > 0, "Failed on character: {c}");
    }
}
