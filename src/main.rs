// ======================================================================================================================== //
// -------------------------------------------------------- RUST.ED ------------------------------------------------------- //
// ======================================================================================================================== //
//
//                               Rust.Ed is a lightweight, portable, rust-based text editor.
// ________________________________________________________________________________________________________________________ //

mod display;
use display::Display;

mod input;

fn main() {

    // Initialize a new window
    let mut main = Display::new();

    // Clear it and display the title
    main.clear();
    main.title();
    main.reset_cursor();

    // Start capturing inputs
    input::capture(&mut main);
}
