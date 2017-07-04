use mouse::*;
use piston::event_loop::*;
use color::*;
use shapes::*;
use piston::input::*;
use copy::*;


pub type RGB = [f64; 3];

pub struct State {
    pub color: RGB,
    pub mouse: Mouse,
}

impl State {
    pub fn new() -> Self {
        State {
            color: [0.15, 0.15, 0.2],
            mouse: Mouse::new(),
        }
    }


    pub fn color(&self) -> Col {
        let col = self.color.clone();
        [col[0] as f32, col[1] as f32, col[2] as f32, 1.0]
    }

    pub fn l_or_d(&self) -> Col {
        let sum: f64 = self.color.iter().sum();
        match (sum / 3.0) > 0.5 {
            false => col_str("white", 0.5),
            true => col_str("black", 0.8),
        }
    }

    pub fn circles(&self) -> Vec<Rect> {
        self.color
            .iter()
            .enumerate()
            .map(|(i, x)| to_rect(i, x))
            .collect()
    }


    pub fn handler(&mut self, e: Input) {
        use Input::*;
        if let Some(button) = e.press_args() {
            //
            //
            //
            match button {
                Button::Keyboard(key) => {
                    match key {
                        Key::C => to_clipboard(rgb_to_hex(self.color)),
                        _ => {}
                    }
                }
                Button::Mouse(b) => {

                    // toggle mouse down
                    self.mouse.toggle();
                    // get circle representation of color
                    let circles = self.circles();
                    // test if click targets one of them
                    self.mouse.on_target(circles);
                }

                _ => {}
            }



        //
        //
        //
        //
        //
        } else if let Some(e) = e.release_args() {
            self.mouse.toggle();
            self.mouse.target = None;
        } else if let Some(args) = e.mouse_cursor_args() {
            self.mouse.set(args);
            self.handle_drag();
        }


        // match e {
        //     Press(t) | Release(t) => self.toggle(),
        //     Motion => println!("{:?}", e),
        //     _ => {}
        // }
    }



    pub fn handle_drag(&mut self) {
        if let Some((i, p, o)) = self.mouse.target {
            self.color[i] = match (self.mouse.point.x + o) / ::W {
                x if x < 0.0 => 0.0,
                x if x > 1.0 => 1.0,
                x => x,
            }
        }
    }


    pub fn display_target(&self) -> String {
        let cols = ["Red", "Green", "Blue"];
        match self.mouse.target {
            Some(t) => {
                let tar = t.to_owned();
                format!("{} - offset:  {}", cols[tar.0], tar.2)
            }
            None => String::from(" "),
        }
    }

    pub fn display_color(&self) -> String {
        format!(
            "Rgb({:.2}, {:.2}, {:.2})",
            self.color[0],
            self.color[1],
            self.color[2]
        )
    }
}

pub fn to_rect(i: usize, x: &f64) -> Rect {
    let hs = [::h1, ::h2, ::h3];
    Rect::from((::W * x, hs[i], ::circle_d, ::circle_d))

}