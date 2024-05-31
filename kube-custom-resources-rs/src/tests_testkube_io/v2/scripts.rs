// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeshop/testkube-operator/tests.testkube.io/v2/scripts.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ScriptSpec defines the desired state of Script
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tests.testkube.io", version = "v2", kind = "Script", plural = "scripts")]
#[kube(namespaced)]
#[kube(status = "ScriptStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ScriptSpec {
    /// script content object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ScriptContent>,
    /// script execution custom name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// execution params passed to executor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    /// script tags
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// script type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// script content object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScriptContent {
    /// script content body
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// repository of script content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<ScriptContentRepository>,
    /// script type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// uri of script content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// repository of script content
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScriptContentRepository {
    /// branch/tag name for checkout
    pub branch: String,
    /// if needed we can checkout particular path (dir or file) in case of BIG/mono repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// git auth token for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// VCS repository type
    #[serde(rename = "type")]
    pub r#type: String,
    /// uri of content file or git directory
    pub uri: String,
    /// git auth username for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// ScriptStatus defines the observed state of Script
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScriptStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executions_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_execution: Option<String>,
}

