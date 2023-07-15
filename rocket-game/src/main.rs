use ggez::audio::SoundSource;
use ggez::{Context, ContextBuilder, GameResult, audio};
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, Rect, PxScale, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};

use core::time;
use std::iter::Iterator;
use std::{env, thread};
use std::f32::consts::PI;
use std::path;

// **********************************************************************
// Player Consts
// **********************************************************************
// Acceleration in pixels per second.
const ROCKET_THRUST: f32 = 25.0;
// Rotation in radians per second.
const ROCKET_TURN_RATE: f32 = 1.5;
// Player Box size
const ROCKET_BBOX: Vec2 = Vec2::new(37.0, 64.0);
const ROCKET_FUEL: f32 = 10000.0;

// **********************************************************************
// Game Generic Consts
// **********************************************************************
const DESIRED_FPS: u32 = 60;
const SCREEN_SIZE: Vec2 = Vec2::new(1600.0, 900.0);
const MAX_IMPACT_VELOCITY: f32 = 75.0;
const GRAVITY_ACCELERATION: f32 = 3.0;

// **************************
// Creating objects
// **************************
enum ObjectType {
    CheckpointGround,
    Ground,
    Wall,
    Fuel
}

struct Objects {
    rect: Rect,
    tag: ObjectType
}

fn create_objects() -> Vec<Objects> {
    let mut objects_vec:Vec<Objects> = Vec::new();
    
    // Draw the walls and ground on map
    let ground_rect =  Objects {
        rect: graphics::Rect::new(50.0, 580.0, 100.0, 20.0),
        tag: ObjectType::Ground
    };

    let checkpoint_ground_rect =  Objects {
        rect: graphics::Rect::new(1450.0, 580.0, 100.0, 20.0),
        tag: ObjectType::CheckpointGround
    };

    let wall_checkpoint_rect =  Objects {
        rect: graphics::Rect::new(1449.0, 582.0, 102.0, 318.0),
        tag: ObjectType::Wall
    };

    let wall_1_rect =  Objects {
        rect: graphics::Rect::new(320.0, 300.0, 20.0, 600.0),
        tag: ObjectType::Wall
    };

    let wall_2_rect =  Objects {
        rect: graphics::Rect::new(580.0, 0.0, 20.0, 600.0),
        tag: ObjectType::Wall
    };

    let wall_3_rect =  Objects {
        rect: graphics::Rect::new(1020.0, 0.0, 20.0, 600.0),
        tag: ObjectType::Wall
    };

    let wall_4_rect =  Objects {
        rect: graphics::Rect::new(1280.0, 300.0, 20.0, 600.0),
        tag: ObjectType::Wall
    };

    // Mid wall
    let wall_5_rect =  Objects {
        rect: graphics::Rect::new(800.0, 300.0, 20.0, 600.0),
        tag: ObjectType::Wall
    };

    let fuel_rect = Objects {
        rect: graphics::Rect::new(768.0, 100.0, 64.0, 64.0),
        tag: ObjectType::Fuel
    };

    objects_vec.push(ground_rect);

    objects_vec.push(checkpoint_ground_rect);

    objects_vec.push(wall_checkpoint_rect);

    objects_vec.push(wall_1_rect);
    objects_vec.push(wall_2_rect);
    objects_vec.push(wall_3_rect);
    objects_vec.push(wall_4_rect);
    objects_vec.push(wall_5_rect);

    objects_vec.push(fuel_rect);

    objects_vec
}



// **************************
// Draw Functions
// **************************
fn draw_rocket(assets: &mut Assets, canvas: &mut graphics::Canvas, actor: &Player) {
    let image = assets.rocket_sprite();

    let drawparams = graphics::DrawParam::new()
        .dest(actor.pos)
        .rotation(actor.facing)
        .offset(Vec2::new(0.5, 0.5));

    canvas.draw(image, drawparams);
}

