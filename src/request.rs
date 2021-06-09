// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyNewValidProps {
  pub user_email: String,
  pub user_password: String,
  pub duration: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyNewCancelProps {
  pub api_key_to_cancel: String,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationChallengeNewProps {
  pub user_name: String,
  pub user_email: String,
  pub user_password: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserNewProps {
  pub verification_challenge_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetNewProps {
  pub user_email: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordNewChangeProps {
  pub old_password: String,
  pub new_password: String,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordNewCancelProps {
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordNewResetProps {
  pub password_reset_key: String,
  pub new_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PasswordKind {
  Change,
  Reset,
  Cancel,
}

impl TryFrom<u8> for PasswordKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<PasswordKind, u8> {
    match val {
      x if x == PasswordKind::Change as u8 => Ok(PasswordKind::Change),
      x if x == PasswordKind::Reset as u8 => Ok(PasswordKind::Reset),
      x if x == PasswordKind::Cancel as u8 => Ok(PasswordKind::Cancel),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiKeyKind {
  Valid,
  Cancel,
}

impl TryFrom<u8> for ApiKeyKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<ApiKeyKind, u8> {
    match val {
      x if x == ApiKeyKind::Valid as u8 => Ok(ApiKeyKind::Valid),
      x if x == ApiKeyKind::Cancel as u8 => Ok(ApiKeyKind::Cancel),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

// Private Api
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByIdProps {
  pub user_id: i64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserByApiKeyIfValid {
  pub api_key: String
}
