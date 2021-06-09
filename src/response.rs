use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::request::{PasswordKind};

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthError {
  NoCapability,
  ApiKeyUnauthorized,
  PasswordIncorrect,
  PasswordInsecure,
  PasswordCannotCreateForOthers,
  UserNonexistent,
  ApiKeyNonexistent,
  UserExistent,
  UserNameEmpty,
  UserEmailEmpty,
  UserEmailInvalidated,
  NegativeDuration,
  CannotAlterPast,
  VerificationChallengeNonexistent,
  VerificationChallengeTimedOut,
  PasswordResetNonexistent,
  PasswordExistent,
  PasswordNonexistent,
  PasswordResetTimedOut,
  EmailBounced,
  EmailUnknown,
  NetworkError,
  DecodeError,
  InternalServerError,
  MethodNotAllowed,
  BadRequest,
  NotFound,
  Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationChallenge {
  pub creation_time: i64,
  pub name: String,
  pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub user_id: i64,
  pub creation_time: i64,
  pub name: String,
  pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordReset {
  pub creation_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Password {
  pub password_id: i64,
  pub creation_time: i64,
  pub creator: User,
  pub password_kind: PasswordKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "api_key_kind")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiKeyData {
  // the interior of the struct should be normal, but the VALID and CANCEL tags should be screaming case
  #[serde(rename_all = "camelCase")]
  Valid {
      key: Option<String>,
      duration: i64
  },
  Cancel
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
  pub api_key_id: i64,
  pub creation_time: i64,
  pub creator: User,
  #[serde(flatten)]
  pub api_key_data: ApiKeyData,
}
