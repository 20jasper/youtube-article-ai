use std::{env, fs};

pub mod error;
mod transcript;

use error::Result;
use transcript::{get_transcript, vtt_to_text};

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let url = args.next().ok_or("missing url argument")?;

    let transcript = get_transcript(&url)?;
    let text = vtt_to_text(&transcript);

    fs::write("./dist/done.txt", text)?;

    Ok(())
}
