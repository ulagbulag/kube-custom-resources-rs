// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/vmware-tanzu/velero/velero.io/v2alpha1/datadownloads.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DataDownloadSpec is the specification for a DataDownload.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "velero.io", version = "v2alpha1", kind = "DataDownload", plural = "datadownloads")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DataDownloadSpec {
    /// BackupStorageLocation is the name of the backup storage location
    /// where the backup repository is stored.
    #[serde(rename = "backupStorageLocation")]
    pub backup_storage_location: String,
    /// Cancel indicates request to cancel the ongoing DataDownload. It can be set
    /// when the DataDownload is in InProgress phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel: Option<bool>,
    /// DataMoverConfig is for data-mover-specific configuration fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataMoverConfig")]
    pub data_mover_config: Option<BTreeMap<String, String>>,
    /// DataMover specifies the data mover to be used by the backup.
    /// If DataMover is "" or "velero", the built-in data mover will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datamover: Option<String>,
    /// OperationTimeout specifies the time used to wait internal operations,
    /// before returning error as timeout.
    #[serde(rename = "operationTimeout")]
    pub operation_timeout: String,
    /// SnapshotID is the ID of the Velero backup snapshot to be restored from.
    #[serde(rename = "snapshotID")]
    pub snapshot_id: String,
    /// SourceNamespace is the original namespace where the volume is backed up from.
    /// It may be different from SourcePVC's namespace if namespace is remapped during restore.
    #[serde(rename = "sourceNamespace")]
    pub source_namespace: String,
    /// TargetVolume is the information of the target PVC and PV.
    #[serde(rename = "targetVolume")]
    pub target_volume: DataDownloadTargetVolume,
}

/// TargetVolume is the information of the target PVC and PV.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataDownloadTargetVolume {
    /// Namespace is the target namespace
    pub namespace: String,
    /// PV is the name of the target PV that is created by Velero restore
    pub pv: String,
    /// PVC is the name of the target PVC that is created by Velero restore
    pub pvc: String,
}

/// DataDownloadStatus is the current status of a DataDownload.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataDownloadStatus {
    /// CompletionTimestamp records the time a restore was completed.
    /// Completion time is recorded even on failed restores.
    /// The server's time is used for CompletionTimestamps
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTimestamp")]
    pub completion_timestamp: Option<String>,
    /// Message is a message about the DataDownload's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Node is name of the node where the DataDownload is processed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// Phase is the current state of the DataDownload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<DataDownloadStatusPhase>,
    /// Progress holds the total number of bytes of the snapshot and the current
    /// number of restored bytes. This can be used to display progress information
    /// about the restore operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<DataDownloadStatusProgress>,
    /// StartTimestamp records the time a restore was started.
    /// The server's time is used for StartTimestamps
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTimestamp")]
    pub start_timestamp: Option<String>,
}

/// DataDownloadStatus is the current status of a DataDownload.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DataDownloadStatusPhase {
    New,
    Accepted,
    Prepared,
    InProgress,
    Canceling,
    Canceled,
    Completed,
    Failed,
}

/// Progress holds the total number of bytes of the snapshot and the current
/// number of restored bytes. This can be used to display progress information
/// about the restore operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataDownloadStatusProgress {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bytesDone")]
    pub bytes_done: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalBytes")]
    pub total_bytes: Option<i64>,
}

