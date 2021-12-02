#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod y2015;
pub mod y2021;

use adventofcode::run;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    run!(2021, 2);

    Ok(())
}
