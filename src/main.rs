#![warn(clippy::all, clippy::pedantic)]
// the above directive activate warnings for two categories "all" and "pedantic" in clippy
// docs: https://doc.rust-lang.org/nightly/clippy/

use std::env;
mod editor;
use editor::Editor;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // default is a static method, that is how we use static
    // method to get a new Editor
    let editor = Editor::default();

    // same as calling
    // Editor::run(&editor);
    editor.run();
}
