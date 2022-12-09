use std::{fs::File, io::Read};

use font_kit::{
    family_name::FamilyName, handle::Handle, hinting::HintingOptions, outline::OutlineSink,
    properties::Properties, source::SystemSource,
};
use pathfinder_geometry::{line_segment::LineSegment2F, vector::Vector2F};
use rusttype::{Font, OutlineBuilder, Scale};
use svg::node::element::Path;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn f() -> Path {
    let handle = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap();
    let font = match handle {
        Handle::Path { path, font_index } => {
            let mut file = File::open(path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf);
            Font::try_from_vec_and_index(buf, font_index).unwrap()
        }
        Handle::Memory { bytes, font_index } => {
            Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
        }
    };

    let mut builder = Builder {
        x: 100.,
        y: 100.,
        d: String::new(),
    };
    font.glyph('A')
        .scaled(Scale::uniform(20.))
        .build_outline(&mut builder);

    Path::new().set("d", builder.d).set("fill", "#000")
}

#[derive(Default)]
pub struct Builder {
    x: f32,
    y: f32,
    d: String,
}

impl OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.d.push_str(&format!("M{} {}", x + self.x, y + self.y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.d.push_str(&format!("L{} {}", x + self.x, y + self.y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.d.push_str(&format!(
            "Q{} {},{} {}",
            x1 + self.x,
            y1 + self.y,
            x + self.x,
            y + self.y
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.d.push_str(&format!(
            "C{} {},{} {},{} {}",
            x1 + self.x,
            y1 + self.y,
            x2 + self.x,
            y2 + self.y,
            x + self.x,
            y + self.y
        ));
    }

    fn close(&mut self) {
        self.d.push('Z');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svg::Document;

    #[test]
    fn it_works() {
        let document = Document::new()
            .set("width", 1000)
            .set("height", 1000)
            .add(f());
        svg::save("image.svg", &document);
    }
}
