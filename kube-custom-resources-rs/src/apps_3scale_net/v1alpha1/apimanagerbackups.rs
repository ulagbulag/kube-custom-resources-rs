// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale/3scale-operator/apps.3scale.net/v1alpha1/apimanagerbackups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// APIManagerBackupSpec defines the desired state of APIManagerBackup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.3scale.net", version = "v1alpha1", kind = "APIManagerBackup", plural = "apimanagerbackups")]
#[kube(namespaced)]
#[kube(status = "APIManagerBackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct APIManagerBackupSpec {
    /// Backup data destination configuration
    #[serde(rename = "backupDestination")]
    pub backup_destination: APIManagerBackupBackupDestination,
}

/// Backup data destination configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerBackupBackupDestination {
    /// PersistentVolumeClaim as backup data destination configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaim")]
    pub persistent_volume_claim: Option<APIManagerBackupBackupDestinationPersistentVolumeClaim>,
}

/// PersistentVolumeClaim as backup data destination configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerBackupBackupDestinationPersistentVolumeClaim {
    /// Resources configuration for the backup data PersistentVolumeClaim. Ignored when VolumeName field is set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<APIManagerBackupBackupDestinationPersistentVolumeClaimResources>,
    /// Storage class to be used by the PersistentVolumeClaim. Ignored when VolumeName field is set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    /// Name of an existing PersistentVolume to be bound to the backup data PersistentVolumeClaim
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeName")]
    pub volume_name: Option<String>,
}

/// Resources configuration for the backup data PersistentVolumeClaim. Ignored when VolumeName field is set
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerBackupBackupDestinationPersistentVolumeClaimResources {
    /// Storage Resource requests to be used on the PersistentVolumeClaim. To learn more about resource requests see: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    pub requests: IntOrString,
}

/// APIManagerBackupStatus defines the observed state of APIManagerBackup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerBackupStatus {
    /// Name of the APIManager from which the backup has been performed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiManagerSourceName")]
    pub api_manager_source_name: Option<String>,
    /// Name of the backup data PersistentVolumeClaim. Only set when PersistentVolumeClaim is used as the backup data destination
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupPersistentVolumeClaimName")]
    pub backup_persistent_volume_claim_name: Option<String>,
    /// Set to true when backup has been completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Backup completion time. It is represented in RFC3339 form and is in UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    /// Set to true when main steps have been completed. At this point backup still cannot be considered  fully completed due to some remaining post-backup tasks are pending (cleanup, ...)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mainStepsCompleted")]
    pub main_steps_completed: Option<bool>,
    /// Backup start time. It is represented in RFC3339 form and is in UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

