// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/supportbundles.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// SupportBundleSpec defines the desired state of the Longhorn SupportBundle
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "SupportBundle", plural = "supportbundles")]
#[kube(namespaced)]
#[kube(status = "SupportBundleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SupportBundleSpec {
    /// A brief description of the issue
    pub description: String,
    /// The issue URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issueURL")]
    pub issue_url: Option<String>,
    /// The preferred responsible controller node ID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
}

/// SupportBundleStatus defines the observed state of the Longhorn SupportBundle
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SupportBundleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,
    /// The support bundle manager image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The support bundle manager IP
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerIP")]
    pub manager_ip: Option<String>,
    /// The current responsible controller node ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

