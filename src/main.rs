use std::{env, fs};

mod ai;
pub mod error;
mod transcript;

use ai::completions::CompletionClient;
use dotenvy::dotenv;
use error::Result;
use transcript::{get_transcript, vtt_to_text};

const PROMPT: &str = r###"Act as the author and provide a comprehensive detailed article in {language}
        in markdown format that has a H1 main title(example "# <{training_shot}> ") and broken down into H2 subtitles (example "## <{training_shot}> ") for the following transcript

You must follow the rules:
	{max_words}

- Write the article in markdown format
- Create a main title for the article as markdown H1 and break the article into subtitles where each subtitle is markdown H2
- Article must be in {language}
- summary should be informative and act as a replacement for the original transcript to the point that the user doesn't have to go back to read the transcript
- Summary should not mention the author or speaker at all should act as your independent writing without referencing the original transcript or speaker.
- You can use bullet points within the article"###;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mut args = env::args();
    args.next();
    let url = args.next().ok_or("missing url argument")?;

    let api_key = env::var("OPEN_AI_API_KEY").unwrap();
    let model = env::var("OPEN_AI_MODEL").unwrap();
    let base_url = env::var("OPEN_AI_BASE_URL").unwrap();

    let transcript = get_transcript(&url)?;
    let text = vtt_to_text(&transcript);

    let client = CompletionClient::build(api_key, &base_url, model)?;
    let res = client.post(PROMPT, &text).await?;

    fs::write(transcript::get_write_path(&url)?, res)?;

    Ok(())
}
