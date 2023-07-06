use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::input::keyboard::KeyInput;

// Not using
use oorandom::Rand32;

use std::env;
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

// Not using (not handle any collision)
// Player Box size
const PLAYER_BBOX: f32 = 12.0;

// **********************************************************************
// Game Generic Consts
// **********************************************************************
const MAX_PHYSICS_VEL: f32 = 250.0;
const DESIRED_FPS: u32 = 60;

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
    ang_vel: f32,
    bbox_size: f32,
}

// **********************************************************************
// Utility functions.
// **********************************************************************
// Create a unit vector representing the given angle (in radians)
fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

// Draw actor
fn draw_actor(
    assets: &mut Assets,
    canvas: &mut graphics::Canvas,
    actor: &Actor,
    world_coords: (f32, f32),
) {
    let (screen_w, screen_h) = world_coords;

    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);

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
        pos: Point2::ZERO,
        facing: 0.,
        velocity: Vector2::ZERO,
        ang_vel: 0.,
        bbox_size: PLAYER_BBOX,
    }
}

// **************************
// Rocket Physics
// **************************
fn player_handle_input(actor: &mut Actor, input: &InputState, dt: f32) {
    actor.facing += dt * PLAYER_TURN_RATE * input.xaxis;

    if input.yaxis > 0.0 {
        player_thrust(actor, dt);
    }
}

fn player_thrust(actor: &mut Actor, dt: f32) {
    let direction_vector = vec_from_angle(actor.facing);
    let thrust_vector = direction_vector * (PLAYER_THRUST);
    actor.velocity += thrust_vector * (dt);
}

fn update_actor_position(actor: &mut Actor, dt: f32) {
    // Clamp the velocity to the max efficiently
    let norm_sq = actor.velocity.length_squared();
    if norm_sq > MAX_PHYSICS_VEL.powi(2) {
        actor.velocity = actor.velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
    }
    let dv = actor.velocity * dt;
    actor.pos += dv;
    actor.facing += actor.ang_vel;
}

// Takes an actor and wraps its position to the bounds of the screen, 
// so if it goes off the left side of the screen it will re-enter on the right side and so on.
fn wrap_actor_position(actor: &mut Actor, sx: f32, sy: f32) {
    // Wrap screen
    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;
    if actor.pos.x > screen_x_bounds {
        actor.pos -= Vec2::new(sx, 0.0);
    } else if actor.pos.x < -screen_x_bounds {
        actor.pos += Vec2::new(sx, 0.0);
    };
    if actor.pos.y > screen_y_bounds {
        actor.pos -= Vec2::new(0.0, sy);
    } else if actor.pos.y < -screen_y_bounds {
        actor.pos += Vec2::new(0.0, sy);
    }
}

// fn handle_timed_life(actor: &mut Actor, dt: f32) {
//     actor.life -= dt;
// }

// Translates the world coordinate system, which
// has Y pointing up and the origin at the center,
// to the screen coordinate system, which has Y
// pointing downward and the origin at the top-left,
fn world_to_screen_coords(screen_width: f32, screen_height: f32, pos: Point2) -> Point2 {
    let x = pos.x + screen_width / 2.0;
    let y = screen_height - (pos.y + screen_height / 2.0);
    Point2::new(x, y)
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
    screen_width: f32,
    screen_height: f32,
    input: InputState,
    rng: Rand32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // RNG Seed
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        // let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        let rng = Rand32::new(u64::from_ne_bytes(seed));


        let assets = Assets::new(ctx)?;
        let player = create_player();

        let (width, height) = ctx.gfx.drawable_size();
        let screen =
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);

        let s = MainState {
            screen,
            player,
            assets,
            screen_width: width,
            screen_height: height,
            input: InputState::default(),
            rng
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
            let seconds = 1.0 / (DESIRED_FPS as f32);
            
            // Update the player state based on the user input.
            player_handle_input(&mut self.player, &self.input, seconds); 

            // Update the physics for player
            update_actor_position(&mut self.player, seconds);
            wrap_actor_position(&mut self.player, self.screen_width, self.screen_height);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Draw Canvas
        let mut canvas = graphics::Canvas::from_screen_image(ctx, &mut self.screen, Color::BLACK);

        // Loop over all objects
        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            let p = &self.player;
            draw_actor(assets, &mut canvas, p, coords);
        }

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
            Some(KeyCode::Left | KeyCode::Right) => {
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