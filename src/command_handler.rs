use std::{path, fs, dbg};
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::completion::{CompletionRequest, self};

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
        dbg!(&VERSION);
        println!("Version: {}", VERSION);
    }

    fn print_help(&self){
        println!("Usage: how [your question] [options]");
        println!("Options on binary:");
        println!("  -v, --version  print version");
        println!("  -h, --help     print help");
        println!("Options on question:");
        println!("  -c, --concise  print concise answer");
        println!("  -e, --explain  print explanation");
    }

    async fn handle_concise(&self, input: &String) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::current_dir()
            .unwrap()
            .join(path::Path::new("openai_key"));
        println!("dir: {:?}", dir);
        let open_ai_key = fs::read_to_string(dir.to_str().unwrap()).unwrap();
        let client = Client::new(open_ai_key);
        let req = CompletionRequest {
            model: completion::GPT3_TEXT_DAVINCI_003.to_string(),
            prompt: Some(String::from(input)),
            suffix: None,
            max_tokens: Some(3000),
            temperature: Some(0.9),
            top_p: Some(1.0),
            n: None,
            stream: None,
            logprobs: None,
            echo: None,
            stop: None,
            presence_penalty: Some(0.6),
            frequency_penalty: Some(0.0),
            best_of: None,
            logit_bias: None,
            user: None,
          };
        let result = client.completion(req).await?;
        println!("{}", result.choices[0].text);
        Ok(())
    }
    

}
