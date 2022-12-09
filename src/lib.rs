use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::{Font, IntoGlyphId, OutlineBuilder, Scale};
use std::{fs::File, io::Read};
use svg::node::element::Path;

pub fn f() -> Path {
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

    let builder = Builder::new(&font, 'A', 20.);

    Path::new().set("d", builder.d).set("fill", "#000")
}

#[derive(Default)]
pub struct Builder {
    x: f32,
    y: f32,
    d: String,
}

impl Builder {
    pub fn new(font: &Font, id: impl IntoGlyphId, size: f32) -> Self {
        let scaled = font.glyph(id).scaled(Scale::uniform(size));
        let bounds = scaled.exact_bounding_box().unwrap();

        let mut builder = Self {
            x: -bounds.min.x,
            y: -bounds.min.y,
            d: String::new(),
        };
        scaled.build_outline(&mut builder);
        builder
    }
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
