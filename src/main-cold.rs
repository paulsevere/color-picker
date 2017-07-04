#![feature(box_syntax, box_patterns, rand)]
#![allow(warnings)]
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate shapes;
extern crate palette;
extern crate rand;
extern crate find_folder;
extern crate piston_window;

// use piston_window::*;
// use find_folder::*;

// use glutin_window::
use std::rc::Rc;
// use piston_window;
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
use color::*;
mod mouse;
use mouse::Mouse;
mod element;
use element::{rect_arr, find_and_remove, lines_and_markers};


fn main() {
    let (width, height) = (500.0, 400.0);
    let mut state = ColorState::new(width);
    let mut window: Window = window::default(width, height);

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(OpenGL::V3_2);
    let mut glyphs =
        glyph_cache::GlyphCache::new(Search::Parents(3).for_folder("assets").unwrap().join(
            "RobotoMono-Regular.ttf",
        )).unwrap();
    // println!("{:?}", glyphs);
    let grey = col_str("gray", 1.0);
    let sea_green = col_str("lightseagreen", 1.0);
    let blue = col_str("lightsteelblue", 0.2);
    // let mut loc = box Point { x: 0.0, y: 0.0 };

    let mut els = lines_and_markers(width, height, 30.0);
    let mut lines = els.0;
    let mut elements = els.1.to_owned();

    let mut mouse = Mouse::new(width, height);


    while let Some(e) = events.next(&mut window) {


        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                clear(state.color, g);
                text(
                    sea_green,
                    30,
                    state.string.as_str(),
                    &mut glyphs,
                    c.transform.trans(100.0, height * 7.0 / 8.0),
                    g,
                );
            });


            for mut x in lines.to_owned() {
                gl.draw(r.viewport(), |c, g| { line(blue, 1.5, x, c.transform, g); });
            }
            gl.draw(
                r.viewport(),
                |c, g| if let Some(rec) = mouse.target_rect() {
                    ellipse(sea_green, rect_arr(rec.0), c.transform, g);
                },
            );


            for mut x in elements.to_owned() {
                gl.draw(r.viewport(), |c, g| {
                    ellipse(sea_green, rect_arr(x), c.transform, g);
                });
            }


        }
        if let Some(m) = e.mouse_cursor_args() {
            mouse.update_loc(m);
            if let Some(rec) = mouse.target_rect() {
                let (rect, index) = rec;
                state.update(index, rect.pos.x);
            }

        }
        //
        // Click Down
        //
        //
        if let Some(p) = e.press_args() {
            let (rest, targ) = find_and_remove(elements, mouse.point());
            elements = rest;
            mouse.update_click(true, targ);
        }
        //
        // release click
        //
        //
        if let Some(r) = e.release_args() {
            if let Some(rec) = mouse.target_rect() {
                elements.insert(rec.1, rec.0);
            }
            mouse.update_click(false, None);
        }
    }
}