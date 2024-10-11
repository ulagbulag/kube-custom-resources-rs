// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/ec2-controller/ec2.services.k8s.aws/v1alpha1/transitgateways.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// TransitGatewaySpec defines the desired state of TransitGateway.
/// 
/// Describes a transit gateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ec2.services.k8s.aws", version = "v1alpha1", kind = "TransitGateway", plural = "transitgateways")]
#[kube(namespaced)]
#[kube(status = "TransitGatewayStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TransitGatewaySpec {
    /// A description of the transit gateway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The transit gateway options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<TransitGatewayOptions>,
    /// The tags. The value parameter is required, but if you don't want the tag
    /// to have a value, specify the parameter with no value, and we set the value
    /// to an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TransitGatewayTags>>,
}

/// The transit gateway options.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransitGatewayOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amazonSideASN")]
    pub amazon_side_asn: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoAcceptSharedAttachments")]
    pub auto_accept_shared_attachments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultRouteTableAssociation")]
    pub default_route_table_association: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultRouteTablePropagation")]
    pub default_route_table_propagation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsSupport")]
    pub dns_support: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multicastSupport")]
    pub multicast_support: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transitGatewayCIDRBlocks")]
    pub transit_gateway_cidr_blocks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpnECMPSupport")]
    pub vpn_ecmp_support: Option<String>,
}

/// Describes a tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransitGatewayTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TransitGatewayStatus defines the observed state of TransitGateway
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransitGatewayStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<TransitGatewayStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The creation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// The ID of the Amazon Web Services account that owns the transit gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    /// The state of the transit gateway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the transit gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transitGatewayID")]
    pub transit_gateway_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TransitGatewayStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
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

