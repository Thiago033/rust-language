use ggez::{Context, GameResult, ContextBuilder};
use ggez::event;
use ggez::graphics::{self, Drawable, PxScale};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode, KeyInput};

use rand::{thread_rng, Rng};

// Pads Consts
const PAD_HEIGHT: f32 = 100.0;
const PAD_WIDTH: f32 = 20.0;
const PAD_HEIGHT_HALF: f32 = PAD_HEIGHT * 0.5;
const PAD_WIDTH_HALF: f32 =  PAD_WIDTH * 0.5;
const PAD_SPEED: f32 = 1000.0;

// Screen Consts
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH_HALF: f32 = SCREEN_WIDTH * 0.5;
const SCREEN_HEIGHT_HALF: f32 = SCREEN_HEIGHT * 0.5;

// Ball Consts
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 120.0;

// Keeps everything inside the screen
fn bounds(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn random_ball_vec(vec: &mut ggez::glam::Vec2, x: f32, y: f32) {
    let mut rng = thread_rng();

    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x
    };

    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y
    };
}

struct MainState {
    player_1_pos: Vec2,
    player_2_pos: Vec2,
    ball_position: Vec2,
    ball_velocity: Vec2,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {

        let mut ball_velocity = ggez::glam::Vec2::new(0.0, 0.0);
        random_ball_vec(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos: ggez::glam::Vec2::new(PAD_WIDTH_HALF, SCREEN_HEIGHT_HALF),
            player_2_pos: ggez::glam::Vec2::new(SCREEN_WIDTH - 10.0, SCREEN_HEIGHT_HALF),

            ball_position: ggez::glam::Vec2::new(SCREEN_WIDTH_HALF, SCREEN_HEIGHT_HALF),
            ball_velocity,

            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // println!("PLAYER_1 = X: {}, Y:{}", self.player_1_pos.x, self.player_1_pos.y);
        // println!("PLAYER_2 = X: {}, Y:{}", self.player_2_pos.x, self.player_2_pos.y);
        // println!("BALL     = X: {}, Y:{}", self.ball_position.x, self.ball_position.y);
        // println!("BALL VELOCITY = {}:", self.ball_velocity.x);


        // Ball movement
        let delta_time = ctx.time.delta().as_secs_f32();
        self.ball_position += self.ball_velocity * delta_time;

        // Scoring
        // Player 2 scores (ball out on left side)
        if self.ball_position.x < 0.0 {
            self.ball_position.x = SCREEN_WIDTH_HALF;
            self.ball_position.y = SCREEN_HEIGHT_HALF;
            random_ball_vec(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }

        // Player 1 scores (ball out on right side)
        if self.ball_position.x > SCREEN_WIDTH {
            self.ball_position.x = SCREEN_WIDTH_HALF;
            self.ball_position.y = SCREEN_HEIGHT_HALF;
            random_ball_vec(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        // Ball Walls bounce
        if self.ball_position.y < BALL_SIZE_HALF {
            self.ball_position.y = BALL_SIZE_HALF;
            self.ball_velocity.y = self.ball_velocity.y.abs();
        } else if self.ball_position.y > SCREEN_HEIGHT - BALL_SIZE_HALF {
            self.ball_position.y = SCREEN_HEIGHT - BALL_SIZE_HALF;
            self.ball_velocity.y = -self.ball_velocity.y.abs();
        }

        
        // Ball Pad bounce
        // Bounce on the left pad
        let player_1_pad = 
            self.ball_position.x - BALL_SIZE_HALF < self.player_1_pos.x + PAD_WIDTH_HALF    &&
            self.ball_position.x + BALL_SIZE_HALF > self.player_1_pos.x - PAD_WIDTH_HALF    &&
            self.ball_position.y - BALL_SIZE_HALF < self.player_1_pos.y + PAD_HEIGHT_HALF   &&
            self.ball_position.y + BALL_SIZE_HALF > self.player_1_pos.y - PAD_HEIGHT_HALF;

        if player_1_pad {
            self.ball_velocity.x = self.ball_velocity.x.abs();
        }

        // Bounce on the right pad
        let player_2_pad = 
            self.ball_position.x - BALL_SIZE_HALF < self.player_2_pos.x + PAD_WIDTH_HALF    &&
            self.ball_position.x + BALL_SIZE_HALF > self.player_2_pos.x - PAD_WIDTH_HALF    &&
            self.ball_position.y - BALL_SIZE_HALF < self.player_2_pos.y + PAD_HEIGHT_HALF   &&
            self.ball_position.y + BALL_SIZE_HALF > self.player_2_pos.y - PAD_HEIGHT_HALF;

        if player_2_pad {
            self.ball_velocity.x = -self.ball_velocity.x.abs();
        }

        Ok(())
    }

    // Pads Movement
    // by default exit the game if the escape key is pressed.
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {

        // synchronize frames and pads/ball movement
        // Passing delta time as f32 to multiply with pad_speed
        let delta_time = _ctx.time.delta().as_secs_f32();

        // Player 1 Movement
        match input.keycode {
            Some(KeyCode::Up) => {
                self.player_1_pos.y -= PAD_SPEED * delta_time;
                //print!("{}", self.player_1_pos.y);            
            }
            Some(KeyCode::Down) => {
                self.player_1_pos.y += PAD_SPEED * delta_time;
                //print!("{}", self.player_1_pos.y);            
            }
            _ => (),
        }
        bounds(&mut self.player_1_pos.y, PAD_HEIGHT_HALF, SCREEN_HEIGHT - PAD_HEIGHT_HALF);

        // Player 2 Movement
        match input.keycode {
            Some(KeyCode::W) => {
                self.player_2_pos.y -= PAD_SPEED * delta_time;
                //print!("{}", self.player_2_pos.y);            
            }
            Some(KeyCode::S) => {
                self.player_2_pos.y += PAD_SPEED * delta_time;
                //print!("{}", self.player_2_pos.y);            
            }
            _ => (),
        }
        bounds(&mut self.player_2_pos.y, PAD_HEIGHT_HALF, SCREEN_HEIGHT - PAD_HEIGHT_HALF);

        Ok(())
    }

    // Draw everything on the game screen
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        // Change window color
        let mut canvas = graphics::Canvas::from_frame(_ctx, graphics::Color::WHITE);

        // Size of both pads
        let pad_rect = graphics::Rect::new(-PAD_WIDTH_HALF, -PAD_HEIGHT_HALF, PAD_WIDTH, PAD_HEIGHT);

        // Create mesh for the pad
        let pad_mesh = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            pad_rect,
            graphics::Color::BLACK,
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
            graphics::Color::new(0.25, 0.25, 0.25, 1.0),
        )?;

        // Drawing ball
        let draw_param = graphics::DrawParam::default().dest(self.ball_position);
        canvas.draw(
            &ball_mesh,
            draw_param
        );

        // Drawing Score HUD
        let mut score_text = graphics::Text::new(format!("{}   {}", self.player_1_score, self.player_2_score));

        // Text Size
        score_text.set_scale(PxScale::from(40.0));
        
        // Text Position
        let score_pos = ggez::glam::Vec2::new(SCREEN_WIDTH_HALF - 50.0, 40.0);
        
        let draw_param = graphics::DrawParam::default()
            .dest(score_pos)
            .color(ggez::graphics::Color::BLACK);

        canvas.draw(
            &score_text, 
            draw_param
        );

        // Finish canvas "draws"
        canvas.finish(_ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "Thiago")
        .build()
        .expect("aieee, could not create ggez context!");

    ctx.gfx.set_window_title("Pong");

    // &mut state?
    let state = MainState::new(&mut ctx);

    event::run(ctx, event_loop, state);
}
