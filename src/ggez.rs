use ggez::{
    graphics::{
        spritebatch::SpriteBatch, BlendMode, Color as GgColor, DrawParam, Drawable, FilterMode,
        Image, Rect,
    },
    mint, Context, GameResult,
};

use crate::{
    char::BunnyChar,
    font::BunnyFont,
    traits::{color::Color, source_image::SourceImage},
};

pub type GgBunnyFont = BunnyFont<Image>;
pub type GgBunnyChar = BunnyChar<GgColor>;

impl Color for GgColor {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        GgColor::new(r, g, b, a)
    }
}

impl SourceImage for Image {
    type Color = GgColor;

    fn get_pixel_dimensions(&self) -> (usize, usize) {
        (self.width().into(), self.height().into())
    }
}

pub struct GgBunnyFontBatch {
    font: GgBunnyFont,
    batch: SpriteBatch,
}

impl GgBunnyFontBatch {
    pub fn new(font: GgBunnyFont) -> GameResult<Self> {
        let mut batch = SpriteBatch::new(font.texture().clone());

        batch.set_filter(FilterMode::Nearest);
        batch.set_blend_mode(Some(BlendMode::Alpha));

        Ok(Self { batch, font })
    }

    pub fn font(&self) -> &GgBunnyFont {
        &self.font
    }

    pub fn tile_size(&self, scaling: f32) -> (f32, f32) {
        (
            scaling * self.font.char_dimensions().0 as f32,
            scaling * self.font.char_dimensions().1 as f32,
        )
    }

    pub fn clear(&mut self) {
        self.batch.clear();
    }
}

const BACKGROUND_CHAR_INDEX: usize = 0x2c7;

impl GgBunnyChar {
    pub fn draw_to_font_batch(&self, batch: &mut GgBunnyFontBatch, dest: (i32, i32), scaling: f32) {
        let mirror_scale = self.mirror.into_scale();
        let (dest_x, dest_y) = dest;
        let (tile_width, tile_height) = batch.tile_size(scaling);

        let dest = mint::Point2::from([dest_x as f32 * tile_width, dest_y as f32 * tile_height]);

        let scale = mint::Vector2::from([mirror_scale.0 * scaling, mirror_scale.1 * scaling]);
        let offset = mint::Point2::from([0.0, 0.0]);
        let (src_x, src_y, src_w, src_h) = batch.font.get_src_uvs(self.index);
        let (bg_src_x, bg_src_y, bg_src_w, bg_src_h) =
            batch.font.get_src_uvs(BACKGROUND_CHAR_INDEX);

        if let Some(background) = self.background {
            batch.batch.add(
                DrawParam::new()
                    .src(Rect::new(bg_src_x, bg_src_y, bg_src_w, bg_src_h))
                    .dest(dest)
                    .rotation(self.rotation.into_rotation())
                    .scale(mint::Vector2::from([
                        scale.x * batch.font.char_dimensions().0 as f32,
                        scale.y * batch.font.char_dimensions().1 as f32,
                    ]))
                    .offset(offset)
                    .color(background.into()),
            );
        }

        batch.batch.add(
            DrawParam::new()
                .src(Rect::new(src_x, src_y, src_w, src_h))
                .dest(dest)
                .rotation(self.rotation.into_rotation())
                .scale(scale)
                .offset(offset)
                .color(self.foreground.into()),
        );
    }
}

impl Drawable for GgBunnyFontBatch {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.batch.draw(ctx, param)
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        self.batch.dimensions(ctx)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.batch.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.batch.blend_mode()
    }
}
