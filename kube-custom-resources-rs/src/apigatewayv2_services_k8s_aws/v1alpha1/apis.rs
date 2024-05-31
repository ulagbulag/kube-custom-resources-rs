// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/apigatewayv2-controller/apigatewayv2.services.k8s.aws/v1alpha1/apis.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ApiSpec defines the desired state of Api.
/// 
/// 
/// Represents an API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apigatewayv2.services.k8s.aws", version = "v1alpha1", kind = "API", plural = "apis")]
#[kube(namespaced)]
#[kube(status = "APIStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct APISpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiKeySelectionExpression")]
    pub api_key_selection_expression: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basepath: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Represents a CORS configuration. Supported only for HTTP APIs. See Configuring
    /// CORS (https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html)
    /// for more information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "corsConfiguration")]
    pub cors_configuration: Option<APICorsConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsARN")]
    pub credentials_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableExecuteAPIEndpoint")]
    pub disable_execute_api_endpoint: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableSchemaValidation")]
    pub disable_schema_validation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failOnWarnings")]
    pub fail_on_warnings: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolType")]
    pub protocol_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeKey")]
    pub route_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeSelectionExpression")]
    pub route_selection_expression: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Represents a CORS configuration. Supported only for HTTP APIs. See Configuring
/// CORS (https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-cors.html)
/// for more information.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APICorsConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCredentials")]
    pub allow_credentials: Option<bool>,
    /// Represents a collection of allowed headers. Supported only for HTTP APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowHeaders")]
    pub allow_headers: Option<Vec<String>>,
    /// Represents a collection of methods. Supported only for HTTP APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowMethods")]
    pub allow_methods: Option<Vec<String>>,
    /// Represents a collection of origins. Supported only for HTTP APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigins")]
    pub allow_origins: Option<Vec<String>>,
    /// Represents a collection of allowed headers. Supported only for HTTP APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposeHeaders")]
    pub expose_headers: Option<Vec<String>>,
    /// An integer with a value between -1 and 86400. Supported only for HTTP APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<i64>,
}

/// APIStatus defines the observed state of API
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<APIStatusAckResourceMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiEndpoint")]
    pub api_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGatewayManaged")]
    pub api_gateway_managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiID")]
    pub api_id: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdDate")]
    pub created_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "importInfo")]
    pub import_info: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIStatusAckResourceMetadata {
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

