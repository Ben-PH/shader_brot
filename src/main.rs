#[macro_use]
extern crate gfx;
extern crate cgmath;

use ggez::graphics::{self, DrawParam, Canvas};
use ggez::Context;
use ggez::event::{ KeyCode, KeyMods };
use ggez::GameResult;

use std::env;
use std::path;
gfx_defines! {
    constant Mandel {
        position: [f32; 2] = "u_MousePos",
        center: [f32; 2] = "u_Center",
        dimension: [f32; 2] = "u_Dimensions",
        time: f32 = "u_Time",
        zoom: f32 = "u_Zoom",
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}
#[derive (Debug)]
struct MainState {
    canvas: Canvas,
    center: cgmath::Point2<f32>,
    zoom: f32,
    mand: Mandel,

    shad: graphics::Shader<Mandel>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        let mand = Mandel{
            position: [0.0, 0.0],
            center: [0.0, 0.0],
            time: 0.0,
            zoom: 1.0,
            dimension: [graphics::size(ctx).0 as f32, graphics::size(ctx).1 as f32],
        };
        let thing1 = include_bytes!("../resources/basic_330.glslv");
        let thing2 = include_bytes!("../resources/fractal.glslf");
        let canvas = Canvas::with_window_size(ctx)?;
        let shad = graphics::Shader::from_u8(
            ctx,
            thing1,
            thing2,
            mand,
            "Mandel",
            None
        )?;

        Ok(Self {
            canvas,
            center: [0.0, -0.5].into(),
            zoom: 1.0,
            mand,
            shad,
        })
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let time = ggez::timer::time_since_start(ctx);
        self.mand.time = ggez::timer::duration_to_f64(time) as f32;
        Ok(())
    }
    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        self.mand.dimension[0] = width * 1.5;
        self.mand.dimension[1] = height * 1.5;
        // println!("dim: {:?}", self.mand.dimension);

    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {

        let mut zoom_coeficient = (0.0f32, 0.0f32);
        match Direction::from_keycode(keycode) {
            Some(Direction::Up)    => zoom_coeficient = (0.0, 1.0),
            Some(Direction::Down)  => zoom_coeficient = (0.0, -1.0),
            Some(Direction::Left)  => zoom_coeficient = (-1.0, 0.0),
            Some(Direction::Right) => zoom_coeficient = (1.0, 0.0),
                None => {},
        }

        self.mand.center[0] -= zoom_coeficient.0 / self.mand.zoom;
        self.mand.center[1] -= zoom_coeficient.1 / self.mand.zoom;
        // self.mand.center[0] *= ZOOM_SCALE;
        // self.mand.center[1] *= ZOOM_SCALE;

        match keycode {
            KeyCode::Period => self.mand.zoom *= 1.0 + 1.0/10.0,
            KeyCode::E => self.mand.zoom *= 1.0 - 1.0/10.000_01,
            KeyCode::Q => {
                println!("MainState\n=========\n {:#?}", &self);
                ggez::quit(ctx);
            }
            _ => {}
        }
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.3, 1.0].into());
        {
            let _lock = graphics::use_shader(ctx, &self.shad);
            self.shad.send(ctx, self.mand)?;
            // let foreground = Canvas::with_window_size(ctx)?;
            graphics::draw(ctx, &self.canvas, DrawParam::default())?;
            graphics::present(ctx)?;
        }
        Ok(())
    }

}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("shader", "moi").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let mut ms = MainState::new(ctx)?;

    ggez::event::run(ctx, event_loop, &mut ms)

}
