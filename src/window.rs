// ======================================================================================================================== //
// ----------------------------------------------------- DISPLAY ---------------------------------------------------------- //
// ======================================================================================================================== //

/// Implements display controls
pub mod display {
    extern crate termion;

    use termion::screen::AlternateScreen;
    use termion::raw::{IntoRawMode, RawTerminal};
    use termion::{color, cursor};
    use termion::cursor::{Save, Restore};

    use std::io::{Write, Stdout, stdout};
    use chrono::Local;

    /// Screen structure for output pipe
    pub struct Screen {
        terminal: RawTerminal<AlternateScreen<Stdout>>,
        pub alive: bool,
        pub border: bool,
    }

    impl Screen {
        /// Create a new screen object
        pub fn new() -> Screen {
            return Screen {
                terminal: AlternateScreen::from(stdout()).into_raw_mode().unwrap(),
                alive: true,
                border: false
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

            &self.flush();
        }

        /// Report screen size
        pub fn size(&mut self) -> (u16, u16) {
            return termion::terminal_size().unwrap();
        }

        /// Go to starting/home position
        pub fn home(&mut self) {
            write!((*self).terminal, "{}", cursor::Goto(2,2)).unwrap();

            &self.flush();
        }
    }

    /// Print the "Rust.Ed" logo onto a given screen
    pub fn logo(screen: &mut Screen) {
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
    }

    /// Update the displayed time
    pub fn time(screen: &mut Screen) {
        // Don't print time if border is not displayed
        if !screen.border {
            return;
        }

        let size = screen.size();
        let blue = color::Fg(color::Blue);
        let green = color::Fg(color::Green);

        // Get current time as string
        let now = Local::now().format("%H:%M:%S");

        // Move over 9 char for every 100 chars of screen size
        let shift = (9 * (size.0 / 100)) as u16;

        // Print the time
        let str_time = format!("{}{}{}│ {}{}{} │{}{}",
            Save, cursor::Goto(size.0 - shift - 12, size.1),
            green, blue, now, green,
            color::Fg(color::Reset), Restore
        );

        screen.write(&str_time);
        screen.flush();
    }

    /// Update the displayed title
    pub fn title(screen: &mut Screen, name: &str) {
        // Don't print title if border is not displayed
        if !screen.border {
            return;
        }

        let size = screen.size();
        let blue = color::Fg(color::Blue);
        let green = color::Fg(color::Green);

        // Move over 9 char for every 100 chars of screen size
        let shift = (9 * (size.0 / 100)) as u16;

        // Print the time
        let str_title = format!("{}{}{}│ {}{}{} │{}{}",
            Save, cursor::Goto(shift, 1),
            green, blue, name, green,
            color::Fg(color::Reset), Restore
        );

        screen.write(&str_title);
        screen.flush();
    }

    /// Draw a border around the screen
    pub fn border(screen: &mut Screen) {
        // Basic details
        let green = color::Fg(color::Green);
        let size = screen.size();

        // Check for minimum size
        if size.0 < 25 || size.1 < 25 {
            screen.border = false;
            return;
        }

        // Enable other border functions
        screen.border = true;

        // Set color
        write!(screen.terminal, "{}", green).unwrap();

        // Top
        write!(screen.terminal, "{}┌{}┐",
            cursor::Goto(1, 1),
            "─".repeat((size.0 - 2) as usize)
        ).unwrap();

        // Left, Right
        for i in 2..size.1 {
            write!(screen.terminal, "{}│", cursor::Goto(1, i)).unwrap();
            write!(screen.terminal, "{}│", cursor::Goto(size.0, i)).unwrap();
        }

        // Bottom
        write!(screen.terminal, "{}└{}┘{}",
            cursor::Goto(1, size.1),
            "─".repeat((size.0 - 2) as usize),
            color::Fg(color::Reset)
        ).unwrap();
    }
}
