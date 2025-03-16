use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct WebSecret {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    web: WebSecret,
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceSecret {
    pub r#type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
    pub universe_domain: String,
}

// Struct to create a JWT token payload
#[derive(Serialize)]
pub struct Claims<'a> {
    pub iss: &'a str,
    pub scope: &'a str,
    pub aud: &'a str,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: usize,
}
