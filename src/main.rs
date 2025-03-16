mod secret;
use anyhow::{Context, Result};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::Client;
use secret::{Claims, ServiceSecret, TokenResponse};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events";
    let token_response = get_access_token()
        .await
        .context("Unable to grab access token.")?;
    let client = Client::new();
    let response = client
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", token_response.access_token),
        )
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {}", response);
    Ok(())
}

async fn get_access_token() -> Result<TokenResponse> {
    let secret_json = &fs::read_to_string("service-secret.json")
        .context("service-secret.json could not be found at root of project.")?;

    let secret: ServiceSecret = serde_json::from_str(secret_json)
        .context("service-secret.json failed to parse successfully.")?;

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;

    let claims = Claims {
        iss: &secret.client_email,
        scope: "https://www.googleapis.com/auth/calendar", // Change scope if needed
        aud: &secret.token_uri,
        exp: now + 3600, // Token valid for 1 hour
        iat: now,
    };

    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(secret.private_key.as_bytes())?,
    )?;

    let client = Client::new();
    let res = client
        .post(&secret.token_uri)
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await?;

    let token_response: TokenResponse = res.json().await?;

    Ok(token_response)
}
