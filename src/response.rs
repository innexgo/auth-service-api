use serde::{Deserialize, Serialize};
use derive_more::Display;
use super::request;

#[derive(Clone, Debug, Serialize, Deserialize, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthError {
    NoCapability,
    ApiKeyUnauthorized,
    NoPermission,
    PasswordIncorrect,
    PasswordInsecure,
    PasswordCannotCreateForOthers,
    UserNonexistent,
    UserDataNonexistent,
    ApiKeyNonexistent,
    UserUsernameInvalid,
    UserUsernameTaken,
    UserRealnameInvalid,
    UserDateofbirthInvalid,
    NegativeDuration,
    CannotAlterPast,
    VerificationChallengeNonexistent,
    VerificationChallengeTimedOut,
    VerificationChallengeUsed,
    VerificationChallengeWrongKind,
    PasswordExistent,
    PasswordNonexistent,
    EmailExistent,
    EmailNonexistent,
    PasswordResetNonexistent,
    PasswordResetTimedOut,
    EmailBounced,
    EmailCooldown,
    DecodeError,
    InternalServerError,
    MethodNotAllowed,
    BadRequest,
    NotFound,
    Unknown,
    Network,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationChallenge {
    pub creation_time: i64,
    pub to_parent: bool,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i64,
    pub creation_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub user_data_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub dateofbirth: i64,
    pub username: String,
    pub realname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub email_id: i64,
    pub creation_time: i64,
    pub verification_challenge: VerificationChallenge,
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
    pub creator_user_id: i64,
    pub password_reset: Option<PasswordReset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub api_key_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub api_key_kind: request::ApiKeyKind,
    pub key: Option<String>,
    pub duration: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub service: String,
    pub version_major: i64,
    pub version_minor: i64,
    pub version_rev: i64,
}
