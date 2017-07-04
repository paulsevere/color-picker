use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;

pub fn default(w: f64, h: f64) -> Window {
    let mut window = WindowSettings::new("color picker", [w as u32, h as u32])
        .exit_on_esc(true)
        .decorated(false)
        .srgb(false)
        .build()
        .unwrap();


    window


}


// pub fn font_cache(window: &mut Window) {
//     let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
//     let ref font = assets.join("RobotoMono-Regular.ttf");
//     let factory = window.factory.clone();
//     let mut glyphs = Glyphs::new(font, factory).unwrap();

// }