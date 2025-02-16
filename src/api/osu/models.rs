use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthTokenResponse {
    pub token_type: String,
    pub expires_in: i32,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BeatmapSet {
    artist_unicode: String,
    title_unicode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Beatmap {
    beatmapset: BeatmapSet,
    created_at: String,
}
