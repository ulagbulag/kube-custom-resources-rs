// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/ansible/awx-operator/awx.ansible.com/v1beta1/awxbackups.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "awx.ansible.com", version = "v1beta1", kind = "AWXBackup", plural = "awxbackups")]
#[kube(namespaced)]
#[kube(status = "AWXBackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AWXBackupSpec {
    /// Additional labels defined on the resource, which should be propagated to child resources
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_labels: Option<Vec<String>>,
    /// Name of the backup PVC
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_pvc: Option<String>,
    /// (Deprecated) Namespace the PVC is in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_pvc_namespace: Option<String>,
    /// Resource requirements for the management pod used to create a backup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_resource_requirements: Option<AWXBackupBackupResourceRequirements>,
    /// Storage class to use when creating PVC for backup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_class: Option<String>,
    /// Storage requirements for backup PVC (may be similar to existing postgres PVC backing up from)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_requirements: Option<String>,
    /// Flag to indicate if backup should be deleted on PVC if AWXBackup object is deleted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clean_backup_on_delete: Option<bool>,
    /// nodeSelector for the Postgres pods to backup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_management_pod_node_selector: Option<String>,
    /// Name of the deployment to be backed up
    pub deployment_name: String,
    /// The image pull policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<AWXBackupImagePullPolicy>,
    /// Configure no_log for no_log tasks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_log: Option<bool>,
    /// Additional parameters for the pg_dump command
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pg_dump_suffix: Option<String>,
    /// Registry path to the PostgreSQL container to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_image: Option<String>,
    /// PostgreSQL container image version to use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_image_version: Option<String>,
    /// Label selector used to identify postgres pod for backing up data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres_label_selector: Option<String>,
    /// Number of hours worth of events table partitions to precreate before backup to avoid pg_dump locks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precreate_partition_hours: Option<i32>,
    /// Maintain some of the recommended `app.kubernetes.io/*` labels on the resource (self)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_self_labels: Option<bool>,
}

/// Resource requirements for the management pod used to create a backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXBackupBackupResourceRequirements {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<AWXBackupBackupResourceRequirementsLimits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<AWXBackupBackupResourceRequirementsRequests>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXBackupBackupResourceRequirementsLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXBackupBackupResourceRequirementsRequests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AWXBackupImagePullPolicy {
    Always,
    #[serde(rename = "always")]
    AlwaysX,
    Never,
    #[serde(rename = "never")]
    NeverX,
    IfNotPresent,
    #[serde(rename = "ifnotpresent")]
    Ifnotpresent,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXBackupStatus {
    /// Backup persistent volume claim
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupClaim")]
    pub backup_claim: Option<String>,
    /// Backup directory name on the specified pvc
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupDirectory")]
    pub backup_directory: Option<String>,
    /// The resulting conditions when a Service Telemetry is instantiated
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AWXBackupStatusConditions>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWXBackupStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

