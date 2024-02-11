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
        services::Services,
    },
    math::Vector,
};
use fj_text::GlyphRegionBuilder;

use test_case::test_case;

fn extrude(glyph_regions: Vec<Region>, services: &mut Services) -> Solid {
    let mut solids = vec![];
    for region in glyph_regions {
        let bottom_surface = services.objects.surfaces.xy_plane();
        let sweep_path: Vector<3> = Vector::from([0., 0., 0.1]);

        let polygon = region.insert(services);
        solids.push(Sketch::empty().add_region(polygon).sweep_sketch(
            bottom_surface,
            sweep_path,
            services,
        ));
    }

    let mut all = Solid::empty();

    for solid in solids {
        all = all.merge(&solid);
    }

    all
}

#[test_case("tests/fonts/AllertaStencil/AllertaStencil-Regular.ttf")]
#[test_case("tests/fonts/NotoSans/NotoSans-Regular.ttf")]
fn test_font(font_file: &str) {
    let mut services = Services::new();
    let mut file = font::File::open(font_file).unwrap();
    let mut font = &mut file[0];
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars() {
        let glyph_regions = GlyphRegionBuilder::try_new(&mut font, c)
            .unwrap()
            .build(&mut services);
        let solid = extrude(glyph_regions, &mut services);
        assert!(solid.shells().len() > 0, "Failed on character: {c}");
    }
}
