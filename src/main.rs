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

const SPEED_SCALE: f64 = 0.3;

gfx_defines! {

    // Input uniforms that get passed into the shader
    // TODO set it up so that you can idiomatically select for mandel or julia
    constant Mandel {
        position: [f64; 2] = "u_MousePos",
        center: [f64; 2] = "u_Center",
        dimension: [f64; 2] = "u_Dimension",
        resolution: [f64; 2] = "u_Resolution",
        time: f32 = "u_Time",
        max_iter: i32 = "u_MaxIter",
        is_mandel: i32 = "u_IsMandel",
    }
}

impl Mandel {
    fn new(ctx: &Context) -> Self {
        Self {
            position: [0.0, 0.0],
            center: [-0.5, -0.0], // TODO check if this is removable duplicate
            dimension: [3.5, 2.0], // TODO check if this is removable duplicate
            time: 0.0,
            max_iter: 120,
            resolution: [graphics::size(ctx).0, graphics::size(ctx).1],
            is_mandel: 1,
        }
    }

}


#[derive (Debug)]
struct MainState {
    canvas_render_target: Canvas,
    zoom: f64,

    mandel_uniforms: Mandel,
    shader: graphics::Shader<Mandel>,
}
const ITER_STEP: i32 = 5;

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        let mandel_uniforms = Mandel::new(ctx);

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
            zoom: 1.0,
            mandel_uniforms,
            shader,
        })
    }

    // fn resolution_center_origin(ctx: &Context, pos: &mut[f32; 2]) {
    //     pos[0] -= ctx.conf.window_mode.width/2.0;
    //     pos[1] -= ctx.conf.window_mode.height/2.0;
    // }

    fn set_origin(&mut self, center: [f64; 2]) {
        self.mandel_uniforms.center = center;
    }

    fn incriment_max_iter(&mut self) {
        self.mandel_uniforms.max_iter += ITER_STEP;
    }

    fn decriment_max_iter(&mut self) {
        let it = self.mandel_uniforms.max_iter;
        let it = std::cmp::max(2, it - ITER_STEP);
        self.mandel_uniforms.max_iter = it;
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // shader uniform update
        let shader_time = ggez::timer::time_since_start(ctx);
        let shader_time = ggez::timer::duration_to_f64(shader_time) * SPEED_SCALE;
        
        self.mandel_uniforms.time = shader_time as f32;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        // TODO get from OS rather than hard-code the scale
        // self.mandel_uniforms.update_resolution(ctx);
        // let im = self.canvas_render_target.image();
        let os_scale = graphics::os_hidpi_factor(ctx);
        self.mandel_uniforms.resolution = [(width*os_scale) as f64, (height*os_scale) as f64];
        println!("size: {:?}", self.mandel_uniforms.resolution);
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

    fn mouse_motion_event(
        &mut self,
        ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32
    ) {
        let scale = graphics::os_hidpi_factor(ctx);

        let y = graphics::size(ctx).1 as f32  - y;
        self.mandel_uniforms.position[0] = (x*scale) as f64;
        self.mandel_uniforms.position[1] = (y*scale) as f64;
        println!("pos: {:?}", self.mandel_uniforms.position);

        // MainState::resolution_center_origin(&ctx, &mut self.mandel_uniforms.position);

    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {

        // TODO work out how to decouble responsibility
        let mut zoom_coeficient = (0.0, 0.0);
        match Direction::from_keycode(keycode) {
            Some(Direction::Up)    => self.mandel_uniforms.center[1] += self.mandel_uniforms.dimension[1] * 0.2,
            Some(Direction::Down)  => self.mandel_uniforms.center[1] -= self.mandel_uniforms.dimension[1] * 0.2,
            Some(Direction::Left)  => self.mandel_uniforms.center[0] -= self.mandel_uniforms.dimension[0] * 0.2,
            Some(Direction::Right) => self.mandel_uniforms.center[0] += self.mandel_uniforms.dimension[0] * 0.2,
            None => {},
        }

        self.mandel_uniforms.center[0] += (zoom_coeficient.0 / self.zoom) as f64;
        self.mandel_uniforms.center[1] += (zoom_coeficient.1 / self.zoom) as f64;

        // TODO add comments to template high-level representation of planned commands
        match keycode {
            KeyCode::E => {
                self.mandel_uniforms.dimension[0] *= 0.9;
                self.mandel_uniforms.dimension[1] *= 0.9;
            }
            KeyCode::W => self.incriment_max_iter(),
            KeyCode::S => self.decriment_max_iter(),
            KeyCode::D => {
                self.mandel_uniforms.dimension[0] *= 1.1;
                self.mandel_uniforms.dimension[1] *= 1.1;
            },
            KeyCode::Q => {
                println!("MainState\n=========\n {:#?}", &self);
                ggez::quit(ctx);
            },
            KeyCode::Tab => {
                match self.mandel_uniforms.is_mandel {
                    0 => self.mandel_uniforms.is_mandel = 1,
                    _ => self.mandel_uniforms.is_mandel = 0,
                }
            },
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
    ctx.conf.window_mode = ggez::conf::WindowMode::resizable(ctx.conf.window_mode, true);

    let mut ms = MainState::new(ctx)?;

    ggez::event::run(ctx, event_loop, &mut ms)
}
