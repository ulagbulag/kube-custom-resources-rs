// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api/cluster.x-k8s.io/v1beta1/machinepools.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// MachinePoolSpec defines the desired state of MachinePool.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.x-k8s.io", version = "v1beta1", kind = "MachinePool", plural = "machinepools")]
#[kube(namespaced)]
#[kube(status = "MachinePoolStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MachinePoolSpec {
    /// clusterName is the name of the Cluster this object belongs to.
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// failureDomains is the list of failure domains this MachinePool should be attached to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomains")]
    pub failure_domains: Option<Vec<String>>,
    /// Minimum number of seconds for which a newly created machine instances should
    /// be ready.
    /// Defaults to 0 (machine instance will be considered available as soon as it
    /// is ready)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReadySeconds")]
    pub min_ready_seconds: Option<i32>,
    /// providerIDList are the identification IDs of machine instances provided by the provider.
    /// This field must match the provider IDs as seen on the node objects corresponding to a machine pool's machine instances.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerIDList")]
    pub provider_id_list: Option<Vec<String>>,
    /// Number of desired machines. Defaults to 1.
    /// This is a pointer to distinguish between explicit zero and not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// template describes the machines that will be created.
    pub template: MachinePoolTemplate,
}

/// template describes the machines that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolTemplate {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MachinePoolTemplateMetadata>,
    /// Specification of the desired behavior of the machine.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MachinePoolTemplateSpec>,
}

/// Standard object's metadata.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolTemplateMetadata {
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
pub struct MachinePoolTemplateSpec {
    /// bootstrap is a reference to a local struct which encapsulates
    /// fields to configure the Machine’s bootstrapping mechanism.
    pub bootstrap: MachinePoolTemplateSpecBootstrap,
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
    /// nodeDeletionTimeout defines how long the controller will attempt to delete the Node that the Machine
    /// hosts after the Machine is marked for deletion. A duration of 0 will retry deletion indefinitely.
    /// Defaults to 10 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDeletionTimeout")]
    pub node_deletion_timeout: Option<String>,
    /// nodeDrainTimeout is the total amount of time that the controller will spend on draining a node.
    /// The default value is 0, meaning that the node can be drained without any time limitations.
    /// NOTE: NodeDrainTimeout is different from `kubectl drain --timeout`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDrainTimeout")]
    pub node_drain_timeout: Option<String>,
    /// nodeVolumeDetachTimeout is the total amount of time that the controller will spend on waiting for all volumes
    /// to be detached. The default value is 0, meaning that the volumes can be detached without any time limitations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeVolumeDetachTimeout")]
    pub node_volume_detach_timeout: Option<String>,
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
    /// readinessGates specifies additional conditions to include when evaluating Machine Ready condition.
    /// 
    /// This field can be used e.g. by Cluster API control plane providers to extend the semantic of the
    /// Ready condition for the Machine they control, like the kubeadm control provider adding ReadinessGates
    /// for the APIServerPodHealthy, SchedulerPodHealthy conditions, etc.
    /// 
    /// Another example are external controllers, e.g. responsible to install special software/hardware on the Machines;
    /// they can include the status of those components with a new condition and add this condition to ReadinessGates.
    /// 
    /// NOTE: this field is considered only for computing v1beta2 conditions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readinessGates")]
    pub readiness_gates: Option<Vec<MachinePoolTemplateSpecReadinessGates>>,
    /// version defines the desired Kubernetes version.
    /// This field is meant to be optionally used by bootstrap providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// bootstrap is a reference to a local struct which encapsulates
/// fields to configure the Machine’s bootstrapping mechanism.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolTemplateSpecBootstrap {
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
pub struct MachinePoolTemplateSpecBootstrapConfigRef {
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
pub struct MachinePoolTemplateSpecInfrastructureRef {
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

/// MachineReadinessGate contains the type of a Machine condition to be used as a readiness gate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolTemplateSpecReadinessGates {
    /// conditionType refers to a positive polarity condition (status true means good) with matching type in the Machine's condition list.
    /// If the conditions doesn't exist, it will be treated as unknown.
    /// Note: Both Cluster API conditions or conditions added by 3rd party controllers can be used as readiness gates.
    #[serde(rename = "conditionType")]
    pub condition_type: String,
}

/// MachinePoolStatus defines the observed state of MachinePool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolStatus {
    /// The number of available replicas (ready for at least minReadySeconds) for this MachinePool.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// bootstrapReady is the state of the bootstrap provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootstrapReady")]
    pub bootstrap_ready: Option<bool>,
    /// conditions define the current service state of the MachinePool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// failureMessage indicates that there is a problem reconciling the state,
    /// and will be set to a descriptive error message.
    /// 
    /// Deprecated: This field is deprecated and is going to be removed in the next apiVersion. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// failureReason indicates that there is a problem reconciling the state, and
    /// will be set to a token value suitable for programmatic interpretation.
    /// 
    /// Deprecated: This field is deprecated and is going to be removed in the next apiVersion. Please see https://github.com/kubernetes-sigs/cluster-api/blob/main/docs/proposals/20240916-improve-status-in-CAPI-resources.md for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// infrastructureReady is the state of the infrastructure provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infrastructureReady")]
    pub infrastructure_ready: Option<bool>,
    /// nodeRefs will point to the corresponding Nodes if it they exist.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeRefs")]
    pub node_refs: Option<Vec<ObjectReference>>,
    /// observedGeneration is the latest generation observed by the controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// phase represents the current phase of cluster actuation.
    /// E.g. Pending, Running, Terminating, Failed etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The number of ready replicas for this MachinePool. A machine is considered ready when the node has been created and is "Ready".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// replicas is the most recently observed number of replicas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Total number of unavailable machine instances targeted by this machine pool.
    /// This is the total number of machine instances that are still required for
    /// the machine pool to have 100% available capacity. They may either
    /// be machine instances that are running but not yet available or machine instances
    /// that still have not been created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableReplicas")]
    pub unavailable_replicas: Option<i32>,
    /// v1beta2 groups all the fields that will be added or modified in MachinePool's status with the V1Beta2 version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v1beta2: Option<MachinePoolStatusV1beta2>,
}

/// v1beta2 groups all the fields that will be added or modified in MachinePool's status with the V1Beta2 version.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolStatusV1beta2 {
    /// availableReplicas is the number of available replicas for this MachinePool. A machine is considered available when Machine's Available condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// conditions represents the observations of a MachinePool's current state.
    /// Known condition types are Available, BootstrapConfigReady, InfrastructureReady, MachinesReady, MachinesUpToDate,
    /// ScalingUp, ScalingDown, Remediating, Deleting, Paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// readyReplicas is the number of ready replicas for this MachinePool. A machine is considered ready when Machine's Ready condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// upToDateReplicas is the number of up-to-date replicas targeted by this MachinePool. A machine is considered up-to-date when Machine's UpToDate condition is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upToDateReplicas")]
    pub up_to_date_replicas: Option<i32>,
}

