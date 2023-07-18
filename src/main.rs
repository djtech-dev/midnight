pub mod defs;
pub mod term;

pub use crate::term::*;
pub use crate::defs::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}