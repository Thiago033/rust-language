use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::*;
use ggez::graphics::{self, Color, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::KeyInput;

use std::env;
use std::f32::consts::PI;
use std::path;

type Point2 = Vec2;
type Vector2 = Vec2;

// **********************************************************************
// Player Consts
// **********************************************************************
// Acceleration in pixels per second.
const PLAYER_THRUST: f32 = 100.0;
// Rotation in radians per second.
const PLAYER_TURN_RATE: f32 = 3.0;
// Player Box size
const PLAYER_BBOX: Vec2 = Vec2::new(37.0, 64.0);

const MAX_IMPACT_VELOCITY: f32 = 75.0;

const GRAVITY_ACCELERATION: f32 = 3.0;

// **********************************************************************
// Game Generic Consts
// **********************************************************************
const DESIRED_FPS: u32 = 60;

const SCREEN_SIZE: Vec2 = Vec2::new(800.0, 600.0);

// Actor type
#[derive(Debug)]
enum ActorType {
    Player,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2,
    facing: f32,
    velocity: Vector2,
    // ang_vel: f32,
    // bbox_size: Vec2,
    rect: Rect
}

// **********************************************************************
// Utility functions.
// **********************************************************************
// Create a unit vector representing the given angle (in radians)
fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, -vy)
}

// Draw actor
fn draw_actor(
    assets: &mut Assets,
    canvas: &mut graphics::Canvas,
    actor: &Actor,
) {
    let pos = actor.pos;

    let image = assets.actor_image(actor);

    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        .rotation(actor.facing)
        .offset(Point2::new(0.5, 0.5));

    // Draw on screen
    canvas.draw(image, drawparams);
}

// **********************************************************************
// Player functions
// **********************************************************************
// Create PLayer
fn create_player() -> Actor {
    Actor {
        tag: ActorType::Player,
        pos: SCREEN_SIZE * 0.5,
        facing: 0.,
        velocity: Vector2::ZERO,
        // ang_vel: 0.,
        // bbox_size: PLAYER_BBOX,
        rect: Rect::new(0.0, 0.0, PLAYER_BBOX.x, PLAYER_BBOX.y)
    }
}

// **************************
// Rocket Physics
// **************************
fn player_handle_input(rocket: &mut Actor, input: &InputState, dt: f32) {
    rocket.facing += dt * PLAYER_TURN_RATE * input.xaxis;

    rocket.facing = rocket.facing % (2.0 * PI);

    if input.yaxis > 0.0 {
        player_thrust(rocket, dt);
    }
}

fn player_thrust(rocket: &mut Actor, dt: f32) {
    let direction_vector = vec_from_angle(rocket.facing);
    let thrust_vector = direction_vector * (PLAYER_THRUST);

    rocket.velocity += thrust_vector * (dt);
}

fn update_actor_position(rocket: &mut Actor, dt: f32) {
    rocket.velocity.y += 10.0 * dt;

    rocket.pos += rocket.velocity * dt;

    rocket.rect.x = rocket.pos.x - rocket.rect.w / 2.0;
    rocket.rect.y = rocket.pos.y - rocket.rect.h / 2.0;
}

fn check_collision(rocket: &mut Actor, ground: graphics::Rect, ctx: &mut Context) {
    if ground.overlaps(&rocket.rect) {

        if rocket.velocity.length() >= MAX_IMPACT_VELOCITY {
                println!("Game over!");
                ctx.request_quit();
        }

        rocket.velocity.y *= -0.5;
        rocket.velocity.x *= 0.99;
        rocket.pos.y = ground.y - rocket.rect.h / 2.0;
    }
}

struct Assets {
    player_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::from_path(ctx, "/rocket.png")?;
        Ok(Assets {player_image })
    }

    fn actor_image(&self, actor: &Actor) -> &graphics::Image {
        match actor.tag {
            ActorType::Player => &self.player_image,
        }
    }
}

// **********************************************************************
// Keeps track of the user's input state 
// Turn keyboard events into state-based commands
// **********************************************************************
#[derive(Debug)]
struct InputState {
    xaxis: f32,
    yaxis: f32,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
        }
    }
}

// **********************************************************************
// MainState is our game's "global" state
// Keeps track of everything we need for actually running the game.
// **********************************************************************
struct MainState {
    screen: graphics::ScreenImage,
    player: Actor,
    assets: Assets,
    input: InputState,
    ground_rect: Rect
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let player = create_player();
        
        let ground_rect = graphics::Rect::new(0.0, 580.0, 600.0, 20.0);

        let screen =
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);

        let s = MainState {
            screen,
            player,
            assets,
            input: InputState::default(),
            ground_rect
        };

        Ok(s)
    }
}

// **********************************************************************
// EventHandler (ggez::event) 
// responsable for updating, drawing game objects,and handling input events.
// **********************************************************************
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //PRINT PLAYER POSITION
        // println!("PLAYER POS X: {}", self.player.pos.x);
        // println!("PLAYER POS Y: {}", self.player.pos.y);

        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = GRAVITY_ACCELERATION / (DESIRED_FPS as f32);
            
            // Update the player state based on the user input.
            player_handle_input(&mut self.player, &self.input, seconds);

            // Update the physics for player
            update_actor_position(&mut self.player, seconds);
            
            // wrap_actor_position(&mut self.player, self.screen_width, self.screen_height);

            check_collision(&mut self.player, self.ground_rect, ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Draw Canvas
        let mut canvas = graphics::Canvas::from_screen_image(ctx, &mut self.screen, Color::BLACK);

        // Loop over all objects
        {
            let assets = &mut self.assets;

            let p = &self.player;
            draw_actor(assets, &mut canvas, p);

        }

        // Create mesh for the pad
        let ground_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ground_rect,
            graphics::Color::WHITE,
        )?;
        
        // Drawing pads
        let draw_param = graphics::DrawParam::default().dest(Vec2::ZERO);
        canvas.draw(
            &ground_mesh,
            draw_param
        );

        canvas.finish(ctx)?;

        ctx.gfx.present(&self.screen.image(ctx))?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool, ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.input.yaxis = 1.0;
            }
            Some(KeyCode::Left) => {
                self.input.xaxis = -1.0;
            }
            Some(KeyCode::Right) => {
                self.input.xaxis = 1.0;
            }
            Some(KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                self.input.yaxis = 0.0;
            }
            Some(KeyCode::Left) => {
                self.input.xaxis = 0.0;
            }
            Some(KeyCode::Right) => {
                self.input.xaxis = 0.0;
            }
            _ => (), // Do nothing
        }
        Ok(())
    }
}



pub fn main() -> GameResult {
    // Access resource folder
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("rocket-game", "Thiago")
        .window_setup(conf::WindowSetup::default()
            .title("Rocket Game!"))
        .window_mode(conf::WindowMode::default()
            .dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let game_state = MainState::new(&mut ctx)?;

    event::run(ctx, events_loop, game_state)
}