fn draw_objects(ctx: &mut Context, canvas: &mut graphics::Canvas, assets: &Assets, objects_vec: &Vec<Objects>) -> GameResult{
    for object in objects_vec {
        // ****************************
        // Draw CheckPoint Ground
        // ****************************
        if matches!(object.tag, ObjectType::CheckpointGround) {
            let object_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                object.rect,
                graphics::Color::MAGENTA,
            )?;

            // Drawing ground
            let draw_param = graphics::DrawParam::default();
            canvas.draw(&object_mesh, draw_param);
        }

        // ****************************
        // Draw Ground
        // ****************************
        // Checks if object is a ground object
        if matches!(object.tag, ObjectType::Ground) {
            let object_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                object.rect,
                graphics::Color::WHITE,
            )?;

            // Drawing ground
            let draw_param = graphics::DrawParam::default();
            canvas.draw(&object_mesh, draw_param);
        }

        // ****************************
        // Draw Walls
        // ****************************
        // Checks if object is wall object
        if matches!(object.tag, ObjectType::Wall) {
            let object_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                object.rect,
                graphics::Color::WHITE,
            )?;

            // Drawing wall
            let draw_param = graphics::DrawParam::default();
            canvas.draw(&object_mesh, draw_param);
        }

        // ****************************
        // Draw Fuel Collectable
        // ****************************
        if objects_vec.iter().any(|x| matches!(x.tag, ObjectType::Fuel)) {
            // Find fuel object index, inside objects vector
            let fuel_index = objects_vec.iter().position(|x| matches!(x.tag, ObjectType::Fuel) ).unwrap();
            let fuel_rect = &objects_vec[fuel_index];
            
            let image = assets.fuel_sprite();
    
            let fuel_pos = Vec2::new(fuel_rect.rect.x, fuel_rect.rect.y);
            let drawparams = graphics::DrawParam::new().dest(fuel_pos);
        
            canvas.draw(image, drawparams);
        }
    }

    Ok(())
}



// **************************
// Utility functions
// **************************
// Create a unit vector representing the given angle (in radians)
fn vec_from_angle(angle: f32) -> Vec2 {
    let x = angle.sin();
    let y = angle.cos();
    Vec2::new(x, -y)
}

fn move_wall_func(move_wall: &mut bool, objects_vec: &mut Vec<Objects>) {
    if objects_vec[7].rect.y == 300.0 {
        *move_wall = true;
    }

    if objects_vec[7].rect.y == 600.0 {
        *move_wall = false;
    }

    if *move_wall {
        objects_vec[4].rect.y -= 1.0;
        objects_vec[5].rect.y -= 1.0;
        objects_vec[7].rect.y += 1.0;
    } else {
        objects_vec[4].rect.y += 1.0;
        objects_vec[5].rect.y += 1.0;
        objects_vec[7].rect.y -= 1.0;
    }
}


// ****************************************************
// Player functions
// ****************************************************
#[derive(Debug)]
struct Player {
    pos: Vec2,
    facing: f32,
    velocity: Vec2,
    fuel: f32,
    rect: Rect
}

// Create PLayer
fn create_player() -> Player {
    Player {
        pos: Vec2::new(100.0, 400.0),
        facing: 0.0,
        velocity: Vec2::ZERO,
        fuel: ROCKET_FUEL,
        
        // Rect object stays "inside" player sprite to check collisions
        rect: Rect::new(0.0, 0.0, ROCKET_BBOX.x, ROCKET_BBOX.y)
    }
}

// **************************
// Rocket Physics
// **************************
fn player_handle_input(rocket: &mut Player, input: &InputState, dt: f32) {
    // Rocket rotation
    rocket.facing += dt * ROCKET_TURN_RATE * input.xaxis;
    rocket.facing = rocket.facing % (2.0 * PI);
    
    // Rocket thrust
    if (input.yaxis > 0.0) && (rocket.fuel > 0.0) {
        rocket_thrust(rocket, dt);
    }
}

