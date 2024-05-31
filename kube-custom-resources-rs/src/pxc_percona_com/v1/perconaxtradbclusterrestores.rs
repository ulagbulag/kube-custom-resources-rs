// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/percona/percona-xtradb-cluster-operator/pxc.percona.com/v1/perconaxtradbclusterrestores.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "pxc.percona.com", version = "v1", kind = "PerconaXtraDBClusterRestore", plural = "perconaxtradbclusterrestores")]
#[kube(namespaced)]
#[kube(status = "PerconaXtraDBClusterRestoreStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct PerconaXtraDBClusterRestoreSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupName")]
    pub backup_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSource")]
    pub backup_source: Option<PerconaXtraDBClusterRestoreBackupSource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerOptions")]
    pub container_options: Option<PerconaXtraDBClusterRestoreContainerOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pitr: Option<PerconaXtraDBClusterRestorePitr>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pxcCluster")]
    pub pxc_cluster: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PerconaXtraDBClusterRestoreResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreBackupSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<PerconaXtraDBClusterRestoreBackupSourceAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastscheduled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<PerconaXtraDBClusterRestoreBackupSourceS3>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslInternalSecretName")]
    pub ssl_internal_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslSecretName")]
    pub ssl_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultSecretName")]
    pub vault_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyTLS")]
    pub verify_tls: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreBackupSourceAzure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreBackupSourceS3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<PerconaXtraDBClusterRestoreContainerOptionsArgs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<PerconaXtraDBClusterRestoreContainerOptionsEnv>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsArgs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xbcloud: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xbstream: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xtrabackup: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsEnv {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<PerconaXtraDBClusterRestoreContainerOptionsEnvValueFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsEnvValueFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromConfigMapKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromFieldRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromResourceFieldRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromSecretKeyRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromConfigMapKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromFieldRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromResourceFieldRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    pub resource: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreContainerOptionsEnvValueFromSecretKeyRef {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestorePitr {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSource")]
    pub backup_source: Option<PerconaXtraDBClusterRestorePitrBackupSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gtid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestorePitrBackupSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<PerconaXtraDBClusterRestorePitrBackupSourceAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastscheduled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<PerconaXtraDBClusterRestorePitrBackupSourceS3>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslInternalSecretName")]
    pub ssl_internal_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslSecretName")]
    pub ssl_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultSecretName")]
    pub vault_secret_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyTLS")]
    pub verify_tls: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestorePitrBackupSourceAzure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestorePitrBackupSourceS3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PerconaXtraDBClusterRestoreResourcesClaims>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreResourcesClaims {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PerconaXtraDBClusterRestoreStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lastscheduled: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

