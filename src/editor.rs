use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char('q') = event.code {
                        break;
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => ()
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}