fn rocket_thrust(rocket: &mut Player, dt: f32) {
    let direction_vector = vec_from_angle(rocket.facing);
    let thrust_vector = direction_vector * (ROCKET_THRUST);

    rocket.velocity += thrust_vector * (dt);

    if rocket.fuel > 0.0 {
        rocket.fuel -= 0.5;
    }
}

fn update_player_position(rocket: &mut Player, dt: f32) {
    rocket.velocity.y += 10.0 * dt;

    rocket.pos += rocket.velocity * dt;
    
    // Update rect position that stays "inside" the rocket
    rocket.rect.x = rocket.pos.x - rocket.rect.w / 2.0;
    rocket.rect.y = rocket.pos.y - rocket.rect.h / 2.0;
}



// **********************************************************************
// So that was the real meat of our game.  Now we just need a structure
// to contain the images, sounds, etc. that we need to hang on to; this
// is our "asset management system".  All the file names and such are
// just hard-coded.
// **********************************************************************
struct Assets {
    rocket_sprite: graphics::Image,
    fuel_sprite: graphics::Image,
    hit_sound: audio::Source
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let rocket_sprite = graphics::Image::from_path(ctx, "/rocket.png")?;
        let fuel_sprite = graphics::Image::from_path(ctx, "/fuel.png")?;
        let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {rocket_sprite, fuel_sprite, hit_sound})
    }

    fn rocket_sprite(&self) -> &graphics::Image {
        &self.rocket_sprite
    }

    fn fuel_sprite(&self) -> &graphics::Image {
        &self.fuel_sprite
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
// Keeps track of everything we need for running the game.
// **********************************************************************
struct MainState {
    screen: graphics::ScreenImage,
    player: Player,
    assets: Assets,
    input: InputState,
    objects_vec: Vec<Objects>,
    rocket_velocity_text: Text,
    rocket_fuel_text: Text,
    move_wall: bool
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let screen = graphics::ScreenImage::new(
            ctx, 
            graphics::ImageFormat::Rgba8UnormSrgb, 
            1.0, 
            1.0,
            1);
        let player = create_player();
        let assets = Assets::new(ctx)?;
        let objects_vec = create_objects();
        let rocket_velocity_text = graphics::Text::new(format!("{}", 0));
        let rocket_fuel_text= graphics::Text::new(format!("{}", ROCKET_FUEL));



        let move_wall: bool = true;



        let s = MainState {
            screen,
            player,
            assets,
            input: InputState::default(),
            objects_vec,
            rocket_velocity_text,
            rocket_fuel_text,
            move_wall
        };

        Ok(s)
    }

    fn check_collision(&mut self, ctx: &mut ggez::Context) {
        let duration = time::Duration::from_secs(1);
        
        // *****************************
        // Collision with walls/ground
        // *****************************
        for object in &self.objects_vec {
            if object.rect.overlaps(&self.player.rect) {
                // *****************************
                // Ground Collision
                // *****************************
                if matches!(object.tag, ObjectType::Ground | ObjectType::CheckpointGround) {
                    // Checks impact velocity
                    if self.player.velocity.length() >= MAX_IMPACT_VELOCITY {     
                        let _ = self.assets.hit_sound.play(ctx);
                        thread::sleep(duration);
                        ctx.request_quit();
                    }

                    // Checks collision with checkpoint ground
                    if matches!(object.tag, ObjectType::CheckpointGround) {
                        println!("You Won!");
                        thread::sleep(duration);
                        ctx.request_quit();
                    };

                    // Update physics
                    self.player.velocity.y *= -0.15;
                    self.player.velocity.x *= 0.99;
                    self.player.pos.y = self.objects_vec[0].rect.y - self.player.rect.h / 2.0;
                }

                // *****************************
                // Walls Collision
                // *****************************
                if matches!(object.tag, ObjectType::Wall) {
                    let _ = self.assets.hit_sound.play(ctx);
                    thread::sleep(duration);
                    ctx.request_quit();
                }
            }
        }

        // *****************************
        // Collision with fuel
        // *****************************
        self.objects_vec.retain(|object| {
            let should_keep = !object.rect.overlaps(&self.player.rect) || !matches!(object.tag, ObjectType::Fuel);
    
            if !should_keep {
                self.player.fuel += 1000.0;
            }
    
            should_keep
        });
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

        // Deciding when to update the game, and how many times.
        // Run once for each frame fitting in the time since the last update.
        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = GRAVITY_ACCELERATION / (DESIRED_FPS as f32);
            
            // Update the player state based on the user input.
            player_handle_input(&mut self.player, &self.input, seconds);

            // Update the physics for player
            update_player_position(&mut self.player, seconds);

            // Check rocket collision with objects
            self.check_collision(ctx);

            // Update rocket fuel
            self.rocket_fuel_text = graphics::Text::new(format!("{:.2?}", self.player.fuel));

            // Update player velocity
            let mut mag = (self.player.velocity.x.powi(2)) + (self.player.velocity.y.powi(2));
            mag = mag.sqrt();
            self.rocket_velocity_text = graphics::Text::new(format!("{:.2}", mag));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Draw Canvas
        let mut canvas = graphics::Canvas::from_screen_image(ctx, &mut self.screen, Color::BLACK);

        // ****************************
        // Draw Player
        // ****************************
        draw_rocket(&mut self.assets, &mut canvas, &self.player);

        // ****************************
        // Draw Objects
        // ****************************
        let _ = draw_objects(ctx, &mut canvas, &self.assets, &self.objects_vec);

        move_wall_func(&mut self.move_wall, &mut self.objects_vec);

        // ********************************************************
        // Draw Texts
        // ********************************************************
        let text_size = PxScale::from(24.0);

        // ****************************
        // Draw rocket velocity
        // ****************************
        let velocity_text_pos = ggez::glam::Vec2::new(0.0, 270.0);
        let velocity_text_pos_2 = ggez::glam::Vec2::new(0.0, 250.0);

        // **************
        // Velocity Number
        // **************
        self.rocket_velocity_text.set_scale(text_size);

        let draw_param = graphics::DrawParam::default()
            .dest(velocity_text_pos)
            .color(ggez::graphics::Color::WHITE);

        canvas.draw(&self.rocket_velocity_text,  draw_param);

        // **************
        // Velocity Text
        // **************
        let mut velocity_text = graphics::Text::new(format!("Velocity:"));
        velocity_text.set_scale(text_size);

        let draw_param = graphics::DrawParam::default()
            .dest(velocity_text_pos_2)
            .color(ggez::graphics::Color::WHITE);

        canvas.draw(&velocity_text,  draw_param);



        // ****************************
        // Draw Rocket fuel
        // ****************************
        let fuel_text_pos = ggez::glam::Vec2::new(0.0, 340.0);
        let fuel_text_pos_2 = ggez::glam::Vec2::new(0.0, 320.0);

        // **************
        // Fuel Number
        // **************
        self.rocket_fuel_text.set_scale(text_size);

        let draw_param = graphics::DrawParam::default()
            .dest(fuel_text_pos)
            .color(ggez::graphics::Color::WHITE);

        canvas.draw(&self.rocket_fuel_text, draw_param);

        // **************
        // Fuel Text
        // **************
        let mut fuel_text = graphics::Text::new(format!("Fuel:"));
        fuel_text.set_scale(text_size);

        let draw_param = graphics::DrawParam::default()
            .dest(fuel_text_pos_2)
            .color(ggez::graphics::Color::WHITE);

        canvas.draw(&fuel_text, draw_param);



        // ****************************
        // Finish Drawing
        // ****************************
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
    let resource_dir = 
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");

            path
        } else {
            path::PathBuf::from("./resources")
        };

    // Setup metadata about our game
    let cb = ContextBuilder::new("rocket-game", "Thiago")
        .window_setup(conf::WindowSetup::default()
            .title("Rocket Game!"))
        .window_mode(conf::WindowMode::default()
            .dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let game_state = MainState::new(&mut ctx)?;

    // Run our game, passing in our context and state.
    event::run(ctx, events_loop, game_state)
}