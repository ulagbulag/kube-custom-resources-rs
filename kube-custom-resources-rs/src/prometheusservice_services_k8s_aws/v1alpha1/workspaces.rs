// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/prometheusservice-controller/prometheusservice.services.k8s.aws/v1alpha1/workspaces.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// WorkspaceSpec defines the desired state of Workspace.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "prometheusservice.services.k8s.aws", version = "v1alpha1", kind = "Workspace", plural = "workspaces")]
#[kube(namespaced)]
#[kube(status = "WorkspaceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct WorkspaceSpec {
    /// An optional user-assigned alias for this workspace. This alias is for user
    /// reference and does not need to be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Optional, user-provided tags for this workspace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// WorkspaceStatus defines the observed state of Workspace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkspaceStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<WorkspaceStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The status of the workspace that was just created (usually CREATING).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkspaceStatusStatus>,
    /// The generated ID of the workspace that was just created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workspaceID")]
    pub workspace_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkspaceStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// The status of the workspace that was just created (usually CREATING).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WorkspaceStatusStatus {
    /// State of a workspace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<String>,
}

