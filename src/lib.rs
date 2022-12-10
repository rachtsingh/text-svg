use rusttype::{Font, IntoGlyphId, OutlineBuilder, Point, Rect, Scale, ScaledGlyph};
use svg::node::element::Path;

pub fn text(font: &Font, s: &str, size: f32, start: Point<f32>, letter_spacing: f32) -> Path {
    let mut d = String::new();
    let mut x = start.x;

    for glyph in font.layout(s, Scale::uniform(size), start) {
        glyph.build_outline(&mut Builder {
            x,
            y: start.y,
            d: &mut d,
        });
        x += glyph.unpositioned().exact_bounding_box().unwrap().width() + letter_spacing;
    }

    Path::new().set("d", d).set("fill", "#000")
}

pub struct Glpyh<'font> {
    pub scaled: ScaledGlyph<'font>,
    pub bounding_box: Rect<f32>,
}

impl<'font> Glpyh<'font> {
    pub fn new(font: &'font Font, id: impl IntoGlyphId, size: f32) -> Self {
        let scaled = font.glyph(id).scaled(Scale::uniform(size));
        let bounding_box = scaled.exact_bounding_box().unwrap();
        Self {
            scaled,
            bounding_box,
        }
    }

    pub fn write_path(&self, x: f32, y: f32, d: &mut String) {
        let mut builder = Builder::new(x - self.bounding_box.min.x, y - self.bounding_box.min.y, d);
        self.scaled.build_outline(&mut builder);
    }

    pub fn path(&self, x: f32, y: f32) -> Path {
        let mut d = String::new();
        self.write_path(x, y, &mut d);
        Path::new().set("d", d).set("fill", "#000")
    }
}

pub struct Builder<'a> {
    pub x: f32,
    pub y: f32,
    pub d: &'a mut String,
}

impl<'a> Builder<'a> {
    pub fn new(x: f32, y: f32, d: &'a mut String) -> Self {
        Self { x, y, d }
    }
}

impl OutlineBuilder for Builder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.d.push_str(&format!("M{} {}", x + self.x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.d.push_str(&format!("L{} {}", x + self.x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.d
            .push_str(&format!("Q{} {},{} {}", x1 + self.x, y1, x + self.x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.d.push_str(&format!(
            "C{} {},{} {},{} {}",
            x1 + self.x,
            y1,
            x2 + self.x,
            y2,
            x + self.x,
            y
        ));
    }

    fn close(&mut self) {
        self.d.push('Z');
    }
}
