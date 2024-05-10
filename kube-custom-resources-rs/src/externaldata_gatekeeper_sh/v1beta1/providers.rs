// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/open-policy-agent/gatekeeper/externaldata.gatekeeper.sh/v1beta1/providers.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec defines the Provider specifications.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "externaldata.gatekeeper.sh", version = "v1beta1", kind = "Provider", plural = "providers")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ProviderSpec {
    /// CABundle is a base64-encoded string that contains the TLS CA bundle in PEM format.
    /// It is used to verify the signature of the provider's certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caBundle")]
    pub ca_bundle: Option<String>,
    /// Timeout is the timeout when querying the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// URL is the url for the provider. URL is prefixed with https://.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

