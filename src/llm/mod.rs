use rig_core::{
    agent::{Agent, stream_to_stdout},
    client::CompletionClient,
    memory::InMemoryConversationMemory,
    providers::llamafile::{Client, CompletionModel, LLAMA_CPP},
    streaming::StreamingPrompt,
};

use crate::{
    eidos::Eidos,
    llm::{create::Create, fetch::Fetch},
};

mod create;
mod fetch;

pub struct LLM {
    client: Client,
    agent: Agent<CompletionModel>,
    eidos: Eidos,
}

impl LLM {
    pub fn new(eidos: &Eidos) -> Self {
        let client = Client::from_url("http://localhost:8080").unwrap();
        let agent = Self::agent(&client, eidos);

        Self {
            client,
            agent,
            eidos: eidos.clone(),
        }
    }

    pub fn new_agent(&mut self) {
        self.agent = Self::agent(&self.client, &self.eidos)
    }

    fn agent(client: &Client, eidos: &Eidos) -> Agent<CompletionModel> {
        let memory = InMemoryConversationMemory::new();
        client
            .agent(LLAMA_CPP)
            .memory(memory)
            .preamble("You are a note taker assistant. Use the provided tools.")
            .tools(vec![
                Box::new(Create::new(eidos.clone())),
                Box::new(Fetch::new(eidos.clone())),
            ])
            .default_max_turns(10)
            .build()
    }

    pub async fn ask(&self, prompt: &str) -> String {
        let mut stream = self.agent.stream_prompt(prompt).await;
        let res = stream_to_stdout(&mut stream).await;
        match res {
            Ok(r) => r.response().to_string(),
            Err(e) => e.to_string(),
        }
    }
}
