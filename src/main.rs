#![feature(box_syntax, box_patterns, rand)]
#![allow(warnings)]
#[macro_use]
extern crate dump;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate shapes;
extern crate palette;
extern crate rand;
extern crate find_folder;
extern crate clipboard;




use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, glyph_cache};
use piston::event_loop::*;
use shapes::{Line, Rect};
use piston::input::*;
use graphics::*;
use palette::*;
use find_folder::*;

mod window;
mod color;
use color::{col_str, rgb_to_hex};
mod mouse;
use mouse::*;
mod state;
use state::*;
mod ui;
mod copy;

const H: f64 = 400.0;
const h1: f64 = 50.0;
const h2: f64 = 125.0;
const h3: f64 = 200.0;
const W: f64 = 450.0;
const offset: f64 = 25.0;
const circle_d: f64 = 30.0;

fn main() {
    let mut window = window::default(W + 3.0 * offset, H);
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(OpenGL::V3_2);
    // let mut lines = ui::lines().to_owned();
    let mut glyphs =
        glyph_cache::GlyphCache::new(Search::Parents(3).for_folder("assets").unwrap().join(
            "RobotoMono-Regular.ttf",
        )).unwrap();


    let mut state = State::new();
    println!("{}", rgb_to_hex(state.color).as_str());
    let mut ui_color = col_str("white", 0.3);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                ui_color = state.l_or_d();
                //
                clear(state.color(), g);
                //
                //
                //

                for x in ui::lines() {
                    line::Line::new(ui_color, 1.5).draw(
                        x,
                        &DrawState::default(),
                        c.transform.trans(offset + circle_d / 2.0, 0.0),
                        g,
                    );

                    // line(ui_color, 6.0, x, c.transform.trans(offset, 0.0), g);
                }

                for x in state.circles() {
                    rectangle(ui_color, x, c.transform.trans(offset, 0.0), g);
                }
                //
                //ell
                //
                // text(
                //     ui_color,
                //     15,
                //     format!(
                //         "x:{} y:{}",
                //         state.mouse.point.x,
                //         state.mouse.point.y,
                //     ).as_str(),
                //     &mut glyphs,
                //     c.transform.trans(15.0, H - 30.0),
                //     g,
                // );
                text(
                    ui_color,
                    100,
                    rgb_to_hex(state.color).as_str(),
                    &mut glyphs,
                    c.transform.trans(15.0, H - 45.0),
                    g,
                );
            });
        }
        state.handler(e);
    }
}