// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gitlab-org/cloud-native/gitlab-operator/apps.gitlab.com/v1beta1/gitlabs.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Specification of the desired behavior of a GitLab instance.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.gitlab.com", version = "v1beta1", kind = "GitLab", plural = "gitlabs")]
#[kube(namespaced)]
#[kube(status = "GitLabStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct GitLabSpec {
    /// The specification of GitLab Chart that is used to deploy the instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart: Option<GitLabChart>,
}

/// The specification of GitLab Chart that is used to deploy the instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitLabChart {
    /// ChartValues is the set of Helm values that is used to render the GitLab Chart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<BTreeMap<String, serde_json::Value>>,
    /// ChartVersion is the semantic version of the GitLab Chart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Most recently observed status of the GitLab instance. It is read-only to the user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GitLabStatus {
    pub conditions: Vec<Condition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

