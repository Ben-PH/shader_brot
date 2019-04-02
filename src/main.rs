// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

    // DONE set it up so that you can idiomatically select for mandel or julia
    // TODO refactor so more idiomatic in the code structure
    constant Mandel {
        center: [f64; 2] = "u_Center",
        dimension: [f64; 2] = "u_Dimension",
        resolution: [f64; 2] = "u_Resolution",
        position: [f32; 2] = "u_MousePos",
        time: f32 = "u_Time",
        max_iter: i32 = "u_MaxIteration",
        is_mandel: i32 = "u_IsMandel",
    }
}

impl Mandel {
    fn new(ctx: &Context) -> Self {
        Self {
            // TODO varify that these uniforms are set in cpu only
            // TODO delegate operations on these uniforms
            position: [0.0, 0.0],
            center: [-0.5, -0.0],
            dimension: [3.0, 2.0],
            time: 0.0,
            max_iter: 120,
            resolution: [graphics::size(ctx).0, graphics::size(ctx).1],
            is_mandel: 1,
        }
    }

}


#[derive (Debug)]
struct MainState {
    // TODO collect these items in a refactor
    canvas_render_target: Canvas,
    zoom: f64,

    mandel_uniforms: Mandel,
    shader: graphics::Shader<Mandel>,
}


const ITER_STEP: i32 = 5;

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {

        let canvas_render_target = Canvas::with_window_size(ctx)?;

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


        // bring it all together
        Ok(Self {
            canvas_render_target,
            zoom: 1.0,
            mandel_uniforms,
            shader,
        })
    }

    fn set_origin(&mut self, center: [f64; 2]) {
        self.mandel_uniforms.center = center;
    }

    // TODO command depends on this. change so this depends on command
    // by doing += argument
    fn incriment_max_iter(&mut self) {
        self.mandel_uniforms.max_iter += ITER_STEP;
    }

    // TODO command depends on this. change so this depends on command
    // by doing += argument
    fn decriment_max_iter(&mut self) {
        let it = self.mandel_uniforms.max_iter;
        let it = std::cmp::max(2, it - ITER_STEP);
        self.mandel_uniforms.max_iter = it;
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        // time since last frame to provide to the shader
        let shader_time = ggez::timer::time_since_start(ctx);
        let shader_time = ggez::timer::duration_to_f64(shader_time) * SPEED_SCALE;
        self.mandel_uniforms.time = shader_time as f32;

        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {

        // We want everything to scale according to a screen-pixel. With a new
        // window size, the shader needs new information on pixel resolution
        let os_scale = graphics::os_hidpi_factor(ctx);
        self.mandel_uniforms.resolution = [(width*os_scale) as f64, (height*os_scale) as f64];
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.3, 1.0].into());

        // Consider the graphics::Canvas object a software-abstraction of the
        // computer screen, and graphics::present as its unveiling. Here, we
        // have the gpu paint to a clean canvas without interuption, then unveil it.
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

        // As with `resize_event()` needing to adjust to match Canvas pixels,
        // we need to do the same with mouse position.
        let scale = graphics::os_hidpi_factor(ctx);

        let y = graphics::size(ctx).1 as f32  - y;
        self.mandel_uniforms.position[0] = x * scale;
        self.mandel_uniforms.position[1] = y * scale;
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
        // TODO move this into a function call
        match Direction::from_keycode(keycode) {
            Some(Direction::Up)    => self.mandel_uniforms.center[1] += self.mandel_uniforms.dimension[1] * 0.2,
            Some(Direction::Down)  => self.mandel_uniforms.center[1] -= self.mandel_uniforms.dimension[1] * 0.2,
            Some(Direction::Left)  => self.mandel_uniforms.center[0] -= self.mandel_uniforms.dimension[0] * 0.2,
            Some(Direction::Right) => self.mandel_uniforms.center[0] += self.mandel_uniforms.dimension[0] * 0.2,
            None => {},
        }

        // TODO make this redundant then delete
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
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ggez::ContextBuilder::new("shader-driven julia/mandelbrot", "BenPH").add_resource_path(resource_dir).with_conf_file(true);
    let (ctx, event_loop) = &mut cb.build()?;
    ctx.conf.window_mode = ggez::conf::WindowMode::resizable(ctx.conf.window_mode, true);

    let mut ms = MainState::new(ctx)?;

    ggez::event::run(ctx, event_loop, &mut ms)
}
