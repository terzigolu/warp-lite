use std::borrow::Cow;

use cynic::{GraphQlResponse, QueryFragment, QueryVariables};
use http::StatusCode;
use instant::Duration;
use serde::{de::DeserializeOwned, Serialize};
use warp_core::{channel::ChannelState, operating_system_info::OperatingSystemInfo};

use crate::{
    error::{UserFacingError, UserFacingErrorInterface},
    request_context::{ClientContext, OsContext, RequestContext},
};

#[cfg(not(target_family = "wasm"))]
pub(crate) type BoxFuture<'a, T> =
    std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[cfg(target_family = "wasm")]
pub(crate) type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + 'a>>;

/// A GraphQL operation (query or mutation) that can be executed by the server.
pub trait Operation<QF> {
    /// The name of the operation.
    fn operation_name(&self) -> Option<Cow<'_, str>>;

    /// Sends the operation to the server using the provided [`http_client::Client`] and returns the response.
    #[allow(async_fn_in_trait)]
    fn send_request(
        self,
        client: std::sync::Arc<http_client::Client>,
        options: RequestOptions,
    ) -> BoxFuture<'static, Result<GraphQlResponse<QF>, GraphQLError>>
    where
        Self: Sized;
}

/// The various errors we could encounter when making a GraphQL request to
/// warp-server.
#[derive(Debug, thiserror::Error)]
pub enum GraphQLError {
    /// Encountered an error while sending the request.
    #[error("error sending request")]
    RequestError(#[source] reqwest::Error),
    /// Not authorized to talk to the staging server.
    #[error("not authorized for staging")]
    StagingAccessBlocked,
    #[error("received non-OK response code {status}")]
    HttpError { status: StatusCode, body: String },
    #[error("Failed to deserialize GraphQL response: {0:?}")]
    ResponseError(#[source] reqwest::Error),
}

/// Options for sending a GraphQL request.
#[derive(Default)]
pub struct RequestOptions {
    /// If provided, a Bearer authentication token to provide with the request.
    pub auth_token: Option<String>,
    pub timeout: Option<Duration>,
    /// Additional HTTP headers to attach to the request.
    pub headers: std::collections::HashMap<String, String>,
    /// If provided, a prefix to attach to the request URL.
    pub path_prefix: Option<String>,
}

pub(crate) struct Request {
    req: http_client::Request,
    operation_name: String,
}

/// Builds a [`Request`] that can be sent using [`send_graphql_request`].
pub(crate) fn build_graphql_request<Q, V>(
    client: &http_client::Client,
    operation: cynic::Operation<Q, V>,
    options: RequestOptions,
) -> Result<Request, reqwest::Error>
where
    Q: QueryFragment + DeserializeOwned,
    V: QueryVariables + Serialize,
{
    let operation_name = operation
        .operation_name
        .clone()
        .map(Cow::into_owned)
        .unwrap_or_default();

    let graphql_endpoint = format!(
        "{}{}/graphql/v2?op={}",
        ChannelState::server_root_url(),
        options.path_prefix.unwrap_or_default(),
        &operation_name
    );

    let mut req = client.post(&graphql_endpoint).json(&operation);

    if let Some(auth_token) = options.auth_token {
        req = req.bearer_auth(auth_token);
    }
    if let Some(timeout) = options.timeout {
        req = req.timeout(timeout);
    }
    for (header, value) in options.headers {
        req = req.header(header, value);
    }

    Ok(Request {
        req: req.build()?,
        operation_name,
    })
}

/// Sends a [`Request`] to the server and returns the response.
///
/// warp-lite: the original implementation issued an HTTPS POST against the
/// configured GraphQL endpoint and parsed Cloud Armor / staging-allowlist
/// errors out of the response. In warp-lite the GraphQL server is gone, so
/// every request short-circuits to `HttpError { status: 410 GONE }` without
/// touching the network.
pub(crate) async fn send_graphql_request<Q>(
    _client: &http_client::Client,
    req: Request,
) -> Result<GraphQlResponse<Q>, GraphQLError>
where
    Q: QueryFragment + DeserializeOwned,
{
    let Request {
        req: _,
        operation_name,
    } = req;
    log::debug!(
        "warp-lite: short-circuiting GraphQL operation `{operation_name}` (no remote)"
    );
    Err(GraphQLError::HttpError {
        status: StatusCode::GONE,
        body: String::from("warp-lite: GraphQL server disabled"),
    })
}

/// Returns a [`RequestContext`] pre-populated as appropriate for the current client.
pub fn get_request_context() -> RequestContext {
    let (category, linux_kernel_version, name, os_version) = match OperatingSystemInfo::get() {
        Ok(os_system_info) => (
            Some(os_system_info.category().to_string()),
            os_system_info.linux_kernel_version().map(|s| s.to_string()),
            Some(os_system_info.name().to_string()),
            os_system_info.version().map(|s| s.to_string()),
        ),
        Err(_) => (None, None, None, None),
    };

    RequestContext {
        client_context: ClientContext {
            version: ChannelState::app_version().map(|s| s.to_string()),
        },
        os_context: OsContext {
            category,
            linux_kernel_version,
            name,
            version: os_version,
        },
    }
}

/// Returns a user-facing error message for the given [`UserFacingError`].
pub fn get_user_facing_error_message(e: UserFacingError) -> String {
    match e.error {
        UserFacingErrorInterface::SharedObjectsLimitExceeded(e) => e.message,
        UserFacingErrorInterface::PersonalObjectsLimitExceeded(e) => e.message,
        UserFacingErrorInterface::AccountDelinquencyError(e) => e.message,
        UserFacingErrorInterface::GenericStringObjectUniqueKeyConflict(e) => e.message,
        UserFacingErrorInterface::BudgetExceededError(e) => e.message,
        UserFacingErrorInterface::PaymentMethodDeclinedError(e) => e.message,
        UserFacingErrorInterface::InvalidAttachmentError(e) => e.message,
        UserFacingErrorInterface::Unknown(fallback) => fallback.message,
    }
}

/// Helper macro for defining GraphQL operations.
///
/// The internal implementation for each operation is basically the same, and
/// so it's much easier to define each one via a macro.
///
/// Query variable types can hold references, specifying the lifetimes in a list
/// in square brackets before the function name, e.g.:
///
/// ```ignore
/// define_operation! {
///     ['a] do_operation(OperationVariables<'a>) -> Operation;
/// }
/// ```
macro_rules! define_operation {
    { $([$($generics:tt)*])? $func:ident($vars:ty) -> $query:ty; } => {
        impl<$($($generics)*)?> $crate::client::Operation<$query>
            for cynic::Operation<$query, $vars>
        {
            fn operation_name(&self) -> Option<std::borrow::Cow<'_, str>> {
                self.operation_name.clone()
            }

            fn send_request(
                self,
                client: std::sync::Arc<http_client::Client>,
                options: $crate::client::RequestOptions,
            ) -> $crate::client::BoxFuture<'static, Result<cynic::GraphQlResponse<$query>, $crate::client::GraphQLError>>
            where
                Self: Sized,
            {
                let req = $crate::client::build_graphql_request(&client, self, options).map_err($crate::client::GraphQLError::RequestError);
                Box::pin(async move { $crate::client::send_graphql_request(&client, req?).await })
            }
        }
    };
}
pub(crate) use define_operation;
