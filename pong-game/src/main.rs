use ggez;
use ggez::{Context, GameResult};
use ggez::ContextBuilder;
use ggez::event;
use ggez::graphics::{self, Rect};
use ggez::glam::Vec2;

// Pads Consts
const PAD_HEIGHT: f32 = 100.0;
const PAD_WIDTH: f32 = 20.0;
const PAD_HEIGHT_HALF: f32 = PAD_HEIGHT * 0.5;
const PAD_WIDTH_HALF: f32 =  PAD_WIDTH * 0.5;

// Screen Consts
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;
const SCREEN_WIDTH_HALF: f32 = SCREEN_WIDTH * 0.5;
const SCREEN_HEIGHT_HALF: f32 = SCREEN_HEIGHT * 0.5;

// Ball Consts
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

struct MainState {
    player_1_pos: Vec2,
    player_2_pos: Vec2,
    ball_pos: Vec2
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {

        MainState {
            player_1_pos: ggez::glam::Vec2::new(PAD_WIDTH_HALF, SCREEN_HEIGHT_HALF),
            player_2_pos: ggez::glam::Vec2::new(SCREEN_WIDTH - PAD_HEIGHT_HALF, SCREEN_HEIGHT_HALF),
            ball_pos: ggez::glam::Vec2::new(SCREEN_WIDTH_HALF, SCREEN_HEIGHT_HALF)
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        // Change window color to white
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::GREEN);


        // Size of both pads
        let pad_rect = graphics::Rect::new(-PAD_WIDTH_HALF, -PAD_HEIGHT_HALF, PAD_WIDTH, PAD_HEIGHT);

        // Create mesh for the pad
        let pad_mesh = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            pad_rect,
            graphics::Color::WHITE,
        )?;
        
        // Drawing pads
        // Player 1
        let draw_param = graphics::DrawParam::default().dest(self.player_1_pos);
        canvas.draw(
            &pad_mesh,
            draw_param
        );

        // Player 2
        let draw_param = graphics::DrawParam::default().dest(self.player_2_pos);
        canvas.draw(
            &pad_mesh,
            draw_param
        );


        // Size of the ball
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);

        // Create mesh for the ball
        let ball_mesh = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::RED,
        )?;

        // Drawing ball
        let draw_param = graphics::DrawParam::default().dest(self.ball_pos);
        canvas.draw(
            &ball_mesh,
            draw_param
        );

        // Finish canvas "draws"
        canvas.finish(_ctx)?;
        Ok(())
    }
}


fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    ctx.gfx.set_window_title("Pong");

    // &mut state?
    let state = MainState::new(&mut ctx);

    event::run(ctx, event_loop, state);
}
