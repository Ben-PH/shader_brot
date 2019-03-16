#[macro_use]
extern crate gfx;
extern crate cgmath;

use ggez::graphics::{self, DrawParam, Canvas};
use ggez::Context;
use ggez::event::{ KeyCode, KeyMods };
use ggez::GameResult;

use std::env;
use std::path;

mod direction;
use direction::*;

gfx_defines! {

    // Input uniforms that get passed into the shader
    // TODO set it up so that you can idiomatically select for mandel or julia
    constant Mandel {
        position: [f32; 2] = "u_MousePos",
        center: [f32; 2] = "u_Center",
        dimension: [f32; 2] = "u_Dimensions",
        time: f32 = "u_Time",
        zoom: f32 = "u_Zoom",
    }
}


#[derive (Debug)]
struct MainState {
    canvas_render_target: Canvas,
    center: cgmath::Point2<f32>,
    zoom: f32,

    mandel_uniforms: Mandel,
    shader: graphics::Shader<Mandel>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        let mandel_uniforms = Mandel{
            position: [0.0, 0.0],
            center: [0.0, 0.0], // TODO check if this is removable duplicate
            time: 0.0,
            zoom: 1.0,// TODO check if this is removable duplicate
            dimension: [graphics::size(ctx).0 as f32, graphics::size(ctx).1 as f32],
        };

        let shader = graphics::Shader::from_u8(
            ctx,
            // vertex source-code
            include_bytes!("../resources/basic_330.glslv"),
            // fragment source-code
            include_bytes!("../resources/fractal.glslf"),
            mandel_uniforms,
            "Mandel",
            None
        )?;

        let canvas_render_target = Canvas::with_window_size(ctx)?;

        // bring it all together
        Ok(Self {
            canvas_render_target,
            center: [0.0, -0.5].into(),// TODO check if this is removable duplicate
            zoom: 1.0,// TODO check if this is removable duplicate
            mandel_uniforms,
            shader,
        })
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // shader uniform update
        let shader_time = ggez::timer::time_since_start(ctx);
        self.mandel_uniforms.time = ggez::timer::duration_to_f64(shader_time) as f32;
        Ok(())
    }

    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) {
        // TODO get from OS rather than hard-code the scale
        self.mandel_uniforms.dimension[0] = width * 1.5;
        self.mandel_uniforms.dimension[1] = height * 1.5;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.3, 1.0].into());
        {
            let _lock = graphics::use_shader(ctx, &self.shader);
            self.shader.send(ctx, self.mandel_uniforms)?;
            graphics::draw(ctx, &self.canvas_render_target, DrawParam::default())?;
            graphics::present(ctx)?;
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {

        // TODO work out how to decouble responsibility
        let mut zoom_coeficient = (0.0f32, 0.0f32);
        match Direction::from_keycode(keycode) {
            Some(Direction::Up)    => zoom_coeficient = (0.0, 1.0),
            Some(Direction::Down)  => zoom_coeficient = (0.0, -1.0),
            Some(Direction::Left)  => zoom_coeficient = (-1.0, 0.0),
            Some(Direction::Right) => zoom_coeficient = (1.0, 0.0),
            None => {},
        }

        self.mandel_uniforms.center[0] -= zoom_coeficient.0 / self.mandel_uniforms.zoom;
        self.mandel_uniforms.center[1] -= zoom_coeficient.1 / self.mandel_uniforms.zoom;

        // TODO add comments to template high-level representation of planned commands
        match keycode {
            KeyCode::Period => self.mandel_uniforms.zoom *= 1.0 + 1.0/10.0,
            KeyCode::E => self.mandel_uniforms.zoom *= 1.0 - 1.0/10.000_01,
            KeyCode::Q => {
                println!("MainState\n=========\n {:#?}", &self);
                ggez::quit(ctx);
            }
            _ => {}
        }
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
