use rig_core::{client::CompletionClient, completion::Prompt, providers::llamafile};

pub struct LLM {
    client: llamafile::Client,
}

impl LLM {
    pub fn new() -> Self {
        let client = llamafile::Client::from_url("http://localhost:8080").unwrap();

        Self { client }
    }

    pub async fn response(&self, prompt: &str) -> String {
        let agent = self
            .client
            .agent(llamafile::LLAMA_CPP)
            .preamble("You are bob.")
            .build();
        agent.prompt(prompt).await.expect("error")
    }
}
