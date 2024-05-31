// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/physicalmachines.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec defines the behavior of a physical machine
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "PhysicalMachine", plural = "physicalmachines")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PhysicalMachineSpec {
    /// Address represents the address of the physical machine
    pub address: String,
}

