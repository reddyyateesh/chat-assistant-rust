extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const API_KEY: &str = "..."; // Your API key here

#[derive(Serialize, Deserialize)]
struct HistoryItem {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatData {
    model: String,
    messages: Vec<HistoryItem>,
    max_tokens: u32,
    n: u32,
    temperature: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
}

#[derive(Serialize, Deserialize)]
struct ResponseData {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn generate_groq_prompt(prompt: &str, model: &str) -> Option<String> {
    let url = "https://api.groq.com/openai/v1/chat/completions";

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", API_KEY).parse().unwrap());

    let history = vec![HistoryItem {
        role: "user".to_string(),
        content: prompt.to_string(),
    }];

    let data = ChatData {
        model: model.to_string(),
        messages: history,
        max_tokens: 2048,
        n: 1,
        temperature: 0.9,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&data) // Ensure `json` method is available by including `reqwest` with `json` feature
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let response_data: ResponseData = response.json().await.unwrap();
                Some(response_data.choices[0].message.content.clone())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread() // Create multi-threaded runtime
        .enable_all() // Enable all features
        .build()
        .unwrap()
        .block_on(async {
            println!("Assistant: Hello and welcome! How can I assist you today?");

            loop {
                print!("User: ");
                io::stdout().flush().unwrap();
                let mut prompt = String::new();
                io::stdin().read_line(&mut prompt).unwrap();

                let prompt = prompt.trim();
                if prompt.is_empty() {
                    continue;
                }

                match generate_groq_prompt(prompt, "llama3-70b-8192").await {
                    Some(response) => {
                        println!("Assistant: {}", response);
                    }
                    None => {
                        println!("Assistant: Sorry, I'm unable to assist with that particular request. If there's anything else you need help with, please feel free to let me know!");
                    }
                }
            }
        });
}
