// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshretries.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec is the specification of the Kuma MeshRetry resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshRetry", plural = "meshretries")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct MeshRetrySpec {
    /// TargetRef is a reference to the resource the policy takes an effect on.
    /// The resource could be either a real store object or virtual resource
    /// defined inplace.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshRetryTargetRef,
    /// To list makes a match between the consumed services and corresponding configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MeshRetryTo>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshRetryTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryTo {
    /// Default is a configuration specific to the group of destinations referenced in
    /// 'targetRef'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<MeshRetryToDefault>,
    /// TargetRef is a reference to the resource that represents a group of
    /// destinations.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshRetryToTargetRef,
}

/// Default is a configuration specific to the group of destinations referenced in
/// 'targetRef'
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefault {
    /// GRPC defines a configuration of retries for GRPC traffic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<MeshRetryToDefaultGrpc>,
    /// HTTP defines a configuration of retries for HTTP traffic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<MeshRetryToDefaultHttp>,
    /// TCP defines a configuration of retries for TCP traffic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<MeshRetryToDefaultTcp>,
}

/// GRPC defines a configuration of retries for GRPC traffic
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultGrpc {
    /// BackOff is a configuration of durations which will be used in an exponential
    /// backoff strategy between retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backOff")]
    pub back_off: Option<MeshRetryToDefaultGrpcBackOff>,
    /// NumRetries is the number of attempts that will be made on failed (and
    /// retriable) requests. If not set, the default value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numRetries")]
    pub num_retries: Option<i32>,
    /// PerTryTimeout is the maximum amount of time each retry attempt can take
    /// before it times out. If not set, the global request timeout for the route
    /// will be used. Setting this value to 0 will disable the per-try timeout.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perTryTimeout")]
    pub per_try_timeout: Option<String>,
    /// RateLimitedBackOff is a configuration of backoff which will be used when
    /// the upstream returns one of the headers configured.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rateLimitedBackOff")]
    pub rate_limited_back_off: Option<MeshRetryToDefaultGrpcRateLimitedBackOff>,
    /// RetryOn is a list of conditions which will cause a retry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryOn")]
    pub retry_on: Option<Vec<String>>,
}

/// BackOff is a configuration of durations which will be used in an exponential
/// backoff strategy between retries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultGrpcBackOff {
    /// BaseInterval is an amount of time which should be taken between retries.
    /// Must be greater than zero. Values less than 1 ms are rounded up to 1 ms.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseInterval")]
    pub base_interval: Option<String>,
    /// MaxInterval is a maximal amount of time which will be taken between retries.
    /// Default is 10 times the "BaseInterval".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxInterval")]
    pub max_interval: Option<String>,
}

/// RateLimitedBackOff is a configuration of backoff which will be used when
/// the upstream returns one of the headers configured.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultGrpcRateLimitedBackOff {
    /// MaxInterval is a maximal amount of time which will be taken between retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxInterval")]
    pub max_interval: Option<String>,
    /// ResetHeaders specifies the list of headers (like Retry-After or X-RateLimit-Reset)
    /// to match against the response. Headers are tried in order, and matched
    /// case-insensitive. The first header to be parsed successfully is used.
    /// If no headers match the default exponential BackOff is used instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resetHeaders")]
    pub reset_headers: Option<Vec<MeshRetryToDefaultGrpcRateLimitedBackOffResetHeaders>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultGrpcRateLimitedBackOffResetHeaders {
    /// The format of the reset header.
    pub format: MeshRetryToDefaultGrpcRateLimitedBackOffResetHeadersFormat,
    /// The Name of the reset header.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryToDefaultGrpcRateLimitedBackOffResetHeadersFormat {
    Seconds,
    UnixTimestamp,
}

/// HTTP defines a configuration of retries for HTTP traffic
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttp {
    /// BackOff is a configuration of durations which will be used in exponential
    /// backoff strategy between retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backOff")]
    pub back_off: Option<MeshRetryToDefaultHttpBackOff>,
    /// HostSelection is a list of predicates that dictate how hosts should be selected
    /// when requests are retried.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostSelection")]
    pub host_selection: Option<Vec<MeshRetryToDefaultHttpHostSelection>>,
    /// HostSelectionMaxAttempts is the maximum number of times host selection will be
    /// reattempted before giving up, at which point the host that was last selected will
    /// be routed to. If unspecified, this will default to retrying once.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostSelectionMaxAttempts")]
    pub host_selection_max_attempts: Option<i64>,
    /// NumRetries is the number of attempts that will be made on failed (and
    /// retriable) requests.  If not set, the default value is 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numRetries")]
    pub num_retries: Option<i32>,
    /// PerTryTimeout is the amount of time after which retry attempt should time out.
    /// If left unspecified, the global route timeout for the request will be used.
    /// Consequently, when using a 5xx based retry policy, a request that times out
    /// will not be retried as the total timeout budget would have been exhausted.
    /// Setting this timeout to 0 will disable it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perTryTimeout")]
    pub per_try_timeout: Option<String>,
    /// RateLimitedBackOff is a configuration of backoff which will be used
    /// when the upstream returns one of the headers configured.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rateLimitedBackOff")]
    pub rate_limited_back_off: Option<MeshRetryToDefaultHttpRateLimitedBackOff>,
    /// RetriableRequestHeaders is an HTTP headers which must be present in the request
    /// for retries to be attempted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retriableRequestHeaders")]
    pub retriable_request_headers: Option<Vec<MeshRetryToDefaultHttpRetriableRequestHeaders>>,
    /// RetriableResponseHeaders is an HTTP response headers that trigger a retry
    /// if present in the response. A retry will be triggered if any of the header
    /// matches the upstream response headers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retriableResponseHeaders")]
    pub retriable_response_headers: Option<Vec<MeshRetryToDefaultHttpRetriableResponseHeaders>>,
    /// RetryOn is a list of conditions which will cause a retry. Available values are:
    /// [5XX, GatewayError, Reset, Retriable4xx, ConnectFailure, EnvoyRatelimited,
    /// RefusedStream, Http3PostConnectFailure, HttpMethodConnect, HttpMethodDelete,
    /// HttpMethodGet, HttpMethodHead, HttpMethodOptions, HttpMethodPatch,
    /// HttpMethodPost, HttpMethodPut, HttpMethodTrace].
    /// Also, any HTTP status code (500, 503, etc.).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryOn")]
    pub retry_on: Option<Vec<String>>,
}

