// ======================================================================================================================== //
// -------------------------------------------------------- RUST.ED ------------------------------------------------------- //
// ======================================================================================================================== //
//
//                               Rust.Ed is a lightweight, portable, rust-based text editor.
// ________________________________________________________________________________________________________________________ //

use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;

mod window;
use window::display;

mod input;
use termion::input::TermRead;
use std::io::stdin;

fn main() {
    // Initialize a new window
    let arc = Arc::new(Mutex::new(display::Screen::new()));

    // Claim ownership and display the title
    let setup = arc.clone();
    {
        let mut main = setup.lock().unwrap();

        main.clear();
        display::title(&mut main);
    }

    // Spawn a thread to keep the border updated
    let border;
    {
        let border_task = arc.clone();
        border = thread::spawn(move || {
            loop {
                let mut main = border_task.lock().unwrap();

                // Stop updating the border if the main thread has been killed
                if main.alive{
                    display::border(&mut main);
                    drop(main);
                    thread::sleep(Duration::from_millis(1000));
                } else {
                    break;
                }
            }
        });
    }

    let session;
    {
        // Spawn a thread to capture inputs
        let input_task = arc.clone();
        let stdin = stdin();
        session = thread::spawn(move || {
            // Loop through all input events, stop when asked
            for op in stdin.events() {
                let mut main = input_task.lock().unwrap();
                let event = op.unwrap();

                // Handle events
                match input::capture(&mut main, event) {
                    1 => {
                        main.alive = false;
                        break
                    },
                    _ => {}
                };

                drop(main);
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    session.join().unwrap();
    border.join().unwrap();
}
