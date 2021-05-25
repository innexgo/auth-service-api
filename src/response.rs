use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::request::{ApiKeyKind, PasswordKind};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
pub enum AuthError {
  NOT_FOUND,
  NO_CAPABILITY,
  API_KEY_UNAUTHORIZED,
  PASSWORD_INCORRECT,
  PASSWORD_INSECURE,
  PASSWORD_CANNOT_CREATE_FOR_OTHERS,
  USER_NONEXISTENT,
  API_KEY_NONEXISTENT,
  USER_EXISTENT,
  USER_NAME_EMPTY,
  USER_EMAIL_EMPTY,
  USER_EMAIL_INVALIDATED,
  NEGATIVE_DURATION,
  CANNOT_ALTER_PAST,
  VERIFICATION_CHALLENGE_NONEXISTENT,
  VERIFICATION_CHALLENGE_TIMED_OUT,
  PASSWORD_RESET_NONEXISTENT,
  PASSWORD_EXISTENT,
  PASSWORD_NONEXISTENT,
  PASSWORD_RESET_TIMED_OUT,
  EMAIL_RATELIMIT,
  EMAIL_BLACKLISTED,
  UNKNOWN,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationChallenge {
  pub creationTime: i64,
  pub name: String,
  pub email: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
  pub userId: i64,
  pub creationTime: i64,
  pub name: String,
  pub email: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordReset {
  pub creationTime: i64,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Password {
  pub passwordId: i64,
  pub creationTime: i64,
  pub creator: User,
  pub user: User,
  pub kind: PasswordKind,
  pub passwordReset: Option<PasswordReset>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKey {
  pub apiKeyId: i64,
  pub creationTime: i64,
  pub creator: User,
  pub apiKeyKind: ApiKeyKind,
  pub duration: Option<i64>, // only valid if ApiKeyKind isn't CANCEL
  pub key: Option<String>,   // only valid if ApiKeyKind isn't CANCEL
}
