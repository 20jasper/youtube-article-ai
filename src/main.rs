use std::env;

pub mod error;
mod transcript;

use error::Result;
use transcript::get_transcript;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let url = args.next().ok_or("missing url argument")?;

    dbg!(get_transcript(&url)?);

    Ok(())
}
