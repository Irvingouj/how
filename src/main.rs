use std::{env};
mod command_handler;
use crate::command_handler::CommandHandler;
use regex::Regex;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<_> = env::args().collect();
    let mut iter = args.iter();

    let mut full_line = String::new();
    let first = iter.next().unwrap_or_else(|| panic!("Please provide a command"));
    
    match trim_question_word(first) {
        Ok(word) => {
            full_line.push_str(word.as_str());
        }
        Err(e) => {
            panic!("Error: {}", e)
        }
    }

    for arg in iter {
        full_line.push_str(" ");
        full_line.push_str(arg);
    }

    let handler = CommandHandler::new();

    if start_with_question_word_with_arg(&full_line) {
        handler.handle_input_with_start_args(&full_line).await?;
    }

    if end_with_arg(&full_line) {
        handler.handle_input_with_end_args(&full_line).await?;
    }
    
    handler.handle(&full_line).await?;
    Ok(())

}

// args supported next to question words: -v --version, -h --help
fn start_with_question_word_with_arg(input: &String) -> bool {
    let question_words = ["how", "what", "why", "when", "is"];

    let re = Regex::new(r"(how|what|why|when|is) (-([a-z])|--(.*)).*").unwrap();
    if !re.is_match(input){
        return false;
    }


    for word in question_words.iter() {
        if input.starts_with(word) {
            return true;
        }
    }
    return false;
}

fn end_with_arg(input: &String) -> bool {
    let re = Regex::new(r".* (-([a-z])|--(.*)).*").unwrap();
    if !re.is_match(input){
        return false;
    }
    return true;
}

fn trim_question_word(first_word: &String) -> Result<String,Box<dyn std::error::Error>> {
    let question_words = ["how", "what", "why", "when", "is"];
    let path_to_executable = split_input(first_word);
    let command_name = path_to_executable.last().unwrap_or_else(|| panic!("Please provide a command"));

    for word in question_words.iter() {
        if command_name.starts_with(word) {
            return Ok(word.to_string());
        }
    }


    Err("No question word found".into())
}

#[cfg(target_os = "windows")]
fn split_input(first_word: &String) -> Vec<&str> {
    first_word.split("\\").collect()
}

#[cfg(not(target_os = "windows"))]
fn split_input(input: &String) -> Vec<&str> {
    input.split("/").collect()
}