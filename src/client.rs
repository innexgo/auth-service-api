use reqwest::Client;

use super::request;
use super::response;

#[derive(Clone)]
pub struct AuthService {
    client: Client,
    auth_service_url: String,
}

impl AuthService {
    pub fn new(auth_service_url: &str) -> Self {
        AuthService {
            auth_service_url: String::from(auth_service_url),
            client: Client::new(),
        }
    }

    // If the ApiKey is valid, returns the user it refers to.
    // If the ApiKey is invalid or the user doesn't exist, returns an error
    pub async fn get_user_by_api_key_if_valid(
        &self,
        api_key: String,
    ) -> Result<response::User, response::AuthError> {
        let resp = self
            .client
            .post(format!(
                "{}/get_user_by_api_key_if_valid",
                self.auth_service_url
            ))
            .json(&request::GetUserByApiKeyIfValid { api_key })
            .send()
            .await
            .map_err(|_| response::AuthError::Network)?;

        if resp.status().as_u16() == 200 {
            Ok(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        } else {
            Err(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        }
    }

    // if the user_id is valid, returns a user.
    // if the user is invalid, returns an error
    pub async fn get_user_by_id(
        &self,
        user_id: i64,
    ) -> Result<response::User, response::AuthError> {
        let resp = self
            .client
            .post(format!("{}/get_user_by_id", self.auth_service_url))
            .json(&request::GetUserByIdProps { user_id })
            .send()
            .await
            .map_err(|_| response::AuthError::Network)?;

        if resp.status().as_u16() == 200 {
            Ok(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        } else {
            Err(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        }
    }

    // fetches api information
    pub async fn info(&self) -> Result<response::Info, response::AuthError> {
        let resp = self
            .client
            .post(format!("{}/public/info", self.auth_service_url))
            .send()
            .await
            .map_err(|_| response::AuthError::Network)?;

        if resp.status().as_u16() == 200 {
            Ok(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        } else {
            Err(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        }
    }

    // log in via email
    pub async fn api_key_new_with_email(
        &self,
        email: String,
        password: String,
        duration: i64,
    ) -> Result<response::ApiKey, response::AuthError> {
        let resp = self
            .client
            .post(format!(
                "{}/public/api_key/new_with_email",
                self.auth_service_url
            ))
            .json(&request::ApiKeyNewWithEmailProps {
                email,
                password,
                duration,
            })
            .send()
            .await
            .map_err(|_| response::AuthError::Network)?;

        if resp.status().as_u16() == 200 {
            Ok(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        } else {
            Err(resp
                .json()
                .await
                .map_err(|_| response::AuthError::DecodeError)?)
        }
    }
}
