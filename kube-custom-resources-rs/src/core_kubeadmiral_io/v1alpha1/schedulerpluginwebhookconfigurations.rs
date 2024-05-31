// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/schedulerpluginwebhookconfigurations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "core.kubeadmiral.io", version = "v1alpha1", kind = "SchedulerPluginWebhookConfiguration", plural = "schedulerpluginwebhookconfigurations")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SchedulerPluginWebhookConfigurationSpec {
    /// Path for the filter call, empty if not supported. This path is appended to the URLPrefix when issuing the filter call to webhook.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterPath")]
    pub filter_path: Option<String>,
    /// HTTPTimeout specifies the timeout duration for a call to the webhook. Timeout fails the scheduling of the workload. Defaults to 5 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpTimeout")]
    pub http_timeout: Option<String>,
    /// PayloadVersions is an ordered list of preferred request and response versions the webhook expects. The scheduler will try to use the first version in the list which it supports. If none of the versions specified in this list supported by the scheduler, scheduling will fail for this object.
    #[serde(rename = "payloadVersions")]
    pub payload_versions: Vec<String>,
    /// Path for the score call, empty if not supported. This verb is appended to the URLPrefix when issuing the score call to webhook.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scorePath")]
    pub score_path: Option<String>,
    /// Path for the select call, empty if not supported. This verb is appended to the URLPrefix when issuing the select call to webhook.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selectPath")]
    pub select_path: Option<String>,
    /// TLSConfig specifies the transport layer security config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsConfig")]
    pub tls_config: Option<SchedulerPluginWebhookConfigurationTlsConfig>,
    /// URLPrefix at which the webhook is available
    #[serde(rename = "urlPrefix")]
    pub url_prefix: String,
}

/// TLSConfig specifies the transport layer security config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SchedulerPluginWebhookConfigurationTlsConfig {
    /// CAData holds PEM-encoded bytes (typically read from a root certificates bundle).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caData")]
    pub ca_data: Option<String>,
    /// CertData holds PEM-encoded bytes (typically read from a client certificate file).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certData")]
    pub cert_data: Option<String>,
    /// Server should be accessed without verifying the TLS certificate. For testing only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    /// KeyData holds PEM-encoded bytes (typically read from a client certificate key file).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyData")]
    pub key_data: Option<String>,
    /// ServerName is passed to the server for SNI and is used in the client to check server certificates against. If ServerName is empty, the hostname used to contact the server is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<String>,
}

