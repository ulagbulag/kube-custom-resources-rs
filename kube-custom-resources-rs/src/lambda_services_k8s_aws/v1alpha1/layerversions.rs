// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/lambda-controller/lambda.services.k8s.aws/v1alpha1/layerversions.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// LayerVersionSpec defines the desired state of LayerVersion.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "lambda.services.k8s.aws", version = "v1alpha1", kind = "LayerVersion", plural = "layerversions")]
#[kube(namespaced)]
#[kube(status = "LayerVersionStatus")]
#[kube(schema = "disabled")]
pub struct LayerVersionSpec {
    /// A list of compatible instruction set architectures (https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compatibleArchitectures")]
    pub compatible_architectures: Option<Vec<String>>,
    /// A list of compatible function runtimes (https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html).
    /// Used for filtering with ListLayers and ListLayerVersions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compatibleRuntimes")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// The function layer archive.
    pub content: LayerVersionContent,
    /// The description of the version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name or Amazon Resource Name (ARN) of the layer.
    #[serde(rename = "layerName")]
    pub layer_name: String,
    /// The layer's software license. It can be any of the following:
    /// 
    /// 
    ///    * An SPDX license identifier (https://spdx.org/licenses/). For example,
    ///    MIT.
    /// 
    /// 
    ///    * The URL of a license hosted on the internet. For example, https://opensource.org/licenses/MIT.
    /// 
    /// 
    ///    * The full text of the license.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "licenseInfo")]
    pub license_info: Option<String>,
}

/// The function layer archive.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LayerVersionContent {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Bucket")]
    pub s3_bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Key")]
    pub s3_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3ObjectVersion")]
    pub s3_object_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zipFile")]
    pub zip_file: Option<String>,
}

/// LayerVersionStatus defines the observed state of LayerVersion
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LayerVersionStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<LayerVersionStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<LayerVersionStatusConditions>>,
    /// The date that the layer version was created, in ISO-8601 format (https://www.w3.org/TR/NOTE-datetime)
    /// (YYYY-MM-DDThh:mm:ss.sTZD).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdDate")]
    pub created_date: Option<String>,
    /// The ARN of the layer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "layerARN")]
    pub layer_arn: Option<String>,
    /// The version number.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionNumber")]
    pub version_number: Option<i64>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LayerVersionStatusAckResourceMetadata {
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

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LayerVersionStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type is the type of the Condition
    #[serde(rename = "type")]
    pub r#type: String,
}

