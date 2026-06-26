use rig_core::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::eidos::Eidos;

#[derive(Deserialize)]
pub(crate) struct FetchArgs {}

#[derive(Debug, thiserror::Error)]
#[error("Text error")]
pub(crate) struct TextError;

#[derive(Serialize)]
pub(crate) struct Fetch {
    #[serde(skip)]
    eidos: Eidos,
}

impl Fetch {
    pub(crate) fn new(eidos: Eidos) -> Self {
        Self { eidos }
    }
}

impl Tool for Fetch {
    const NAME: &'static str = "fetch";
    type Error = TextError;
    type Args = FetchArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch".to_string(),
            description: "fetch notes".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }

    async fn call(&self, _: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Tool call: Fetching notes");
        let notes = self.eidos.notes().await;
        Ok(notes.iter().map(|n| n.title().to_string()).collect())
    }
}
