// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1alpha4/machinedeployments.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// MachineDeploymentSpec defines the desired state of MachineDeployment.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.x-k8s.io", version = "v1alpha4", kind = "MachineDeployment", plural = "machinedeployments")]
#[kube(namespaced)]
#[kube(status = "MachineDeploymentStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MachineDeploymentSpec {
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// Minimum number of seconds for which a newly created machine should
    /// be ready.
    /// Defaults to 0 (machine will be considered available as soon as it
    /// is ready)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReadySeconds")]
    pub min_ready_seconds: Option<i32>,
    /// Indicates that the deployment is paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// The maximum time in seconds for a deployment to make progress before it
    /// is considered to be failed. The deployment controller will continue to
    /// process failed deployments and a condition with a ProgressDeadlineExceeded
    /// reason will be surfaced in the deployment status. Note that progress will
    /// not be estimated during the time a deployment is paused. Defaults to 600s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "progressDeadlineSeconds")]
    pub progress_deadline_seconds: Option<i32>,
    /// Number of desired machines. Defaults to 1.
    /// This is a pointer to distinguish between explicit zero and not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The number of old MachineSets to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i32>,
    /// Label selector for machines. Existing MachineSets whose machines are
    /// selected by this will be the ones affected by this deployment.
    /// It must match the machine template's labels.
    pub selector: MachineDeploymentSelector,
    /// The deployment strategy to use to replace existing machines with
    /// new ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<MachineDeploymentStrategy>,
    /// template describes the machines that will be created.
    pub template: MachineDeploymentTemplate,
}

/// Label selector for machines. Existing MachineSets whose machines are
/// selected by this will be the ones affected by this deployment.
/// It must match the machine template's labels.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<MachineDeploymentSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// The deployment strategy to use to replace existing machines with
/// new ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStrategy {
    /// Rolling update config params. Present only if
    /// MachineDeploymentStrategyType = RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<MachineDeploymentStrategyRollingUpdate>,
    /// type of deployment.
    /// Default is RollingUpdate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<MachineDeploymentStrategyType>,
}

/// Rolling update config params. Present only if
/// MachineDeploymentStrategyType = RollingUpdate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStrategyRollingUpdate {
    /// deletePolicy defines the policy used by the MachineDeployment to identify nodes to delete when downscaling.
    /// Valid values are "Random, "Newest", "Oldest"
    /// When no value is supplied, the default DeletePolicy of MachineSet is used
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deletePolicy")]
    pub delete_policy: Option<MachineDeploymentStrategyRollingUpdateDeletePolicy>,
    /// The maximum number of machines that can be scheduled above the
    /// desired number of machines.
    /// Value can be an absolute number (ex: 5) or a percentage of
    /// desired machines (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up.
    /// Defaults to 1.
    /// Example: when this is set to 30%, the new MachineSet can be scaled
    /// up immediately when the rolling update starts, such that the total
    /// number of old and new machines do not exceed 130% of desired
    /// machines. Once old machines have been killed, new MachineSet can
    /// be scaled up further, ensuring that total number of machines running
    /// at any time during the update is at most 130% of desired machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSurge")]
    pub max_surge: Option<IntOrString>,
    /// The maximum number of machines that can be unavailable during the update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired
    /// machines (ex: 10%).
    /// Absolute number is calculated from percentage by rounding down.
    /// This can not be 0 if MaxSurge is 0.
    /// Defaults to 0.
    /// Example: when this is set to 30%, the old MachineSet can be scaled
    /// down to 70% of desired machines immediately when the rolling update
    /// starts. Once new machines are ready, old MachineSet can be scaled
    /// down further, followed by scaling up the new MachineSet, ensuring
    /// that the total number of machines available at all times
    /// during the update is at least 70% of desired machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
}

/// Rolling update config params. Present only if
/// MachineDeploymentStrategyType = RollingUpdate.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineDeploymentStrategyRollingUpdateDeletePolicy {
    Random,
    Newest,
    Oldest,
}

