// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubean-io/kubean/kubean.io/v1alpha1/localartifactsets.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kubean.io", version = "v1alpha1", kind = "LocalArtifactSet", plural = "localartifactsets")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct LocalArtifactSetSpec {
    /// Arch for x86_64  aarch64... , represent for the arch of this offline package
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker: Option<Vec<LocalArtifactSetDocker>>,
    /// Items cni containerd kubeadm kube etcd cilium calico
    pub items: Vec<LocalArtifactSetItems>,
    /// Kubespray , the tag of kubespray
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubespray: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalArtifactSetDocker {
    pub os: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionRange")]
    pub version_range: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalArtifactSetItems {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionRange")]
    pub version_range: Option<Vec<String>>,
}

