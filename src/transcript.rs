use crate::Result;
use core::str;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

const YTDLP: &str = "yt-dlp";
const RETRIES: &str = "10";
/// https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#output-template-examples
const OUTPUT_TEMPLATE: &str = "%(id)s";
const OUTPUT_PATH: &str = "./transcripts";

pub fn get_transcript(url: &str) -> Result<String> {
    let mut binding = Command::new(YTDLP);
    let cmd = binding.args([
        "--print",
        "filename",
        "--write-subs",
        "--write-auto-subs",
        "--sub-langs",
        "en*",
        "--sub-format",
        "vtt",
        "--skip-download",
        "--retries",
        RETRIES,
        "--output",
        OUTPUT_TEMPLATE,
        "--paths",
        OUTPUT_PATH,
        "-i",
        url,
    ]);

    let Output {
        status,
        stdout,
        stderr,
    } = cmd.output()?;

    if !status.success() {
        panic!("uh oh, failed with status {status:?}\n {stderr:?}");
    }

    let stdout = str::from_utf8(&stdout)?;
    let mut path = PathBuf::from(stdout.trim_end());
    path.set_extension("en.vtt");
    dbg!(&path);
    Ok(fs::read_to_string(path)?)
}
