// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/vmware-tanzu/velero/velero.io/v1/backuprepositories.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BackupRepositorySpec is the specification for a BackupRepository.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "velero.io", version = "v1", kind = "BackupRepository", plural = "backuprepositories")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackupRepositorySpec {
    /// BackupStorageLocation is the name of the BackupStorageLocation
    /// that should contain this repository.
    #[serde(rename = "backupStorageLocation")]
    pub backup_storage_location: String,
    /// MaintenanceFrequency is how often maintenance should be run.
    #[serde(rename = "maintenanceFrequency")]
    pub maintenance_frequency: String,
    /// RepositoryType indicates the type of the backend repository
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryType")]
    pub repository_type: Option<BackupRepositoryRepositoryType>,
    /// ResticIdentifier is the full restic-compatible string for identifying
    /// this repository.
    #[serde(rename = "resticIdentifier")]
    pub restic_identifier: String,
    /// VolumeNamespace is the namespace this backup repository contains
    /// pod volume backups for.
    #[serde(rename = "volumeNamespace")]
    pub volume_namespace: String,
}

/// BackupRepositorySpec is the specification for a BackupRepository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupRepositoryRepositoryType {
    #[serde(rename = "kopia")]
    Kopia,
    #[serde(rename = "restic")]
    Restic,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// BackupRepositoryStatus is the current status of a BackupRepository.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupRepositoryStatus {
    /// LastMaintenanceTime is the last time maintenance was run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastMaintenanceTime")]
    pub last_maintenance_time: Option<String>,
    /// Message is a message about the current status of the BackupRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Phase is the current state of the BackupRepository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<BackupRepositoryStatusPhase>,
}

/// BackupRepositoryStatus is the current status of a BackupRepository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupRepositoryStatusPhase {
    New,
    Ready,
    NotReady,
}

