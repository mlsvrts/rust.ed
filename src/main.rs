// ======================================================================================================================== //
// -------------------------------------------------------- RUST.ED ------------------------------------------------------- //
// ======================================================================================================================== //
//
//                               Rust.Ed is a lightweight, portable, rust-based text editor.
// ________________________________________________________________________________________________________________________ //

// Interface and control terminal
extern crate termion;

use std::io::{Write, stdout};

use termion::{color, cursor};
use termion::screen::AlternateScreen;


// Convient function for sleeping millis
fn sleepms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

// Run an alternate screen for printing
fn alt() {
    // Get some information about the size of the current terminal
    let size = termion::terminal_size().unwrap();
    let mids = (size.0 / 2, size.1 / 2);
    
    // Draw on this screen instead of the main terminal
    let mut screen = AlternateScreen::from(stdout());

    // Clear screen and set cursor position
    write!(screen, "{}{}", termion::clear::All, color::Fg(color::Blue)).unwrap();

    // Get the start position for line horizontals
    let xpad = mids.0 - 25;
    let ypad = mids.1 - 3;

    // Write the title to the screen 
    write!(screen, "{}██████╗ ██╗   ██╗███████╗████████╗███████╗██████╗\n", cursor::Goto(xpad, ypad)).unwrap();
    write!(screen, "{}██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗\n", cursor::Goto(xpad, ypad + 1)).unwrap();
    write!(screen, "{}██████╔╝██║   ██║███████╗   ██║   █████╗  ██║  ██║\n", cursor::Goto(xpad, ypad + 2)).unwrap();
    write!(screen, "{}██╔══██╗██║   ██║╚════██║   ██║   ██╔══╝  ██║  ██║\n", cursor::Goto(xpad, ypad + 3)).unwrap();
    write!(screen, "{}██║  ██║╚██████╔╝███████║   ██║██╗███████╗██████╔╝\n", cursor::Goto(xpad, ypad + 4)).unwrap();
    write!(screen, "{}╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝╚═╝╚══════╝╚═════╝\n", cursor::Goto(xpad, ypad + 5)).unwrap();

    // Flush the output (display)
    screen.flush().unwrap();
}

fn main() {
    // Print the new screen
    alt(); 
    
    // Give it some time to display
    sleepms(6000);
}


