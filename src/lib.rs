// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyNewValidProps {
  user_email: String,
  user_password: String,
  duration: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyNewCancelProps {
  api_key_to_cancel: String,
  api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationChallengeNewProps {
  user_name: String,
  user_email: String,
  user_password: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserNewProps {
  verification_challenge_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordResetNewProps {
  user_email: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordNewChangeProps {
  user_id: u64,
  old_password: String,
  new_password: String,
  api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordNewCancelProps {
  user_id: u64,
  api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordNewResetProps {
  password_reset_key: String,
  new_password: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserViewProps {
  user_id: Option<u64>,              //
  creation_time: Option<u64>,        //
  min_creation_time: Option<u64>,    //
  max_creation_time: Option<u64>,    //
  user_name: Option<String>,         //
  partial_user_name: Option<String>, //
  user_email: Option<String>,        //
  offset: Option<u64>,
  count: Option<u64>,
  api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PasswordKind {
  Change,
  Reset,
  Cancel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordViewProps {
  password_id: Option<u64>,            //
  creation_time: Option<u64>,          //
  min_creation_time: Option<u64>,      //
  max_creation_time: Option<u64>,      //
  creator_user_id: Option<u64>,        //
  user_id: Option<u64>,                //
  password_kind: Option<PasswordKind>, //
  only_recent: Option<bool>,
  offset: Option<u64>,
  count: Option<u64>,
  api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiKeyKind {
  Valid,
  Cancel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiKeyViewProps {
  api_key_id: Option<u64>,          //
  creator_user_id: Option<u64>,     //
  creation_time: Option<u64>,       //
  min_creation_time: Option<u64>,   //
  max_creation_time: Option<u64>,   //
  duration: Option<u64>,            //
  min_duration: Option<u64>,        //
  max_duration: Option<u64>,        //
  api_key_kind: Option<ApiKeyKind>, //
  only_recent: Option<bool>,        //
  offset: Option<u64>,
  count: Option<u64>,
  api_key: String,
}
