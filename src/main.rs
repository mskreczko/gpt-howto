#![allow(dead_code)]

use std::env;
use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String
}

impl Message {
    fn new(role: String, content: String) -> Self {
        Self {role, content}
    }
}

#[derive(Serialize, Deserialize)]
struct GPTRequestBody {
    model: String,
    messages: Vec<Message>,
    temperature: i32
}

#[derive(Deserialize)]
struct GPTUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32
}

#[derive(Deserialize)]
struct GPTChoice {
    message: Message,
    finish_reason: String,
    index: i32
}

#[derive(Deserialize)]
struct GPTResponseBody {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: GPTUsage,
    choices: Vec<GPTChoice>
}

fn call_gpt_api(input: String, headers: HeaderMap) {
    let client = reqwest::blocking::Client::new();
    let body = GPTRequestBody {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![Message::new(String::from("user"), input)],
        temperature: 0
    };

    let res = client.post(env::var("API_URL").expect("API_URL variable not set"))
        .body(serde_json::to_string(&body).unwrap())
        .headers(headers)
        .send()
        .expect("Failed to get a response")
        .json::<GPTResponseBody>();

    println!("{}", res.unwrap().choices[0].message.content);
}

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let mut headers= HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(
        &format!("Bearer {}", env::var("API_KEY").expect("API_KEY variable not set"))
    ).unwrap());


    call_gpt_api(args[1].clone(), headers);

}
