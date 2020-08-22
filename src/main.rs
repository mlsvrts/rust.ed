// ======================================================================================================================== //
// -------------------------------------------------------- RUST.ED ------------------------------------------------------- //
// ======================================================================================================================== //
//
//                               Rust.Ed is a lightweight, portable, rust-based text editor.
// ________________________________________________________________________________________________________________________ //

mod display;
use display::Display;

// Convient function for sleeping millis
fn sleepms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}


fn main() {

    let mut main = Display::new();

    main.clear();
    main.title();

    // Give it some time to display
    sleepms(6000);
}
