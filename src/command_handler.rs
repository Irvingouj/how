use std::{path, fs};

use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::api::Client;

pub struct CommandHandler;

impl CommandHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self,input: &String) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::current_dir()
            .unwrap()
            .join(path::Path::new("openai_key"));
        println!("dir: {:?}", dir);
        let open_ai_key = fs::read_to_string(dir.to_str().unwrap()).unwrap();
        let client = Client::new(open_ai_key);
        let req = ChatCompletionRequest {
            model: chat_completion::GPT3_5_TURBO.to_string(),
            messages: vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: String::from(input),
            }],
        };
        let result = client.chat_completion(req).await?;
        println!("{}", result.choices[0].message.content);
        Ok(())
    }

    pub async fn handle_input_with_start_args(&self,input: &String) -> Result<(), Box<dyn std::error::Error>> {
        let arg = input.split(" ").collect::<Vec<&str>>()[1];
        match arg {
            "-v" | "--version" => self.print_version(),
            "-h" | "--help" => self.print_help(),
            _ => panic!("Unknown argument: {}", arg),
        }
        Ok(())
    }


    pub async fn handle_input_with_end_args(&self,input: &String) -> Result<(), Box<dyn std::error::Error>> {
        let inputs = input.split(" ").collect::<Vec<&str>>();
        let arg = inputs[inputs.len() - 1];
        match arg {
            "-c" | "--concise" => self.handle_concise(input).await?,
            "-e" | "--explain" => self.handle(input).await?,
            _ => panic!("Unknown argument: {}", arg),
        }
        Ok(())
    }

    fn print_version(&self){
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("Version: {}", VERSION);
    }

    fn print_help(&self){
        println!("help");
    }

    async fn handle_concise(&self, input: &String) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::current_dir()
            .unwrap()
            .join(path::Path::new("openai_key"));
        println!("dir: {:?}", dir);
        let open_ai_key = fs::read_to_string(dir.to_str().unwrap()).unwrap();
        let client = Client::new(open_ai_key);
        let req = ChatCompletionRequest {
            model: chat_completion::GPT3_5_TURBO.to_string(),
            messages: vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::user,
                content: String::from(input),
            }],
        };
        let result = client.chat_completion(req).await?;
        println!("{}", result.choices[0].message.content);
        Ok(())
    }
    

}
