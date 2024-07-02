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