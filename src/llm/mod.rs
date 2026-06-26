use rig_core::{client::CompletionClient, completion::Prompt, providers::llamafile};

use crate::{eidos::Eidos, llm::create::Create};

mod create;

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
            .preamble("You are a note taker assistant. Use the provided tools.")
            .tools(vec![Box::new(Create::new(eidos))])
            .build();
        println!("Thinking...");
        agent.prompt(prompt).await.expect("error")
    }
}
