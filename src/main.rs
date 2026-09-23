use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c == 'q' {
                    disable_raw_mode().unwrap();
                    break;
                }
            },
            Err(e) => println!("{}", e)
        }
    }
}