use std::{fs, path::PathBuf};

use bunnyfont::ggez::{BunnyFontBatch, GgBunnyChar, GgBunnyFont};
use failure::Fallible;
use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Image, Rect},
    input::mouse::MouseButton,
    Context, ContextBuilder, GameResult,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    /// Path to the font image
    #[structopt(parse(from_os_str))]
    font_path: PathBuf,

    /// Width of a single char
    #[structopt(short("w"), long("width"), parse(try_from_str))]
    char_width: usize,

    /// Width of a single char
    #[structopt(short("h"), long("height"), parse(try_from_str))]
    char_height: usize,

    /// Scaling factor
    #[structopt(short("s"), long("scaling"), parse(try_from_str), default_value("1"))]
    scaling: usize,
}

fn main() {
    let opts = Opts::from_args();

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let indexer = Indexer::new(&mut ctx, opts).unwrap();

    event::run(ctx, event_loop, indexer);
}

struct Indexer {
    font_batch: BunnyFontBatch,
}

impl Indexer {
    pub fn new(ctx: &mut Context, opts: Opts) -> Fallible<Indexer> {
        let texture = Image::from_bytes(ctx, &fs::read(&opts.font_path)?)?;

        let width = texture.width() as f32 * opts.scaling as f32;
        let height = texture.height() as f32 * opts.scaling as f32;

        graphics::set_mode(
            ctx,
            WindowMode {
                width,
                height,
                ..WindowMode::default()
            },
        )?;
        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height))?;

        Ok(Indexer {
            font_batch: BunnyFontBatch::new(
                ctx,
                GgBunnyFont::new(texture, (opts.char_width, opts.char_height)),
                opts.scaling as f32,
            )?,
        })
    }
}

impl EventHandler<ggez::GameError> for Indexer {
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let char_x = (x / self.font_batch.tile_width()).floor() as usize;
        let char_y = (y / self.font_batch.tile_height()).floor() as usize;

        let width = self.font_batch.font().charset_dimensions().0;

        let index = char_y * width + char_x;

        println!(
            "X: {}, Y: {}, Index: 0x{:03X} ({})",
            char_x, char_y, index, index
        );
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        self.font_batch.clear();

        let width = self.font_batch.font().charset_dimensions().0;

        for index in 0..self.font_batch.font().len() {
            self.font_batch.add(
                GgBunnyChar::new(index),
                [(index % width) as u32, (index / width) as u32],
            );
        }

        graphics::draw(ctx, &self.font_batch, DrawParam::default())?;
        graphics::present(ctx)?;

        Ok(())
    }
}
