use rig_core::{client::CompletionClient, completion::Prompt, providers::llamafile};

use crate::{
    eidos::Eidos,
    llm::{create::Create, fetch::Fetch},
};

mod create;
mod fetch;

pub struct LLM {
    client: llamafile::Client,
    eidos: Eidos,
}

impl LLM {
    pub fn new(eidos: &Eidos) -> Self {
        let client = llamafile::Client::from_url("http://localhost:8080").unwrap();

        Self {
            client,
            eidos: eidos.clone(),
        }
    }

    pub async fn ask(&self, prompt: &str) -> String {
        let agent = self
            .client
            .agent(llamafile::LLAMA_CPP)
            .preamble("You are a note taker assistant. Use the provided tools.")
            .tools(vec![
                Box::new(Create::new(self.eidos.clone())),
                Box::new(Fetch::new(self.eidos.clone())),
            ])
            .default_max_turns(10)
            .build();
        println!("Thinking...");
        agent.prompt(prompt).await.expect("error")
    }
}
