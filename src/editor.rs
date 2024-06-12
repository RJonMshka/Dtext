use std::io::{self, Read};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

// defining the construct: empty rn
pub struct Editor {

}

// implementation for the construct
impl Editor {
    
    // Self means its own type: Editor
    pub fn default() -> Self {
        Editor{}
    }

    // &self is referencing the struct from conext, Editor in this case
    pub fn run(&self) {
        enable_raw_mode().unwrap();
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

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);

                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        },
                        _ => (),
                    }
                },
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}