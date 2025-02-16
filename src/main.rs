use std::path::Path;

use api::osu::osu::OsuService;
use osu_fm::{create_file, get_env};

mod api;

#[tokio::main]
async fn main() {
    if !Path::new("token.txt").exists() {
        first_entry().await;
    }
}

async fn first_entry() {
    let (client_id, client_secret) = match (get_env("CLIENT_ID"), get_env("CLIENT_SECRET")) {
        (Ok(client_id), Ok(client_secret)) => (client_id, client_secret),
        _ => return,
    };

    let osu_service = OsuService::new(client_id, client_secret);

    match osu_service.get_authtoken().await {
        Ok(response) => {
            create_file("token.txt", &response.access_token);
        }
        Err(e) => {
            println!("Error to get OSU token. {}", e);
        }
    }
}
