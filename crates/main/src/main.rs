#![warn(clippy::all)]

use std::error::Error;

pub(crate) fn main() -> Result<(), Box<dyn Error>> {
    println!("hello world");
    Ok(())
}
