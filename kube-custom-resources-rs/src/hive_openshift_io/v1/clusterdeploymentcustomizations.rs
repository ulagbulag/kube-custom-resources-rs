// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/hive/hive.openshift.io/v1/clusterdeploymentcustomizations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ClusterDeploymentCustomizationSpec defines the desired state of ClusterDeploymentCustomization.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "ClusterDeploymentCustomization", plural = "clusterdeploymentcustomizations")]
#[kube(namespaced)]
#[kube(status = "ClusterDeploymentCustomizationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterDeploymentCustomizationSpec {
    /// InstallConfigPatches is a list of patches to be applied to the install-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "installConfigPatches")]
    pub install_config_patches: Option<Vec<ClusterDeploymentCustomizationInstallConfigPatches>>,
}

/// PatchEntity represent a json patch (RFC 6902) to be applied to the install-config
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDeploymentCustomizationInstallConfigPatches {
    /// From is the json path to copy or move the value from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Op is the operation to perform: add, remove, replace, move, copy, test
    pub op: String,
    /// Path is the json path to the value to be modified
    pub path: String,
    /// Value is the value to be used in the operation
    pub value: String,
}

/// ClusterDeploymentCustomizationStatus defines the observed state of ClusterDeploymentCustomization.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDeploymentCustomizationStatus {
    /// ClusterDeploymentRef is a reference to the cluster deployment that this customization is applied on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDeploymentRef")]
    pub cluster_deployment_ref: Option<ClusterDeploymentCustomizationStatusClusterDeploymentRef>,
    /// ClusterPoolRef is the name of the current cluster pool the CDC used at.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterPoolRef")]
    pub cluster_pool_ref: Option<ClusterDeploymentCustomizationStatusClusterPoolRef>,
    /// Conditions describes the state of the operator's reconciliation functionality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastAppliedConfiguration contains the last applied patches to the install-config. The information will retain for reference in case the customization is updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAppliedConfiguration")]
    pub last_applied_configuration: Option<String>,
}

/// ClusterDeploymentRef is a reference to the cluster deployment that this customization is applied on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDeploymentCustomizationStatusClusterDeploymentRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ClusterPoolRef is the name of the current cluster pool the CDC used at.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterDeploymentCustomizationStatusClusterPoolRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

