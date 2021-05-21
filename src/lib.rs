// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyNewValidProps {
  pub user_email: String,
  pub user_password: String,
  pub duration: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyNewCancelProps {
  pub api_key_to_cancel: String,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationChallengeNewProps {
  pub user_name: String,
  pub user_email: String,
  pub user_password: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserNewProps {
  pub verification_challenge_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordResetNewProps {
  pub user_email: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordNewChangeProps {
  pub user_id: i64,
  pub old_password: String,
  pub new_password: String,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordNewCancelProps {
  pub user_id: i64,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordNewResetProps {
  pub password_reset_key: String,
  pub new_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserViewProps {
  pub user_id: Option<i64>,              //
  pub creation_time: Option<i64>,        //
  pub min_creation_time: Option<i64>,    //
  pub max_creation_time: Option<i64>,    //
  pub user_name: Option<String>,         //
  pub partial_user_name: Option<String>, //
  pub user_email: Option<String>,        //
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
pub enum PasswordKind {
  CHANGE,
  RESET,
  CANCEL,
}

impl TryFrom<u8> for PasswordKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<PasswordKind, u8> {
    match val {
      x if x == PasswordKind::CHANGE as u8 => Ok(PasswordKind::CHANGE),
      x if x == PasswordKind::RESET as u8 => Ok(PasswordKind::RESET),
      x if x == PasswordKind::CANCEL as u8 => Ok(PasswordKind::CANCEL),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordViewProps {
  pub password_id: Option<i64>,            //
  pub creation_time: Option<i64>,          //
  pub min_creation_time: Option<i64>,      //
  pub max_creation_time: Option<i64>,      //
  pub creator_user_id: Option<i64>,        //
  pub user_id: Option<i64>,                //
  pub password_kind: Option<PasswordKind>, //
  pub only_recent: bool,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
pub enum ApiKeyKind {
  VALID,
  CANCEL,
}

impl TryFrom<u8> for ApiKeyKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<ApiKeyKind, u8> {
    match val {
      x if x == ApiKeyKind::VALID as u8 => Ok(ApiKeyKind::VALID),
      x if x == ApiKeyKind::CANCEL as u8 => Ok(ApiKeyKind::CANCEL),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyViewProps {
  pub api_key_id: Option<i64>,          //
  pub creator_user_id: Option<i64>,     //
  pub creation_time: Option<i64>,       //
  pub min_creation_time: Option<i64>,   //
  pub max_creation_time: Option<i64>,   //
  pub duration: Option<i64>,            //
  pub min_duration: Option<i64>,        //
  pub max_duration: Option<i64>,        //
  pub api_key_kind: Option<ApiKeyKind>, //
  pub only_recent: bool,                //
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}
