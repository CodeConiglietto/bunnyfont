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

pub struct BunnyFontBatch {
    font: GgBunnyFont,
    fg_batch: SpriteBatch,
    bg_batch: SpriteBatch,
    scaling: f32,
}

impl BunnyFontBatch {
    pub fn new(ctx: &mut Context, font: GgBunnyFont, scaling: f32) -> GameResult<Self> {
        let mut fg_batch = SpriteBatch::new(font.texture().clone());
        let mut bg_batch = SpriteBatch::new(Image::solid(ctx, 1, GgColor::WHITE)?);

        fg_batch.set_filter(FilterMode::Nearest);
        bg_batch.set_filter(FilterMode::Nearest);

        fg_batch.set_blend_mode(Some(BlendMode::Alpha));
        bg_batch.set_blend_mode(Some(BlendMode::Alpha));

        Ok(Self {
            fg_batch,
            bg_batch,
            font,
            scaling,
        })
    }

    pub fn font(&self) -> &GgBunnyFont {
        &self.font
    }

    pub fn set_scaling(&mut self, scaling: f32) {
        self.scaling = scaling;
    }

    pub fn scaling(&self) -> f32 {
        self.scaling
    }

    pub fn tile_width(&self) -> f32 {
        self.scaling * self.font.char_dimensions().0 as f32
    }

    pub fn tile_height(&self) -> f32 {
        self.scaling * self.font.char_dimensions().1 as f32
    }

    pub fn add<P>(&mut self, c: GgBunnyChar, dest: P)
    where
        P: Into<mint::Point2<u32>>,
    {
        let mirror_scale = c.mirror.into_scale();
        let dest = dest.into();
        let dest = mint::Point2::from([
            dest.x as f32 * self.tile_width(),
            dest.y as f32 * self.tile_height(),
        ]);

        let scale =
            mint::Vector2::from([mirror_scale.0 * self.scaling, mirror_scale.1 * self.scaling]);
        let offset = mint::Point2::from([0.0, 0.0]);
        let src = self.font.get_src_uvs(c.index);

        self.fg_batch.add(
            DrawParam::new()
                .src(Rect::new(src.0, src.1, src.2, src.3))
                .dest(dest)
                .rotation(c.rotation.into_rotation())
                .scale(scale)
                .offset(offset)
                .color(c.foreground.into()),
        );

        if let Some(background) = c.background {
            self.bg_batch.add(
                DrawParam::new()
                    .src(Rect::new(0.0, 0.0, 1.0, 1.0))
                    .dest(dest)
                    .rotation(c.rotation.into_rotation())
                    .scale(mint::Vector2::from([
                        scale.x * self.font.char_dimensions().0 as f32,
                        scale.y * self.font.char_dimensions().1 as f32,
                    ]))
                    .offset(offset)
                    .color(background.into()),
            );
        }
    }

    pub fn clear(&mut self) {
        self.fg_batch.clear();
        self.bg_batch.clear();
    }
}

impl Drawable for BunnyFontBatch {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.bg_batch.draw(ctx, param)?;
        self.fg_batch.draw(ctx, param)?;
        Ok(())
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        match (self.fg_batch.dimensions(ctx), self.bg_batch.dimensions(ctx)) {
            (None, None) => None,
            (Some(fg_dim), None) => Some(fg_dim),
            (None, Some(bg_dim)) => Some(bg_dim),
            (Some(fg_dim), Some(bg_dim)) => Some(fg_dim.combine_with(bg_dim)),
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.bg_batch.set_blend_mode(mode);
        self.fg_batch.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        let fg_mode = self.fg_batch.blend_mode();
        let bg_mode = self.bg_batch.blend_mode();

        assert_eq!(fg_mode, bg_mode);

        fg_mode
    }
}
