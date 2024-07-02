use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use std::io::stdout;

// defining the construct: empty rn
pub struct Editor {
    should_quit: bool,
}

// implementation for the construct
impl Editor {
    
    // Self means its own type: Editor
    pub fn default() -> Self {
        Editor{ should_quit: false }
    }

    // &self is referencing the struct from context, Editor in this case
    // &mut as now run changes the Editor its called upon.
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        // execute! means that we want to write out immediately and not wait for the buffer to be filled until it's written out.
        execute!(stdout, Clear(ClearType::All))
    }

    // This function will return either a pink Ok box with nothing in it, or a black Err box with a std::io::Error in it
    fn repl(&mut self) -> Result<(), std::io::Error> {

        // It unwraps the Result of enable_raw_mode for us. 
        // If it's an error, it returns the error immediately. If not, it continues.
        enable_raw_mode()?;

        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                // dereferencing with * as event is an address
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
