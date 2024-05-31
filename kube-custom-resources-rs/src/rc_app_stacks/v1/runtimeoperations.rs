// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/application-stacks/runtime-component-operator/rc.app.stacks/v1/runtimeoperations.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Defines the desired state of RuntimeOperation
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "rc.app.stacks", version = "v1", kind = "RuntimeOperation", plural = "runtimeoperations")]
#[kube(namespaced)]
#[kube(status = "RuntimeOperationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RuntimeOperationSpec {
    /// Command to execute. Not executed within a shell.
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Name of the Pod to perform runtime operation on. Pod must be from the same namespace as the RuntimeOperation instance.
    #[serde(rename = "podName")]
    pub pod_name: String,
}

/// Defines the observed state of RuntimeOperation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuntimeOperationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation identifier of this RuntimeOperation instance completely reconciled by the Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<RuntimeOperationStatusVersions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuntimeOperationStatusVersions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciled: Option<String>,
}

