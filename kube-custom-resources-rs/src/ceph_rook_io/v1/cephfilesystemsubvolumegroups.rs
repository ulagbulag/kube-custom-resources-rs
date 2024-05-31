// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/rook/rook/ceph.rook.io/v1/cephfilesystemsubvolumegroups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// Spec represents the specification of a Ceph Filesystem SubVolumeGroup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephFilesystemSubVolumeGroup", plural = "cephfilesystemsubvolumegroups")]
#[kube(namespaced)]
#[kube(status = "CephFilesystemSubVolumeGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephFilesystemSubVolumeGroupSpec {
    /// The data pool name for the Ceph Filesystem subvolume group layout, if the default CephFS pool is not desired.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataPoolName")]
    pub data_pool_name: Option<String>,
    /// FilesystemName is the name of Ceph Filesystem SubVolumeGroup volume name. Typically it's the name of
    /// the CephFilesystem CR. If not coming from the CephFilesystem CR, it can be retrieved from the
    /// list of Ceph Filesystem volumes with `ceph fs volume ls`. To learn more about Ceph Filesystem
    /// abstractions see https://docs.ceph.com/en/latest/cephfs/fs-volumes/#fs-volumes-and-subvolumes
    #[serde(rename = "filesystemName")]
    pub filesystem_name: String,
    /// The name of the subvolume group. If not set, the default is the name of the subvolumeGroup CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Pinning configuration of CephFilesystemSubVolumeGroup,
    /// reference https://docs.ceph.com/en/latest/cephfs/fs-volumes/#pinning-subvolumes-and-subvolume-groups
    /// only one out of (export, distributed, random) can be set at a time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinning: Option<CephFilesystemSubVolumeGroupPinning>,
    /// Quota size of the Ceph Filesystem subvolume group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<IntOrString>,
}

/// Pinning configuration of CephFilesystemSubVolumeGroup,
/// reference https://docs.ceph.com/en/latest/cephfs/fs-volumes/#pinning-subvolumes-and-subvolume-groups
/// only one out of (export, distributed, random) can be set at a time
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemSubVolumeGroupPinning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distributed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random: Option<f64>,
}

/// Status represents the status of a CephFilesystem SubvolumeGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephFilesystemSubVolumeGroupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<BTreeMap<String, String>>,
    /// ObservedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ConditionType represent a resource's status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

