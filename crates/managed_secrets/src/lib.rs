// warp-lite v0.4 cloud purge:
// `warp_managed_secrets` is reduced to an inert stub. The original crate
// performed Tink/HPKE encryption against Warp's GCP-hosted secrets
// service. In warp-lite there is no remote secrets backend, so every
// operation surfaces as `Err("inert")`. The public API surface is kept
// so call sites in `app/` and the AI subsystem still compile.
//
// Original LOC ~1655 across 10 files; stubbed to ~150 LOC in a single
// monolith. All cryptographic dependencies (tink-*, hpke, zeroize, rand)
// have been dropped from `Cargo.toml`.

#![allow(dead_code)]
#![allow(clippy::needless_pass_by_value)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use warp_graphql::managed_secrets::{ManagedSecret, ManagedSecretType};
use warpui::{Entity, SingletonEntity};

pub use warp_graphql::queries::task_secrets::ManagedSecretValue as GqlManagedSecretValue;

// =====================================================================
// Public surface re-exports (matches original `lib.rs`)
// =====================================================================

pub use self::client::TaskIdentityToken;
pub use self::gcp::{
    GcpCredentials, GcpFederationConfig, GcpWorkloadIdentityFederationError,
    GcpWorkloadIdentityFederationToken, PrepareGcpCredentialsError,
};

// `init_envelope` and `UploadKey` were the entry points to the HPKE
// envelope encryption layer. In the inert stub neither is needed at
// runtime, but a no-op `init_envelope` is preserved for any caller that
// still references it.
pub fn init_envelope() {}

pub struct UploadKey;

// =====================================================================
// `ManagedSecretValue` — variants are preserved verbatim because consumers
// destructure them (e.g. `ManagedSecretValue::RawValue { value }`).
// =====================================================================

#[derive(Serialize)]
#[serde(untagged)]
pub enum ManagedSecretValue {
    RawValue {
        value: String,
    },
    AnthropicApiKey {
        api_key: String,
    },
    AnthropicBedrockAccessKey {
        aws_access_key_id: String,
        aws_secret_access_key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        aws_session_token: Option<String>,
        aws_region: String,
    },
    AnthropicBedrockApiKey {
        aws_bearer_token_bedrock: String,
        aws_region: String,
    },
}

impl ManagedSecretValue {
    pub fn raw_value(s: impl Into<String>) -> Self {
        Self::RawValue { value: s.into() }
    }
    pub fn anthropic_api_key(s: impl Into<String>) -> Self {
        Self::AnthropicApiKey { api_key: s.into() }
    }
    pub fn anthropic_bedrock_access_key(
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
        session_token: Option<String>,
        region: impl Into<String>,
    ) -> Self {
        Self::AnthropicBedrockAccessKey {
            aws_access_key_id: access_key_id.into(),
            aws_secret_access_key: secret_access_key.into(),
            aws_session_token: session_token,
            aws_region: region.into(),
        }
    }
    pub fn anthropic_bedrock_api_key(
        token: impl Into<String>,
        region: impl Into<String>,
    ) -> Self {
        Self::AnthropicBedrockApiKey {
            aws_bearer_token_bedrock: token.into(),
            aws_region: region.into(),
        }
    }
    pub fn secret_type(&self) -> ManagedSecretType {
        match self {
            ManagedSecretValue::RawValue { .. } => ManagedSecretType::RawValue,
            ManagedSecretValue::AnthropicApiKey { .. } => ManagedSecretType::AnthropicApiKey,
            ManagedSecretValue::AnthropicBedrockAccessKey { .. } => {
                ManagedSecretType::AnthropicBedrockAccessKey
            }
            ManagedSecretValue::AnthropicBedrockApiKey { .. } => {
                ManagedSecretType::AnthropicBedrockApiKey
            }
        }
    }
}

impl std::fmt::Debug for ManagedSecretValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Never print secret material.
        f.debug_struct("ManagedSecretValue").finish_non_exhaustive()
    }
}

// =====================================================================
// `ActorProvider` — implemented by `AuthState` in `app/src/auth`. Trait
// shape is preserved.
// =====================================================================

pub trait ActorProvider: Send + Sync + 'static {
    fn actor_uid(&self) -> Option<String>;
}

// =====================================================================
// `ManagedSecretManager` — surface preserved, every async method is an
// inert error (no network, no encryption).
// =====================================================================

pub struct ManagedSecretManager {
    _client: Arc<dyn client::ManagedSecretsClient>,
    _actor_provider: Arc<dyn ActorProvider>,
}

impl ManagedSecretManager {
    pub fn new(
        client: Arc<dyn client::ManagedSecretsClient>,
        actor_provider: Arc<dyn ActorProvider>,
    ) -> Self {
        Self {
            _client: client,
            _actor_provider: actor_provider,
        }
    }

    pub async fn create_secret(
        &self,
        _owner: client::SecretOwner,
        _name: String,
        _value: ManagedSecretValue,
        _description: Option<String>,
    ) -> anyhow::Result<ManagedSecret> {
        Err(anyhow::anyhow!("warp-lite: managed secrets are disabled"))
    }

