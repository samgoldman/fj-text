# Fornjot Text

![Character '3' in Fornjot](assets/screenshot.png)

## About

Fornjot Text is a library for creating text in [Fornjot](https://www.fornjot.app/), a CAD kernel written in Rust.

This currently very much an experiment. Feedback is welcome!

## Usage

For now, see the [tests](tests/test_extrusion.rs). More examples coming soon!

## Limitations

* Unlikely to support fonts with nested features (like a zero with a dot in the center).
* Rendering using Fornjot is slow - try exporting to an STL/OBJ/3MF and viewing in another viewer.
* All glyphs are individually normalized to a height of 1 - this should be changed, but I need to learn more about fonts first.

## Future

* Support string generation (currently only single glyph at a time)
* Reduce number of vertices used

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
