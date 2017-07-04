use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub fn to_clipboard(hex: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(hex.to_owned());
}
