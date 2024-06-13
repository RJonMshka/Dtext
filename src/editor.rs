use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

// defining the construct: empty rn
pub struct Editor {}

// implementation for the construct
impl Editor {
    
    // Self means its own type: Editor
    pub fn default() -> Self {
        Editor{}
    }

    // &self is referencing the struct from context, Editor in this case
    pub fn run(&self) {
        // top level error handler
        if let Err(err) = self.repl() {
            // panic! is a macro which basically crashes our program cleanly
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    // This function will return either a pink Ok box with nothing in it, or a black Err box with a std::io::Error in it
    fn repl(&self) -> Result<(), std::io::Error> {

        // It unwraps the Result of enable_raw_mode for us. 
        // If it's an error, it returns the error immediately. If not, it continues.
        enable_raw_mode()?;

        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");

                if let Char(c) = event.code {
                    if c == 'q' {
                         break;
                     }
                }
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
