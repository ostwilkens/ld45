use ggez;
use ggez::{event, graphics, Context, GameResult};
use mint::Vector2;

const MAP_SIZE: (f32, f32) = (32.0, 16.0);
const TILE_SCALE: f32 = 16.0;
const RENDER_SIZE: (f32, f32) = (MAP_SIZE.0 * TILE_SCALE, MAP_SIZE.1 * TILE_SCALE);

struct Player {
    pos: Vector2<f32>,
}

impl Player {
    pub fn new(pos: Vector2<f32>) -> Self {
        Player { pos }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, TILE_SCALE, TILE_SCALE),
            [1.0, 0.0, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &rect, (self.pos,))?;
        Ok(())
    }
}

struct State {
    player: Player,
}

impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(Vector2 { x: 1.0, y: 1.0 }),
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player.pos.x += 0.1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("run", "ostwilkens")
        .window_setup(ggez::conf::WindowSetup::default().title("run"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(RENDER_SIZE.0, RENDER_SIZE.1))
        .build()?;

    let state = &mut State::new();
    event::run(ctx, event_loop, state)
}
