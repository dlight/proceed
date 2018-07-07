extern crate ggez;

use ggez::graphics::*;
use ggez::*;

struct FpsBox {
    font: Font,
    text: Vec<Text>,
    margin: u32,
}

impl FpsBox {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let font = Font::default_font()?;
        let text = vec![Text::new(ctx, "tick: 0", &font)?];

        Ok(Self {
            font,
            text,
            margin: 5,
        })
    }

    fn update(&mut self, ctx: &mut Context, tick: u32) -> GameResult<()> {
        let fps = timer::get_fps(ctx);

        let fps_str = format!("tick {}\nfps {:.0}", tick, fps);

        let wrap_limit = 500;

        let (_, lines) = self.font.get_wrap(&fps_str, wrap_limit);

        self.text = lines
            .iter()
            .map(|line| graphics::Text::new(ctx, &line, &self.font))
            .collect::<GameResult<_>>()?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (n, line) in self.text.iter().enumerate() {
            let fps_x = ctx.conf.window_mode.width - line.width() - self.margin;

            let fps_y = (n as u32) * (self.font.get_height() as u32) + self.margin;

            let pos = Point2::new(fps_x as f32, fps_y as f32);

            line.draw(ctx, pos, 0.0)?;
        }

        Ok(())
    }
}

struct GameState {
    fps_box: FpsBox,
    wait_time: f32,
    tick: u32,
}

impl GameState {
    fn new(ctx: &mut Context, wait_time: f32) -> GameResult<Self> {
        Ok(Self {
            fps_box: FpsBox::new(ctx)?,
            wait_time,
            tick: 0,
        })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut update_fps = false;

        while timer::check_update_time(ctx, (1.0 / self.wait_time) as u32) {
            self.tick += 1;

            update_fps = true;
        }

        if update_fps {
            self.fps_box.update(ctx, self.tick)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx);

        self.fps_box.draw(ctx)?;

        present(ctx);

        timer::yield_now();

        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let mut ctx = Context::load_from_conf("proceed", "Elias Amaral", conf::Conf::new())?;

    let mut state = GameState::new(&mut ctx, 0.5)?;

    event::run(&mut ctx, &mut state)?;

    Ok(())
}