    pub async fn delete_secret(
        &self,
        _owner: client::SecretOwner,
        _name: String,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("warp-lite: managed secrets are disabled"))
    }

    pub async fn update_secret(
        &self,
        _owner: client::SecretOwner,
        _name: String,
        _value: Option<ManagedSecretValue>,
        _description: Option<String>,
    ) -> anyhow::Result<ManagedSecret> {
        Err(anyhow::anyhow!("warp-lite: managed secrets are disabled"))
    }

    pub async fn list_secrets(&self) -> anyhow::Result<Vec<ManagedSecret>> {
        Ok(Vec::new())
    }

    pub async fn get_task_secrets(
        &self,
        _task_id: String,
    ) -> anyhow::Result<HashMap<String, ManagedSecretValue>> {
        Ok(HashMap::new())
    }

    pub async fn issue_task_identity_token(
        &self,
        _options: client::IdentityTokenOptions,
    ) -> anyhow::Result<TaskIdentityToken> {
        Err(anyhow::anyhow!("warp-lite: identity tokens are disabled"))
    }

    pub async fn issue_gcp_workload_identity_federation_token(
        &self,
        _audience: String,
        _token_type: String,
        _requested_duration: Duration,
    ) -> Result<GcpWorkloadIdentityFederationToken, GcpWorkloadIdentityFederationError> {
        Err(GcpWorkloadIdentityFederationError::new(
            "warp-lite: identity federation is disabled",
        ))
    }
}

impl Entity for ManagedSecretManager {
    type Event = ();
}
impl SingletonEntity for ManagedSecretManager {}

// =====================================================================
// `client` submodule — surface for trait + types.
// =====================================================================

pub mod client {
    use std::collections::HashMap;
    use std::time::Duration;

    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use vec1::Vec1;
    use warp_graphql::managed_secrets::{ManagedSecret, ManagedSecretConfig, ManagedSecretType};

    pub use warp_graphql::queries::task_secrets::ManagedSecretValue;

    #[derive(Debug, Clone)]
    pub struct TaskIdentityToken {
        pub token: String,
        pub expires_at: DateTime<Utc>,
        pub issuer: String,
    }

    pub struct IdentityTokenOptions {
        pub audience: String,
        pub requested_duration: Duration,
        pub subject_template: Vec1<String>,
    }

    #[derive(Debug, Default)]
    pub struct ManagedSecretConfigs {
        pub user_secrets: Option<ManagedSecretConfig>,
        pub team_secrets: HashMap<String, ManagedSecretConfig>,
    }

    #[derive(Debug, Clone)]
    pub enum SecretOwner {
        CurrentUser,
        Team { team_uid: String },
    }

    #[cfg_attr(not(target_family = "wasm"), async_trait)]
    #[cfg_attr(target_family = "wasm", async_trait(?Send))]
    pub trait ManagedSecretsClient: 'static + Send + Sync {
        async fn get_managed_secret_configs(&self) -> anyhow::Result<ManagedSecretConfigs>;
        async fn create_managed_secret(
            &self,
            owner: SecretOwner,
            name: String,
            secret_type: ManagedSecretType,
            encrypted_value: String,
            description: Option<String>,
        ) -> anyhow::Result<ManagedSecret>;
        async fn delete_managed_secret(
            &self,
            owner: SecretOwner,
            name: String,
        ) -> anyhow::Result<()>;
        async fn update_managed_secret(
            &self,
            owner: SecretOwner,
            name: String,
            encrypted_value: Option<String>,
            description: Option<String>,
        ) -> anyhow::Result<ManagedSecret>;
        async fn list_secrets(&self) -> anyhow::Result<Vec<ManagedSecret>>;
        async fn get_task_secrets(
            &self,
            task_id: String,
            workload_token: String,
        ) -> anyhow::Result<HashMap<String, ManagedSecretValue>>;
        async fn issue_task_identity_token(
            &self,
            options: IdentityTokenOptions,
        ) -> anyhow::Result<TaskIdentityToken>;
    }
}

// =====================================================================
// `gcp` submodule — `GcpCredentials::federated()` is now an inert error.
// =====================================================================

pub mod gcp {
    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::time::Duration;

    use serde::{Deserialize, Serialize};

    use super::client::TaskIdentityToken;

    const VERSION: u8 = 1;
    pub(crate) const TOKEN_TYPE_ID_TOKEN: &str = "urn:ietf:params:oauth:token-type:id_token";
    pub(crate) const TOKEN_TYPE_JWT: &str = "urn:ietf:params:oauth:token-type:jwt";

    #[derive(Debug, Clone)]
    pub struct GcpFederationConfig {
        pub project_number: String,
        pub pool_id: String,
        pub provider_id: String,
        pub service_account_email: Option<String>,
        pub token_lifetime: Option<Duration>,
    }

    pub struct GcpCredentials;

    impl GcpCredentials {
        pub fn federated(
            _task_id: &str,
            _config: &GcpFederationConfig,
        ) -> Result<Self, PrepareGcpCredentialsError> {
            Err(PrepareGcpCredentialsError::NoBinaryPath)
        }

        pub fn env_vars(&self) -> HashMap<OsString, OsString> {
            HashMap::new()
        }

        pub fn cleanup(self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum PrepareGcpCredentialsError {
        #[error("warp-lite: GCP federation is disabled")]
        NoBinaryPath,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct GcpWorkloadIdentityFederationToken {
        pub version: u8,
        pub success: bool,
        pub token_type: String,
        pub id_token: String,
        pub expiration_time: i64,
    }

    impl GcpWorkloadIdentityFederationToken {
        pub(crate) fn new(token: TaskIdentityToken, token_type: String) -> Self {
            Self {
                version: VERSION,
                success: true,
                token_type,
                id_token: token.token,
                expiration_time: token.expires_at.timestamp(),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct GcpWorkloadIdentityFederationError {
        pub version: u8,
        pub success: bool,
        pub code: String,
        pub message: String,
    }

    impl GcpWorkloadIdentityFederationError {
        pub fn new(message: impl Into<String>) -> Self {
            Self {
                version: VERSION,
                success: false,
                code: "TOKEN_ISSUANCE_FAILED".into(),
                message: message.into(),
            }
        }
    }
}
