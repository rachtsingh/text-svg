use rusttype::{Font, IntoGlyphId, OutlineBuilder, Point, Rect, Scale, ScaledGlyph};
use svg::node::element::Path;

pub fn text(
    font: &Font,
    s: &str,
    size: f32,
    start: Point<f32>,
    letter_spacing: f32,
) -> (Path, Point<f32>) {
    let mut d = String::new();
    let mut x = start.x;

    let scale = Scale::uniform(size);
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = v_metrics.ascent - v_metrics.descent;

    for glyph in font.layout(
        s,
        scale,
        Point {
            x,
            y: start.y + v_metrics.ascent,
        },
    ) {
        let bounding_box = glyph.unpositioned().exact_bounding_box().unwrap();
        glyph.build_outline(&mut Builder {
            x: x + bounding_box.min.x,
            y: glyphs_height + bounding_box.min.y,
            d: &mut d,
        });
        x += bounding_box.width() + letter_spacing;
    }

    (
        Path::new().set("d", d).set("fill", "#000"),
        Point {
            x,
            y: glyphs_height,
        },
    )
}

pub struct Glyph<'font> {
    pub scaled: ScaledGlyph<'font>,
    pub bounding_box: Rect<f32>,
}

impl<'font> Glyph<'font> {
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
