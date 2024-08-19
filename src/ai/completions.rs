use crate::ai::deepinfra::{Message, Response};
use crate::Result;
use derive_builder::Builder;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

/// more documentation can be found here https://deepinfra.com/meta-llama/Meta-Llama-3.1-70B-Instruct/api?version=25acb1b514688b222a02a89c6976a8d7ad0e017f#input-model
#[derive(Clone, Serialize, Deserialize, Default, Debug, Builder, PartialEq)]
#[builder(pattern = "mutable")]
#[builder(derive(Debug))]
#[builder(setter(into, strip_option), default)]
pub struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_penalty: Option<f32>,
}

pub const COMPLETIONS_PATH: &str = "chat/completions";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionClient {
    model: String,
    url: Url,
    token: String,
}

impl CompletionClient {
    pub fn build(
        token: impl Into<String>,
        base_url: &str,
        model: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            model: model.into(),
            url: dbg!(base_url.parse::<Url>()?.join(COMPLETIONS_PATH)?),
            token: token.into(),
        })
    }

    pub async fn post(&self, prompt: &str, text: &str) -> Result<String> {
        let payload = CompletionRequestBuilder::default()
            .model(&self.model)
            .max_tokens(1000_u32)
            .messages([
                Message {
                    role: "system".into(),
                    content: prompt.into(),
                },
                Message {
                    role: "user".into(),
                    content: text.into(),
                },
            ])
            .build()?;
        dbg!(&payload);

        let response = dbg!(Client::new()
            .post(self.url.as_ref())
            .bearer_auth(&self.token)
            .json(&payload))
        .send()
        .await?;

        dbg!(&response);

        let json = response.json::<Response>().await?;
        let content = json.choices[0].message.content.clone();
        Ok(content)
    }
}
