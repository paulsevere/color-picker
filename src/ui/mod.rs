use shapes::*;


pub fn lines() -> Vec<[f64; 4]> {
    [::h1, ::h2, ::h3]
        .iter()
        .map(|e| [0.0, off(*e), ::W, off(*e)])
        .collect()
}

pub fn lines_r() -> Vec<[f64; 4]> {
    [::h1, ::h2, ::h3]
        .iter()
        .map(|e| [0.0, off(*e), ::W, off(*e)])
        .collect()
}

fn off(h: f64) -> f64 {
    h + (::circle_d / 2.0)
}