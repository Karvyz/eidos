use rig_core::{
    client::CompletionClient,
    completion::{Prompt, ToolDefinition},
    providers::llamafile,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::eidos::Eidos;

pub struct LLM {
    client: llamafile::Client,
}

impl LLM {
    pub fn new() -> Self {
        let client = llamafile::Client::from_url("http://localhost:8080").unwrap();

        Self { client }
    }

    pub async fn create_note(&self, prompt: &str, eidos: Eidos) -> String {
        let agent = self
            .client
            .agent(llamafile::LLAMA_CPP)
            .preamble("You are bob.")
            .tools(vec![Box::new(Create::new(eidos))])
            .build();
        agent.prompt(prompt).await.expect("error")
    }
}

#[derive(Deserialize)]
pub(crate) struct CreationArgs {
    title: String,
    content: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Text error")]
pub(crate) struct TextError;

#[derive(Serialize)]
pub(crate) struct Create {
    #[serde(skip)]
    eidos: Eidos,
}

impl Create {
    pub(crate) fn new(eidos: Eidos) -> Self {
        Self { eidos }
    }
}

impl Tool for Create {
    const NAME: &'static str = "create";
    type Error = TextError;
    type Args = CreationArgs;
    type Output = bool;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "create".to_string(),
            description: "Create a new note".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "the title of the note, no extension" },
                    "content": { "type": "string", "description": "the content of the note formated in markdown" }
                },
                "required": ["title", "content"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        self.eidos.create_note(args.title, args.content).await;
        Ok(true)
    }
}
