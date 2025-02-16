use reqwest::Client;
use std::collections::HashMap;

use super::models::{Beatmap, OAuthTokenResponse};

pub struct OsuService {
    client: Client,
    client_id: String,
    client_secret: String,
}

impl OsuService {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
        }
    }

    pub async fn get_authtoken(&self) -> Result<OAuthTokenResponse, reqwest::Error> {
        let params = self.get_params();

        const AUTH_URL: &str = "https://osu.ppy.sh/oauth/token";

        self.client
            .post(AUTH_URL)
            .form(&params)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_beatmap(&self, token: &str) -> Result<Option<Beatmap>, reqwest::Error> {
        //TODO change for dynamic user id
        const BEATMAP_URL: &str = "https://osu.ppy.sh/api/v2/users/23870149/scores/recent?limit=1";

        let res: Vec<Beatmap> = self
            .client
            .get(BEATMAP_URL)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .json()
            .await?;

        Ok(res.into_iter().next())
    }

    fn get_params(&self) -> HashMap<String, String> {
        HashMap::from([
            ("client_id".to_string(), self.client_id.to_string()),
            ("client_secret".to_string(), self.client_secret.to_string()),
            ("grant_type".to_string(), "client_credentials".to_string()),
            ("scope".to_string(), "public".to_string()),
        ])
    }
}
