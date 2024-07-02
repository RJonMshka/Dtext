use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

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
        // top level error handler
        if let Err(err) = self.repl() {
            // panic! is a macro which basically crashes our program cleanly
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    // This function will return either a pink Ok box with nothing in it, or a black Err box with a std::io::Error in it
    fn repl(&mut self) -> Result<(), std::io::Error> {

        // It unwraps the Result of enable_raw_mode for us. 
        // If it's an error, it returns the error immediately. If not, it continues.
        enable_raw_mode()?;

        loop {
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");

                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                        println!("yes this is control");
                        // The print! macro in Rust lets us send data directly to the terminal.
                        // \x1b, the escape character, or 27 in decimal.
                        // \x signifies that what follows should be read as a hexadecimal number.
                        // 1b translates to 27 in decimal 
                        // The remaining part, [2J, forms part of an escape sequence

                        // For a deeper dive into the magic of terminal commands, explore the VT100 escape: 
                        // https://en.wikipedia.org/wiki/VT100

                        // https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences

                        // print!("\x1b[2J");
                    },
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}


// for b in io::stdin().bytes() {
        //     match b {
        //         Ok(b) => {
        //             let c = b as char;
        //             if c.is_control() {
        //                 println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        //             } else {
        //                 println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        //             }
        //             if c == 'q' {
        //                 break;
        //             }
        //         }
        //         Err(err) => println!("Error: {}", err)
        //     }
        // }
