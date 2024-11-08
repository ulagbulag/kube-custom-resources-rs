// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/runtime.cluster.x-k8s.io/v1alpha1/extensionconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ExtensionConfigSpec is the desired state of the ExtensionConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "runtime.cluster.x-k8s.io", version = "v1alpha1", kind = "ExtensionConfig", plural = "extensionconfigs")]
#[kube(status = "ExtensionConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ExtensionConfigSpec {
    /// clientConfig defines how to communicate with the Extension server.
    #[serde(rename = "clientConfig")]
    pub client_config: ExtensionConfigClientConfig,
    /// namespaceSelector decides whether to call the hook for an object based
    /// on whether the namespace for that object matches the selector.
    /// Defaults to the empty LabelSelector, which matches all objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<ExtensionConfigNamespaceSelector>,
    /// settings defines key value pairs to be passed to all calls
    /// to all supported RuntimeExtensions.
    /// Note: Settings can be overridden on the ClusterClass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<BTreeMap<String, String>>,
}

/// clientConfig defines how to communicate with the Extension server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigClientConfig {
    /// caBundle is a PEM encoded CA bundle which will be used to validate the Extension server's server certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caBundle")]
    pub ca_bundle: Option<String>,
    /// service is a reference to the Kubernetes service for the Extension server.
    /// Note: Exactly one of `url` or `service` must be specified.
    /// 
    /// If the Extension server is running within a cluster, then you should use `service`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ExtensionConfigClientConfigService>,
    /// url gives the location of the Extension server, in standard URL form
    /// (`scheme://host:port/path`).
    /// Note: Exactly one of `url` or `service` must be specified.
    /// 
    /// The scheme must be "https".
    /// 
    /// The `host` should not refer to a service running in the cluster; use
    /// the `service` field instead.
    /// 
    /// A path is optional, and if present may be any string permissible in
    /// a URL. If a path is set it will be used as prefix to the hook-specific path.
    /// 
    /// Attempting to use a user or basic auth e.g. "user:password@" is not
    /// allowed. Fragments ("#...") and query parameters ("?...") are not
    /// allowed either.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// service is a reference to the Kubernetes service for the Extension server.
/// Note: Exactly one of `url` or `service` must be specified.
/// 
/// If the Extension server is running within a cluster, then you should use `service`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigClientConfigService {
    /// name is the name of the service.
    pub name: String,
    /// namespace is the namespace of the service.
    pub namespace: String,
    /// path is an optional URL path and if present may be any string permissible in
    /// a URL. If a path is set it will be used as prefix to the hook-specific path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// port is the port on the service that's hosting the Extension server.
    /// Defaults to 443.
    /// Port should be a valid port number (1-65535, inclusive).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

/// namespaceSelector decides whether to call the hook for an object based
/// on whether the namespace for that object matches the selector.
/// Defaults to the empty LabelSelector, which matches all objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ExtensionConfigNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// ExtensionConfigStatus is the current state of the ExtensionConfig
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigStatus {
    /// conditions define the current service state of the ExtensionConfig.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// handlers defines the current ExtensionHandlers supported by an Extension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handlers: Option<Vec<ExtensionConfigStatusHandlers>>,
}

/// ExtensionHandler specifies the details of a handler for a particular runtime hook registered by an Extension server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigStatusHandlers {
    /// failurePolicy defines how failures in calls to the ExtensionHandler should be handled by a client.
    /// Defaults to Fail if not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failurePolicy")]
    pub failure_policy: Option<String>,
    /// name is the unique name of the ExtensionHandler.
    pub name: String,
    /// requestHook defines the versioned runtime hook which this ExtensionHandler serves.
    #[serde(rename = "requestHook")]
    pub request_hook: ExtensionConfigStatusHandlersRequestHook,
    /// timeoutSeconds defines the timeout duration for client calls to the ExtensionHandler.
    /// Defaults to 10 is not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
}

/// requestHook defines the versioned runtime hook which this ExtensionHandler serves.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtensionConfigStatusHandlersRequestHook {
    /// apiVersion is the group and version of the Hook.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// hook is the name of the hook.
    pub hook: String,
}