/// BackOff is a configuration of durations which will be used in exponential
/// backoff strategy between retries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttpBackOff {
    /// BaseInterval is an amount of time which should be taken between retries.
    /// Must be greater than zero. Values less than 1 ms are rounded up to 1 ms.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseInterval")]
    pub base_interval: Option<String>,
    /// MaxInterval is a maximal amount of time which will be taken between retries.
    /// Default is 10 times the "BaseInterval".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxInterval")]
    pub max_interval: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttpHostSelection {
    /// Type is requested predicate mode.
    pub predicate: MeshRetryToDefaultHttpHostSelectionPredicate,
    /// Tags is a map of metadata to match against for selecting the omitted hosts. Required if Type is
    /// OmitHostsWithTags
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    /// UpdateFrequency is how often the priority load should be updated based on previously attempted priorities.
    /// Used for OmitPreviousPriorities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateFrequency")]
    pub update_frequency: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryToDefaultHttpHostSelectionPredicate {
    OmitPreviousHosts,
    OmitHostsWithTags,
    OmitPreviousPriorities,
}

/// RateLimitedBackOff is a configuration of backoff which will be used
/// when the upstream returns one of the headers configured.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttpRateLimitedBackOff {
    /// MaxInterval is a maximal amount of time which will be taken between retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxInterval")]
    pub max_interval: Option<String>,
    /// ResetHeaders specifies the list of headers (like Retry-After or X-RateLimit-Reset)
    /// to match against the response. Headers are tried in order, and matched
    /// case-insensitive. The first header to be parsed successfully is used.
    /// If no headers match the default exponential BackOff is used instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resetHeaders")]
    pub reset_headers: Option<Vec<MeshRetryToDefaultHttpRateLimitedBackOffResetHeaders>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttpRateLimitedBackOffResetHeaders {
    /// The format of the reset header.
    pub format: MeshRetryToDefaultHttpRateLimitedBackOffResetHeadersFormat,
    /// The Name of the reset header.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryToDefaultHttpRateLimitedBackOffResetHeadersFormat {
    Seconds,
    UnixTimestamp,
}

/// HeaderMatch describes how to select an HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttpRetriableRequestHeaders {
    /// Name is the name of the HTTP Header to be matched. Name MUST be lower case
    /// as they will be handled with case insensitivity (See https://tools.ietf.org/html/rfc7230#section-3.2).
    pub name: String,
    /// Type specifies how to match against the value of the header.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<MeshRetryToDefaultHttpRetriableRequestHeadersType>,
    /// Value is the value of HTTP Header to be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// HeaderMatch describes how to select an HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryToDefaultHttpRetriableRequestHeadersType {
    Exact,
    Present,
    RegularExpression,
    Absent,
    Prefix,
}

/// HeaderMatch describes how to select an HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultHttpRetriableResponseHeaders {
    /// Name is the name of the HTTP Header to be matched. Name MUST be lower case
    /// as they will be handled with case insensitivity (See https://tools.ietf.org/html/rfc7230#section-3.2).
    pub name: String,
    /// Type specifies how to match against the value of the header.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<MeshRetryToDefaultHttpRetriableResponseHeadersType>,
    /// Value is the value of HTTP Header to be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// HeaderMatch describes how to select an HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryToDefaultHttpRetriableResponseHeadersType {
    Exact,
    Present,
    RegularExpression,
    Absent,
    Prefix,
}

/// TCP defines a configuration of retries for TCP traffic
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToDefaultTcp {
    /// MaxConnectAttempt is a maximal amount of TCP connection attempts
    /// which will be made before giving up
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectAttempt")]
    pub max_connect_attempt: Option<i32>,
}

/// TargetRef is a reference to the resource that represents a group of
/// destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshRetryToTargetRef {
    /// Kind of the referenced resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MeshRetryToTargetRefKind>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of
/// destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshRetryToTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
}