/// The deployment strategy to use to replace existing machines with
/// new ones.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachineDeploymentStrategyType {
    RollingUpdate,
    OnDelete,
}

/// template describes the machines that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplate {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MachineDeploymentTemplateMetadata>,
    /// Specification of the desired behavior of the machine.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MachineDeploymentTemplateSpec>,
}

/// Standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateMetadata {
    /// annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Specification of the desired behavior of the machine.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpec {
    /// bootstrap is a reference to a local struct which encapsulates
    /// fields to configure the Machine’s bootstrapping mechanism.
    pub bootstrap: MachineDeploymentTemplateSpecBootstrap,
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// failureDomain is the failure domain the machine will be created in.
    /// Must match a key in the FailureDomains map stored on the cluster object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomain")]
    pub failure_domain: Option<String>,
    /// infrastructureRef is a required reference to a custom resource
    /// offered by an infrastructure provider.
    #[serde(rename = "infrastructureRef")]
    pub infrastructure_ref: ObjectReference,
    /// nodeDrainTimeout is the total amount of time that the controller will spend on draining a node.
    /// The default value is 0, meaning that the node can be drained without any time limitations.
    /// NOTE: NodeDrainTimeout is different from `kubectl drain --timeout`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDrainTimeout")]
    pub node_drain_timeout: Option<String>,
    /// providerID is the identification ID of the machine provided by the provider.
    /// This field must match the provider ID as seen on the node object corresponding to this machine.
    /// This field is required by higher level consumers of cluster-api. Example use case is cluster autoscaler
    /// with cluster-api as provider. Clean-up logic in the autoscaler compares machines to nodes to find out
    /// machines at provider which could not get registered as Kubernetes nodes. With cluster-api as a
    /// generic out-of-tree provider for autoscaler, this field is required by autoscaler to be
    /// able to have a provider view of the list of machines. Another list of nodes is queried from the k8s apiserver
    /// and then a comparison is done to find out unregistered machines and are marked for delete.
    /// This field will be set by the actuators and consumed by higher level entities like autoscaler that will
    /// be interfacing with cluster-api as generic provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// version defines the desired Kubernetes version.
    /// This field is meant to be optionally used by bootstrap providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// bootstrap is a reference to a local struct which encapsulates
/// fields to configure the Machine’s bootstrapping mechanism.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecBootstrap {
    /// configRef is a reference to a bootstrap provider-specific resource
    /// that holds configuration details. The reference is optional to
    /// allow users/operators to specify Bootstrap.DataSecretName without
    /// the need of a controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<ObjectReference>,
    /// dataSecretName is the name of the secret that stores the bootstrap data script.
    /// If nil, the Machine should remain in the Pending state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSecretName")]
    pub data_secret_name: Option<String>,
}

/// configRef is a reference to a bootstrap provider-specific resource
/// that holds configuration details. The reference is optional to
/// allow users/operators to specify Bootstrap.DataSecretName without
/// the need of a controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecBootstrapConfigRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// infrastructureRef is a required reference to a custom resource
/// offered by an infrastructure provider.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentTemplateSpecInfrastructureRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// MachineDeploymentStatus defines the observed state of MachineDeployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeploymentStatus {
    /// Total number of available machines (ready for at least minReadySeconds)
    /// targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// conditions defines current service state of the MachineDeployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation observed by the deployment controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// phase represents the current phase of a MachineDeployment (ScalingUp, ScalingDown, Running, Failed, or Unknown).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Total number of ready machines targeted by this deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated machines targeted by this deployment
    /// (their labels match the selector).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// selector is the same as the label selector but in the string format to avoid introspection
    /// by clients. The string will be in the same format as the query-param syntax.
    /// More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Total number of unavailable machines targeted by this deployment.
    /// This is the total number of machines that are still required for
    /// the deployment to have 100% available capacity. They may either
    /// be machines that are running but not yet available or machines
    /// that still have not been created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableReplicas")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated machines targeted by this deployment
    /// that have the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReplicas")]
    pub updated_replicas: Option<i32>,
}

