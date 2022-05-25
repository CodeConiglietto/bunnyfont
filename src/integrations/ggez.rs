use ggez::{
    graphics::{
        spritebatch::SpriteBatch, BlendMode, Color as GgColor, DrawParam, Drawable, FilterMode,
        Image as GgImage, Rect,
    },
    mint, Context, GameResult,
};
use glam::f32::{mat2, vec2, Affine2};

use crate::{
    char::BunnyChar,
    char_transforms::{CharMirror, CharRotation},
    font::BunnyFont,
    traits::{color::Color, source_image::SourceImage},
};

pub type GgBunnyFont = BunnyFont<GgImage>;
pub type GgBunnyChar = BunnyChar<GgColor>;

impl Color for GgColor {}

impl SourceImage for GgImage {
    type Color = GgColor;

    fn get_pixel_dimensions(&self) -> (usize, usize) {
        (self.width().into(), self.height().into())
    }
}

impl CharRotation {
    pub fn to_transform(&self) -> Affine2 {
        match self {
            Self::None => Affine2::IDENTITY,
            Self::Rotation90 => Affine2::from_mat2_translation(
                mat2(vec2(0.0, 1.0), vec2(-1.0, 0.0)),
                vec2(1.0, 0.0),
            ),
            Self::Rotation180 => Affine2::from_mat2_translation(
                mat2(vec2(-1.0, 0.0), vec2(0.0, -1.0)),
                vec2(1.0, 1.0),
            ),
            Self::Rotation270 => Affine2::from_mat2_translation(
                mat2(vec2(0.0, -1.0), vec2(1.0, 0.0)),
                vec2(0.0, 1.0),
            ),
        }
    }
}

impl CharMirror {
    pub fn to_transform(&self) -> Affine2 {
        match self {
            Self::None => Affine2::IDENTITY,
            Self::MirrorX => Affine2::from_mat2_translation(
                mat2(vec2(-1.0, 0.0), vec2(0.0, 1.0)),
                vec2(1.0, 0.0),
            ),
            Self::MirrorY => Affine2::from_mat2_translation(
                mat2(vec2(1.0, 0.0), vec2(0.0, -1.0)),
                vec2(0.0, 1.0),
            ),
            Self::MirrorBoth => Affine2::from_mat2_translation(
                mat2(vec2(-1.0, 0.0), vec2(0.0, -1.0)),
                vec2(1.0, 1.0),
            ),
        }
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
        let (dest_x, dest_y) = dest;
        let (char_width, char_height) = batch.font.char_dimensions();

        let rotation = self.rotation.to_transform();
        let mirror = self.mirror.to_transform();
        let translation = Affine2::from_translation(vec2(dest_x as f32, dest_y as f32));
        let scaling = Affine2::from_scale(vec2(
            scaling * char_width as f32,
            scaling * char_height as f32,
        ));

        let transform = scaling * translation * mirror * rotation;

        let transform_arr = transform.to_cols_array_2d();
        let transform_homogeneous = mint::ColumnMatrix4::from([
            [transform_arr[0][0], transform_arr[0][1], 0.0, 0.0],
            [transform_arr[1][0], transform_arr[1][1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform_arr[2][0], transform_arr[2][1], 0.0, 1.0],
        ]);

        let (src_x, src_y, src_w, src_h) = batch.font.get_src_uvs(self.index);

        if let Some(background) = self.background {
            let (bg_src_x, bg_src_y, bg_src_w, bg_src_h) =
                batch.font.get_src_uvs(BACKGROUND_CHAR_INDEX);

            batch.batch.add(
                DrawParam::new()
                    .src(Rect::new(bg_src_x, bg_src_y, bg_src_w, bg_src_h))
                    .transform(transform_homogeneous)
                    .color(background.into()),
            );
        }

        batch.batch.add(
            DrawParam::new()
                .src(Rect::new(src_x, src_y, src_w, src_h))
                .transform(transform_homogeneous)
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
