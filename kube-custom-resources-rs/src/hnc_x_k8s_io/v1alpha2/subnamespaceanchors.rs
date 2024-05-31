// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/hierarchical-namespaces/hnc.x-k8s.io/v1alpha2/subnamespaceanchors.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hnc.x-k8s.io", version = "v1alpha2", kind = "SubnamespaceAnchor", plural = "subnamespaceanchors")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SubnamespaceAnchorSpec {
    /// Annotations is a list of annotations and values to apply to the current subnamespace and all of its descendants. All annotation keys must match a regex specified on the command line by --managed-namespace-annotation. All annotation keys must be managed annotations (see HNC docs) and must match a regex
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<SubnamespaceAnchorAnnotations>>,
    /// Labels is a list of labels and values to apply to the current subnamespace and all of its descendants. All label keys must match a regex specified on the command line by --managed-namespace-label. All label keys must be managed labels (see HNC docs) and must match a regex
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<SubnamespaceAnchorLabels>>,
}

/// MetaKVP represents a label or annotation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnamespaceAnchorAnnotations {
    /// Key is the name of the label or annotation. It must conform to the normal rules for Kubernetes label/annotation keys.
    pub key: String,
    /// Value is the value of the label or annotation. It must confirm to the normal rules for Kubernetes label or annoation values, which are far more restrictive for labels than for anntations.
    pub value: String,
}

/// MetaKVP represents a label or annotation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnamespaceAnchorLabels {
    /// Key is the name of the label or annotation. It must conform to the normal rules for Kubernetes label/annotation keys.
    pub key: String,
    /// Value is the value of the label or annotation. It must confirm to the normal rules for Kubernetes label or annoation values, which are far more restrictive for labels than for anntations.
    pub value: String,
}

/// SubnamespaceAnchorStatus defines the observed state of SubnamespaceAnchor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SubnamespaceAnchorStatus {
    /// Describes the state of the subnamespace anchor. 
    ///  Currently, the supported values are: 
    ///  - "Missing": the subnamespace has not been created yet. This should be the default state when the anchor is just created. 
    ///  - "Ok": the subnamespace exists. This is the only good state of the anchor. 
    ///  - "Conflict": a namespace of the same name already exists. The admission controller will attempt to prevent this. 
    ///  - "Forbidden": the anchor was created in a namespace that doesn't allow children, such as kube-system or hnc-system. The admission controller will attempt to prevent this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

