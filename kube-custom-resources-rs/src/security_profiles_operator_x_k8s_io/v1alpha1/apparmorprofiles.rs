// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/security-profiles-operator/security-profiles-operator.x-k8s.io/v1alpha1/apparmorprofiles.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AppArmorProfileSpec defines the desired state of AppArmorProfile
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "security-profiles-operator.x-k8s.io", version = "v1alpha1", kind = "AppArmorProfile", plural = "apparmorprofiles")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AppArmorProfileSpec {
    pub policy: String,
}

/// AppArmorProfileStatus defines the observed state of AppArmorProfile
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AppArmorProfileStatus {
}

