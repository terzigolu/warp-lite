//! warp-lite stub for the firebase auth response types.
//!
//! Original Warp used Firebase Identity Platform for user account lookup and
//! refresh-token-to-access-token exchange. The auth flow is gone in warp-lite,
//! but the response shapes are still referenced by `app/src/server` so we
//! keep just the types needed for compilation. No network calls live here.
use serde::{Deserialize, Serialize};

/// Format for error response payloads for Google APIs.
///
/// This error format is standardized across 'v1' Google APIs; its used for both
/// POST /v1/accounts/lookup and POST /v1/token requests.
///
/// This format is documented at https://firebase.google.com/docs/reference/rest/auth#section-error-format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseError {
    pub code: i32,
    pub message: String,
}

impl std::error::Error for FirebaseError {}

impl std::fmt::Display for FirebaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Firebase request failed with status {} and message: {}",
            self.code, self.message
        )
    }
}

/// The possible response values from fetching an access token from a refresh token.
///
/// Both `expiresIn`/`expires_in` aliasing variants are kept because the original
/// Warp client mixed REST endpoints with different naming conventions; even
/// though warp-lite does not call those endpoints, dead code that still
/// references this enum needs the same shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchAccessTokenResponse {
    Success {
        #[serde(alias = "expiresIn")]
        expires_in: String,

        #[serde(alias = "idToken")]
        id_token: String,

        #[serde(alias = "refreshToken")]
        refresh_token: String,
    },
    Error {
        error: FirebaseError,
    },
}
