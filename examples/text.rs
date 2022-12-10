use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use font_svg::text;
use rusttype::{Font, Point};
use std::{fs::File, io::Read};
use svg::Document;

fn main() {
    let handle = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap();

    let font = match handle {
        Handle::Path { path, font_index } => {
            let mut file = File::open(path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            Font::try_from_vec_and_index(buf, font_index).unwrap()
        }
        Handle::Memory { bytes, font_index } => {
            Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
        }
    };

    let path = text(&font, "FontSvg", 20., Point { x: 10., y: 10. }, 2.);

    let document = Document::new()
        .set("width", 200.)
        .set("height", 200.)
        .add(path);

    svg::save("image.svg", &document).unwrap();
}
