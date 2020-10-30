// ======================================================================================================================== //
// ----------------------------------------------------- DISPLAY ---------------------------------------------------------- //
// ======================================================================================================================== //

/// Implements display controls
pub mod display {
    extern crate termion;

    use termion::screen::AlternateScreen;
    use termion::raw::{IntoRawMode, RawTerminal};
    use termion::{color, cursor};

    use std::io::{Write, Stdout, stdout};
    use chrono::Local;

    /// Screen structure for output pipe
    pub struct Screen {
        terminal: RawTerminal<AlternateScreen<Stdout>>,
        title: String,
    }

    impl Screen {
        /// Create a new screen object
        pub fn new() -> Screen {
            return Screen {
                terminal: AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
                title: String::from("Rust.Ed")
            };
        }

        /// Flush output to screen
        pub fn flush(&mut self) {
            &self.terminal.flush().unwrap();
        }

        /// Completely wipe screen
        pub fn clear(&mut self) {
            write!((*self).terminal, "{}", termion::clear::All).unwrap();
            &self.flush();
        }

        /// Write a string to screen
        pub fn write(&mut self, string: &str) {
            write!((*self).terminal, "{}", string).unwrap();
        }

        /// Report screen size
        pub fn size(&mut self) -> (u16, u16) {
            return termion::terminal_size().unwrap();
        }

        /// Return colors to default
        pub fn reset(&mut self) {
            write!((*self).terminal, "{}{}",
                   color::Fg(color::Reset), color::Bg(color::Reset)).unwrap();
            &self.flush();
        }
    }

    /// Print the "Rust.Ed" title onto a given screen
    pub fn title(screen: &mut Screen) {
        // Generates the 'Rust.Ed' logo, and prints it in the middle of the window

        // Get information about the current size of the screen
        let size = screen.size();
        let mids = (size.0 / 2, size.1 / 2);

        // Set colors for display
        let bl = color::Fg(color::Blue);
        let rd = color::Fg(color::Red);
        let bk = color::Fg(color::Black);

        // Get the start position for line horizontals
        let xp = mids.0 - 26;
        let yp = mids.1 - 2;

        // Write the title to the screen
        let title = format!(
            "{}{}██████╗ ██╗   ██╗███████╗████████╗{}███████╗██████╗\n\
             {}{}██╔══██╗██║   ██║██╔════╝╚══██╔══╝{}██╔════╝██╔══██╗\n\
             {}{}██████╔╝██║   ██║███████╗   ██║   {}█████╗  ██║  ██║\n\
             {}{}██╔══██╗██║   ██║╚════██║   ██║   {}██╔══╝  ██║  ██║\n\
             {}{}██║  ██║╚██████╔╝███████║   ██║{}██╗{}███████╗██████╔╝\n\
             {}{}╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝{}╚═╝{}╚══════╝╚═════╝\n",
            cursor::Goto(xp, yp), rd, bl,
            cursor::Goto(xp, yp+1), rd, bl,
            cursor::Goto(xp, yp+2), rd, bl,
            cursor::Goto(xp, yp+3), rd, bl,
            cursor::Goto(xp, yp+4), rd, bk, bl,
            cursor::Goto(xp, yp+5), rd, bk, bl);

        screen.write(&title);

        // Flush the output (display)
        screen.reset();
    }

    /// Draw a border around the screen
    pub fn border(screen: &mut Screen) {
        // Basic details
        let green = color::Fg(color::Green);
        let blue = color::Fg(color::Blue);
        let size = screen.size();

        // Move over 9 char for every 100 chars of screen size
        let shift = (9 * (size.0 / 100)) as u16;

        // Check for minimum size
        if size.0 < 25 || size.1 < 25 {
            println!("{} {}", size.0, size.1);
            return;
        }

        // Set color
        write!(screen.terminal, "{}", green).unwrap();

        let title_len = screen.title.len() as u16;
        // Top
        write!(screen.terminal, "{}┌{}│ {}{}{} │{}┐",
            cursor::Goto(1, 1),
            "─".repeat(shift as usize),
            blue, screen.title, green,
            "─".repeat((size.0 - shift - title_len - 6) as usize)
        ).unwrap();

        // Left, Right
        for i in 2..size.1 {
            write!(screen.terminal, "{}│", cursor::Goto(1, i)).unwrap();
            write!(screen.terminal, "{}│", cursor::Goto(size.0, i)).unwrap();
        }

        // Bottom
        let time = Local::now().format("%H:%M:%S");
        let len_t = 8;

        write!(screen.terminal, "{}└{}│ {}{}{} │{}┘",
            cursor::Goto(1, size.1),
            "─".repeat((size.0 - shift - len_t) as usize),
            blue, time, green,
            "─".repeat((shift - 6) as usize)
        ).unwrap();

        // Reset
        screen.reset();

    }
}
