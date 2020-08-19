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

// Print the title page an wait for user to type
fn title() {
    // Get some information about the size of the current terminal
    let size = termion::terminal_size().unwrap();
    let mids = (size.0 / 2, size.1 / 2);
    let bl = color::Fg(color::Blue);
    let rd = color::Fg(color::Red);
    let bk = color::Fg(color::Black);
    
    // Draw on this screen instead of the main terminal
    let mut screen = AlternateScreen::from(stdout());

    // Clear screen and set cursor position
    write!(screen, "{}{}", termion::clear::All, color::Fg(color::Blue)).unwrap();

    // Get the start position for line horizontals
    let xp = mids.0 - 25;
    let yp = mids.1 - 3;

    // Write the title to the screen 
    write!(screen, "{}{}██████╗ ██╗   ██╗███████╗████████╗{}███████╗██████╗\n", 
           cursor::Goto(xp, yp), rd, bl).unwrap();
    write!(screen, "{}{}██╔══██╗██║   ██║██╔════╝╚══██╔══╝{}██╔════╝██╔══██╗\n", 
           cursor::Goto(xp, yp+1), rd, bl).unwrap();
    write!(screen, "{}{}██████╔╝██║   ██║███████╗   ██║   {}█████╗  ██║  ██║\n", 
           cursor::Goto(xp, yp+2), rd, bl).unwrap();
    write!(screen, "{}{}██╔══██╗██║   ██║╚════██║   ██║   {}██╔══╝  ██║  ██║\n", 
           cursor::Goto(xp, yp+3), rd, bl).unwrap();
    write!(screen, "{}{}██║  ██║╚██████╔╝███████║   ██║{}██╗{}███████╗██████╔╝\n", 
           cursor::Goto(xp, yp+4), rd, bk, bl).unwrap();
    write!(screen, "{}{}╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝{}╚═╝{}╚══════╝╚═════╝\n", 
           cursor::Goto(xp, yp+5), rd, bk, bl).unwrap();

    // Set the text to default color, and the cursor to the start position
    write!(screen, "{}{}", cursor::Goto(1, 1), color::Fg(color::Reset));

    // Flush the output (display)
    screen.flush().unwrap();
}

fn main() {
    // Print the new screen
    title(); 
    
    // Give it some time to display
    sleepms(6000);
}


