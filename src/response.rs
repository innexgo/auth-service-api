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
struct VerificationChallenge {
  creationTime: i64,
  name: String,
  email: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
  userId: i64,
  creationTime: i64,
  name: String,
  email: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PasswordReset {
  creationTime: i64,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Password {
  passwordId: i64,
  creationTime: i64,
  creator: User,
  user: User,
  kind: PasswordKind,
  passwordReset: Option<PasswordReset>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ApiKey {
  apiKeyId: i64,
  creationTime: i64,
  creator: User,
  duration: i64, // only valid if ApiKeyKind isn't CANCEL
  key: String,   // only valid if ApiKeyKind isn't CANCEL
  apiKeyKind: ApiKeyKind,
}
