# Dtext
Rust based Text Editor

## About the escape sequence
The print! macro in Rust lets us send data directly to the terminal.
\x1b, the escape character, or 27 in decimal.
\x signifies that what follows should be read as a hexadecimal number.
1b translates to 27 in decimal 
The remaining part, [2J, forms part of an escape sequence

For a deeper dive into the magic of terminal commands, explore the VT100 escape: 
https://en.wikipedia.org/wiki/VT100

https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences

print!("\x1b[2J");


## About &self
&self is referencing the struct from context, Editor in this case
&mut as now run changes the Editor its called upon.

## panic
panic! is a macro which basically crashes our program cleanly

## About Result<(), std::io::Error>
This function will return either a pink Ok box with nothing in it, or a black Err box with a std::io::Error in it
It unwraps the Result of enable_raw_mode for us. 
If it's an error, it returns the error immediately. If not, it continues.

## Crossterm

### Execute
execute! means that we want to write out immediately and not wait for the buffer to be filled until it's written out.



## Old Code References

### code for handling key presses

```
for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                    } else {
                        println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                    }
                    if c == 'q' {
                        break;
                    }
                }
                Err(err) => println!("Error: {}", err)
            }
        }
```

### old code for run
```
pub fn run(&mut self) {
        // top level error handler
        if let Err(err) = self.repl() {
            // panic! is a macro which basically crashes our program cleanly
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }
```

### old code for repl
```
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
```