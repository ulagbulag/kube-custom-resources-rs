// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/dataprotection.kubeblocks.io/v1alpha1/actionsets.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// ActionSetSpec defines the desired state of ActionSet
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "dataprotection.kubeblocks.io", version = "v1alpha1", kind = "ActionSet", plural = "actionsets")]
#[kube(status = "ActionSetStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ActionSetSpec {
    /// Specifies the backup action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<ActionSetBackup>,
    /// Specifies the backup type. Supported values include: 
    ///  - `Full` for a full backup. - `Incremental` back up data that have changed since the last backup (either full or incremental). - `Differential` back up data that has changed since the last full backup. - `Continuous` back up transaction logs continuously, such as MySQL binlog, PostgreSQL WAL, etc. 
    ///  Continuous backup is essential for implementing Point-in-Time Recovery (PITR).
    #[serde(rename = "backupType")]
    pub backup_type: String,
    /// Specifies a list of environment variables to be set in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<ActionSetEnv>>,
    /// Specifies a list of sources to populate environment variables in the container. The keys within a source must be a C_IDENTIFIER. Any invalid keys will be reported as an event when the container starts. If a key exists in multiple sources, the value from the last source will take precedence. Any values defined by an Env with a duplicate key will take precedence. 
    ///  This field cannot be updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envFrom")]
    pub env_from: Option<Vec<ActionSetEnvFrom>>,
    /// Specifies the restore action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<ActionSetRestore>,
}

/// Specifies the backup action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackup {
    /// Represents the action to be performed for backing up data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupData")]
    pub backup_data: Option<ActionSetBackupBackupData>,
    /// Represents a set of actions that should be executed after the backup process has completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postBackup")]
    pub post_backup: Option<Vec<ActionSetBackupPostBackup>>,
    /// Represents a set of actions that should be executed before the backup process begins.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preBackup")]
    pub pre_backup: Option<Vec<ActionSetBackupPreBackup>>,
    /// Represents a custom deletion action that can be executed before the built-in deletion action. Note: The preDelete action job will ignore the env/envFrom.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preDelete")]
    pub pre_delete: Option<ActionSetBackupPreDelete>,
}

/// Represents the action to be performed for backing up data.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupBackupData {
    /// Defines the commands to back up the volume data.
    pub command: Vec<String>,
    /// Specifies the image of the backup container.
    pub image: String,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetBackupBackupDataOnError>,
    /// Determines whether to run the job workload on the target pod node. If the backup container needs to mount the target pod's volumes, this field should be set to true. Otherwise, the target pod's volumes will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runOnTargetPodNode")]
    pub run_on_target_pod_node: Option<bool>,
    /// Determines if the backup progress should be synchronized and the interval for synchronization in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncProgress")]
    pub sync_progress: Option<ActionSetBackupBackupDataSyncProgress>,
}

/// Represents the action to be performed for backing up data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetBackupBackupDataOnError {
    Continue,
    Fail,
}

/// Determines if the backup progress should be synchronized and the interval for synchronization in seconds.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupBackupDataSyncProgress {
    /// Determines if the backup progress should be synchronized. If set to true, a sidecar container will be instantiated to synchronize the backup progress with the Backup Custom Resource (CR) status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Defines the interval in seconds for synchronizing the backup progress.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "intervalSeconds")]
    pub interval_seconds: Option<i32>,
}

/// ActionSpec defines an action that should be executed. Only one of the fields may be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPostBackup {
    /// Specifies that the action should be executed using the pod's exec API within a container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ActionSetBackupPostBackupExec>,
    /// Specifies that the action should be executed by a Kubernetes Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<ActionSetBackupPostBackupJob>,
}

/// Specifies that the action should be executed using the pod's exec API within a container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPostBackupExec {
    /// Defines the command and arguments to be executed.
    pub command: Vec<String>,
    /// Specifies the container within the pod where the command should be executed. If not specified, the first container in the pod is used by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetBackupPostBackupExecOnError>,
    /// Specifies the maximum duration to wait for the hook to complete before considering the execution a failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Specifies that the action should be executed using the pod's exec API within a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetBackupPostBackupExecOnError {
    Continue,
    Fail,
}

/// Specifies that the action should be executed by a Kubernetes Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPostBackupJob {
    /// Defines the commands to back up the volume data.
    pub command: Vec<String>,
    /// Specifies the image of the backup container.
    pub image: String,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetBackupPostBackupJobOnError>,
    /// Determines whether to run the job workload on the target pod node. If the backup container needs to mount the target pod's volumes, this field should be set to true. Otherwise, the target pod's volumes will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runOnTargetPodNode")]
    pub run_on_target_pod_node: Option<bool>,
}

/// Specifies that the action should be executed by a Kubernetes Job.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetBackupPostBackupJobOnError {
    Continue,
    Fail,
}

/// ActionSpec defines an action that should be executed. Only one of the fields may be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPreBackup {
    /// Specifies that the action should be executed using the pod's exec API within a container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ActionSetBackupPreBackupExec>,
    /// Specifies that the action should be executed by a Kubernetes Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<ActionSetBackupPreBackupJob>,
}

