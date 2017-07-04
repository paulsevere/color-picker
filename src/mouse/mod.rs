use shapes::*;
use piston::input::*;


pub struct Mouse {
    pub down: bool,
    pub point: Point,
    pub target: Option<(usize, f64, f64)>,
}


impl Mouse {
    pub fn new() -> Self {
        Mouse {
            down: false,
            point: Point { x: 0.0, y: 0.0 },
            target: None,
        }
    }


    pub fn toggle(&mut self) {
        self.down = !self.down;
    }
    pub fn set(&mut self, point: [f64; 2]) {
        self.point = Point {
            x: point[0] - ::offset,
            y: point[1],
        };
    }

    pub fn on_target(&mut self, rects: Vec<Rect>) -> Vec<Rect> {
        self.target = rects.iter().enumerate().fold(
            None,
            |acc, (i, x)| if x.contains(
                self.point,
            )
            {
                return Some((i, x.left(), x.left() - self.point.x));
            } else {
                return acc;
            },
        );
        rects
    }
}