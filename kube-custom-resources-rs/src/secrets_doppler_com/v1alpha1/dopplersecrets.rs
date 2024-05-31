// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/DopplerHQ/kubernetes-operator/secrets.doppler.com/v1alpha1/dopplersecrets.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DopplerSecretSpec defines the desired state of DopplerSecret
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "secrets.doppler.com", version = "v1alpha1", kind = "DopplerSecret", plural = "dopplersecrets")]
#[kube(namespaced)]
#[kube(status = "DopplerSecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct DopplerSecretSpec {
    /// The Doppler config
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Format enables the downloading of secrets as a file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<DopplerSecretFormat>,
    /// The Doppler API host
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The Kubernetes secret where the operator will store and sync the fetched secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managedSecret")]
    pub managed_secret: Option<DopplerSecretManagedSecret>,
    /// The environment variable compatible secrets name transformer to apply
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameTransformer")]
    pub name_transformer: Option<DopplerSecretNameTransformer>,
    /// A list of processors to transform the data during ingestion
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<BTreeMap<String, DopplerSecretProcessors>>,
    /// The Doppler project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The number of seconds to wait between resyncs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resyncSeconds")]
    pub resync_seconds: Option<i64>,
    /// A list of secrets to sync from the config
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,
    /// The Kubernetes secret containing the Doppler service token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenSecret")]
    pub token_secret: Option<DopplerSecretTokenSecret>,
    /// Whether or not to verify TLS
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyTLS")]
    pub verify_tls: Option<bool>,
}

/// DopplerSecretSpec defines the desired state of DopplerSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DopplerSecretFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "dotnet-json")]
    DotnetJson,
    #[serde(rename = "env")]
    Env,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "docker")]
    Docker,
}

/// The Kubernetes secret where the operator will store and sync the fetched secrets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DopplerSecretManagedSecret {
    /// The name of the Secret resource
    pub name: String,
    /// Namespace of the resource being referred to. Ignored if not cluster scoped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The secret type of the managed secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DopplerSecretManagedSecretType>,
}

/// The Kubernetes secret where the operator will store and sync the fetched secrets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DopplerSecretManagedSecretType {
    Opaque,
    #[serde(rename = "kubernetes.io/tls")]
    KubernetesIoTls,
    #[serde(rename = "kubernetes.io/service-account-token")]
    KubernetesIoServiceAccountToken,
    #[serde(rename = "kubernetes.io/dockercfg")]
    KubernetesIoDockercfg,
    #[serde(rename = "kubernetes.io/dockerconfigjson")]
    KubernetesIoDockerconfigjson,
    #[serde(rename = "kubernetes.io/basic-auth")]
    KubernetesIoBasicAuth,
    #[serde(rename = "kubernetes.io/ssh-auth")]
    KubernetesIoSshAuth,
    #[serde(rename = "bootstrap.kubernetes.io/token")]
    BootstrapKubernetesIoToken,
}

/// DopplerSecretSpec defines the desired state of DopplerSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DopplerSecretNameTransformer {
    #[serde(rename = "upper-camel")]
    UpperCamel,
    #[serde(rename = "camel")]
    Camel,
    #[serde(rename = "lower-snake")]
    LowerSnake,
    #[serde(rename = "tf-var")]
    TfVar,
    #[serde(rename = "dotnet-env")]
    DotnetEnv,
    #[serde(rename = "lower-kebab")]
    LowerKebab,
}

/// A list of processors to transform the data during ingestion
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DopplerSecretProcessors {
    /// The mapped name of the field in the managed secret, defaults to the original Doppler secret name for Opaque Kubernetes secrets. If omitted for other types, the value is not copied to the managed secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asName")]
    pub as_name: Option<String>,
    /// The type of process to be performed, either "plain" or "base64"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DopplerSecretProcessorsType>,
}

/// A list of processors to transform the data during ingestion
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DopplerSecretProcessorsType {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "base64")]
    Base64,
}

/// The Kubernetes secret containing the Doppler service token
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DopplerSecretTokenSecret {
    /// The name of the Secret resource
    pub name: String,
    /// Namespace of the resource being referred to. Ignored if not cluster scoped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// DopplerSecretStatus defines the observed state of DopplerSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DopplerSecretStatus {
    pub conditions: Vec<Condition>,
}

