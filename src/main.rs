pub mod defs;
pub mod term;
pub mod config;

pub mod pandocs;

//pub mod commands;
//pub mod helium;
//pub mod terminal_emulator;

pub use crate::defs::*;
pub use crate::term::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
