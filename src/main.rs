use std::{env, fs, path};
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::api::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<_> = env::args().collect();
    let mut iter = args.iter();

    let mut full_line = "how".to_owned();
    iter.next();
    for arg in iter {
        full_line.push_str(" ");
        full_line.push_str(arg);
    }
    println!("full_line: {}", full_line);

    let dir = std::env::current_dir().unwrap().join(path::Path::new("openai_key"));
    println!("dir: {:?}", dir);
    let open_ai_key = fs::read_to_string(dir.to_str().unwrap()).unwrap();
    let client = Client::new(open_ai_key);
    let req = ChatCompletionRequest {
            model: chat_completion::GPT3_5_TURBO.to_string(),
            messages: vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: String::from(full_line),
            }],
        };
        let result = client.chat_completion(req).await?;
        println!("{}", result.choices[0].message.content);

    Ok(())

}