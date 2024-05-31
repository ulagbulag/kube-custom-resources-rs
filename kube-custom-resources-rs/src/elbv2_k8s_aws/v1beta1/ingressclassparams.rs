// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/aws-load-balancer-controller/elbv2.k8s.aws/v1beta1/ingressclassparams.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// IngressClassParamsSpec defines the desired state of IngressClassParams
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "elbv2.k8s.aws", version = "v1beta1", kind = "IngressClassParams", plural = "ingressclassparams")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IngressClassParamsSpec {
    /// CertificateArn specifies the ARN of the certificates for all Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateArn")]
    pub certificate_arn: Option<Vec<String>>,
    /// Group defines the IngressGroup for all Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<IngressClassParamsGroup>,
    /// InboundCIDRs specifies the CIDRs that are allowed to access the Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inboundCIDRs")]
    pub inbound_cid_rs: Option<Vec<String>>,
    /// IPAddressType defines the ip address type for all Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressType")]
    pub ip_address_type: Option<IngressClassParamsIpAddressType>,
    /// LoadBalancerAttributes define the custom attributes to LoadBalancers for all Ingress that that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerAttributes")]
    pub load_balancer_attributes: Option<Vec<IngressClassParamsLoadBalancerAttributes>>,
    /// NamespaceSelector restrict the namespaces of Ingresses that are allowed to specify the IngressClass with this IngressClassParams.
    /// * if absent or present but empty, it selects all namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<IngressClassParamsNamespaceSelector>,
    /// Scheme defines the scheme for all Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<IngressClassParamsScheme>,
    /// SSLPolicy specifies the SSL Policy for all Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslPolicy")]
    pub ssl_policy: Option<String>,
    /// Subnets defines the subnets for all Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<IngressClassParamsSubnets>,
    /// Tags defines list of Tags on AWS resources provisioned for Ingresses that belong to IngressClass with this IngressClassParams.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<IngressClassParamsTags>>,
}

/// Group defines the IngressGroup for all Ingresses that belong to IngressClass with this IngressClassParams.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressClassParamsGroup {
    /// Name is the name of IngressGroup.
    pub name: String,
}

/// IngressClassParamsSpec defines the desired state of IngressClassParams
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IngressClassParamsIpAddressType {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "dualstack")]
    Dualstack,
    #[serde(rename = "dualstack-without-public-ipv4")]
    DualstackWithoutPublicIpv4,
}

/// Attributes defines custom attributes on resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressClassParamsLoadBalancerAttributes {
    /// The key of the attribute.
    pub key: String,
    /// The value of the attribute.
    pub value: String,
}

/// NamespaceSelector restrict the namespaces of Ingresses that are allowed to specify the IngressClass with this IngressClassParams.
/// * if absent or present but empty, it selects all namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressClassParamsNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<IngressClassParamsNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressClassParamsNamespaceSelectorMatchExpressions {
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

/// IngressClassParamsSpec defines the desired state of IngressClassParams
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IngressClassParamsScheme {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "internet-facing")]
    InternetFacing,
}

/// Subnets defines the subnets for all Ingresses that belong to IngressClass with this IngressClassParams.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressClassParamsSubnets {
    /// IDs specify the resource IDs of subnets. Exactly one of this or `tags` must be specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Tags specifies subnets in the load balancer's VPC where each
    /// tag specified in the map key contains one of the values in the corresponding
    /// value list.
    /// Exactly one of this or `ids` must be specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// Tag defines a AWS Tag on resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IngressClassParamsTags {
    /// The key of the tag.
    pub key: String,
    /// The value of the tag.
    pub value: String,
}

