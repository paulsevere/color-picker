use palette::{Rgba, Rgb};
use palette::pixel::Srgb;
use palette::named;

pub type Col = [f32; 4];

pub fn col_str(name: &str, alpha: f32) -> [f32; 4] {
    let olive = named::from_str(name).expect("unknown color");
    let rgba: Rgba = Srgb::from_pixel(&olive).into();
    // dump!(rgba);
    // let alpha = from_str.alpha;
    rgb_to_arr(rgba, alpha)
}


pub fn rgb_to_arr(rgba: Rgba, alpha: f32) -> Col {

    let rgb = rgba.color;
    // let alpha = rgba.alpha;
    let Rgb { red, green, blue } = rgb;
    [red, green, blue, alpha]

    // [0.5, 0.5, 0.5, 0.5]

}

pub fn rgb_to_hex(vals: [f64; 3]) -> String {
    // let ints = vals.iter()
    //     .map(|e| (e * 255.0).round() as u32)
    //     .collect::<u32>();

    format!(
        "#{:x}{:x}{:x}",
        (vals[0] * 255.0).round() as u32,
        (vals[1] * 255.0).round() as u32,
        (vals[2] * 255.0).round() as u32
    )
}


pub struct ColorState {
    pub color: Col,
    pub div: f64,
    pub string: String,
}

impl ColorState {
    pub fn new(w: f64) -> Self {
        let fc = col_str("black", 1.0);
        let mut me = ColorState {
            color: col_str("black", 1.0),
            div: w,
            string: String::new(),
        };
        me.calc_string();
        me
    }

    pub fn calc_string(&mut self) {
        let fc = self.color;
        self.string = String::from(format!(
            "rgb({},{},{})",
            (fc[0] * 255.0) as i64,
            (fc[1] * 255.0) as i64,
            (fc[2] * 255.0) as i64
        ));
    }

    pub fn update(&mut self, index: usize, val: f64) {
        self.color[index] = (val / self.div) as f32;
        self.calc_string();
    }
}