/// Specifies that the action should be executed using the pod's exec API within a container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPreBackupExec {
    /// Defines the command and arguments to be executed.
    pub command: Vec<String>,
    /// Specifies the container within the pod where the command should be executed. If not specified, the first container in the pod is used by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetBackupPreBackupExecOnError>,
    /// Specifies the maximum duration to wait for the hook to complete before considering the execution a failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Specifies that the action should be executed using the pod's exec API within a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetBackupPreBackupExecOnError {
    Continue,
    Fail,
}

/// Specifies that the action should be executed by a Kubernetes Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPreBackupJob {
    /// Defines the commands to back up the volume data.
    pub command: Vec<String>,
    /// Specifies the image of the backup container.
    pub image: String,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetBackupPreBackupJobOnError>,
    /// Determines whether to run the job workload on the target pod node. If the backup container needs to mount the target pod's volumes, this field should be set to true. Otherwise, the target pod's volumes will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runOnTargetPodNode")]
    pub run_on_target_pod_node: Option<bool>,
}

/// Specifies that the action should be executed by a Kubernetes Job.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetBackupPreBackupJobOnError {
    Continue,
    Fail,
}

/// Represents a custom deletion action that can be executed before the built-in deletion action. Note: The preDelete action job will ignore the env/envFrom.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetBackupPreDelete {
    /// Defines the commands to back up the volume data.
    pub command: Vec<String>,
    /// Specifies the image of the backup container.
    pub image: String,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ActionSetEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ActionSetEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<ActionSetEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<ActionSetEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ActionSetEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvFrom {
    /// The ConfigMap to select from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapRef")]
    pub config_map_ref: Option<ActionSetEnvFromConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The Secret to select from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ActionSetEnvFromSecretRef>,
}

/// The ConfigMap to select from
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvFromConfigMapRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The Secret to select from
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetEnvFromSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Specifies the restore action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetRestore {
    /// Specifies the actions that should be executed after the data has been prepared and is ready for restoration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postReady")]
    pub post_ready: Option<Vec<ActionSetRestorePostReady>>,
    /// Specifies the action required to prepare data for restoration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepareData")]
    pub prepare_data: Option<ActionSetRestorePrepareData>,
}

/// ActionSpec defines an action that should be executed. Only one of the fields may be set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetRestorePostReady {
    /// Specifies that the action should be executed using the pod's exec API within a container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ActionSetRestorePostReadyExec>,
    /// Specifies that the action should be executed by a Kubernetes Job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<ActionSetRestorePostReadyJob>,
}

/// Specifies that the action should be executed using the pod's exec API within a container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetRestorePostReadyExec {
    /// Defines the command and arguments to be executed.
    pub command: Vec<String>,
    /// Specifies the container within the pod where the command should be executed. If not specified, the first container in the pod is used by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetRestorePostReadyExecOnError>,
    /// Specifies the maximum duration to wait for the hook to complete before considering the execution a failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Specifies that the action should be executed using the pod's exec API within a container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetRestorePostReadyExecOnError {
    Continue,
    Fail,
}

/// Specifies that the action should be executed by a Kubernetes Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetRestorePostReadyJob {
    /// Defines the commands to back up the volume data.
    pub command: Vec<String>,
    /// Specifies the image of the backup container.
    pub image: String,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetRestorePostReadyJobOnError>,
    /// Determines whether to run the job workload on the target pod node. If the backup container needs to mount the target pod's volumes, this field should be set to true. Otherwise, the target pod's volumes will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runOnTargetPodNode")]
    pub run_on_target_pod_node: Option<bool>,
}

/// Specifies that the action should be executed by a Kubernetes Job.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetRestorePostReadyJobOnError {
    Continue,
    Fail,
}

/// Specifies the action required to prepare data for restoration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetRestorePrepareData {
    /// Defines the commands to back up the volume data.
    pub command: Vec<String>,
    /// Specifies the image of the backup container.
    pub image: String,
    /// Indicates how to behave if an error is encountered during the execution of this action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onError")]
    pub on_error: Option<ActionSetRestorePrepareDataOnError>,
    /// Determines whether to run the job workload on the target pod node. If the backup container needs to mount the target pod's volumes, this field should be set to true. Otherwise, the target pod's volumes will be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runOnTargetPodNode")]
    pub run_on_target_pod_node: Option<bool>,
}

/// Specifies the action required to prepare data for restoration.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetRestorePrepareDataOnError {
    Continue,
    Fail,
}

/// ActionSetStatus defines the observed state of ActionSet
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ActionSetStatus {
    /// Provides a human-readable explanation detailing the reason for the current phase of the ActionSet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Represents the generation number that has been observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Indicates the phase of the ActionSet. This can be either 'Available' or 'Unavailable'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ActionSetStatusPhase>,
}

/// ActionSetStatus defines the observed state of ActionSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ActionSetStatusPhase {
    Available,
    Unavailable,
}

