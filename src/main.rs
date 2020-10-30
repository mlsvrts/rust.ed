// ======================================================================================================================== //
// -------------------------------------------------------- RUST.ED ------------------------------------------------------- //
// ======================================================================================================================== //
//
//                               Rust.Ed is a lightweight, portable, rust-based text editor.
// ________________________________________________________________________________________________________________________ //

mod window;
use window::display;

mod input;

fn main() {

    // Initialize a new window
    let mut main = display::Screen::new();

    // Clear it and display the title
    main.clear();
    display::title(&mut main);
    display::border(&mut main);

    // Start capturing inputs
    input::capture(&mut main);
}
