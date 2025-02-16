use dotenv::dotenv;
use std::{env, fs};

pub fn get_env(var: &str) -> Result<String, String> {
    dotenv().ok();

    env::var(var).map_err(|e| {
        let error_message = format!("Error accessing '{var}' environment variable: {}", e);
        println!("{}", error_message);
        error_message
    })
}

pub fn create_file(file_name: &str, content: &str) {
    let _ = fs::write(file_name, content);
}

pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_name)
}
