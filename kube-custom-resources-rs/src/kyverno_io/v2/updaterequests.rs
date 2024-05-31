// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kyverno/kyverno/kyverno.io/v2/updaterequests.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ResourceSpec is the information to identify the trigger resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kyverno.io", version = "v2", kind = "UpdateRequest", plural = "updaterequests")]
#[kube(namespaced)]
#[kube(status = "UpdateRequestStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct UpdateRequestSpec {
    /// Context ...
    pub context: UpdateRequestContext,
    /// DeleteDownstream represents whether the downstream needs to be deleted.
    #[serde(rename = "deleteDownstream")]
    pub delete_downstream: bool,
    /// Specifies the name of the policy.
    pub policy: String,
    /// Type represents request type for background processing
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestType")]
    pub request_type: Option<UpdateRequestRequestType>,
    /// ResourceSpec is the information to identify the trigger resource.
    pub resource: UpdateRequestResource,
    /// Rule is the associate rule name of the current UR.
    pub rule: String,
    /// Synchronize represents the sync behavior of the corresponding rule
    /// Optional. Defaults to "false" if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synchronize: Option<bool>,
}

/// Context ...
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContext {
    /// AdmissionRequestInfoObject stores the admission request and operation details
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admissionRequestInfo")]
    pub admission_request_info: Option<UpdateRequestContextAdmissionRequestInfo>,
    /// RequestInfo contains permission info carried in an admission request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userInfo")]
    pub user_info: Option<UpdateRequestContextUserInfo>,
}

/// AdmissionRequestInfoObject stores the admission request and operation details
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfo {
    /// AdmissionRequest describes the admission.Attributes for the admission request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admissionRequest")]
    pub admission_request: Option<UpdateRequestContextAdmissionRequestInfoAdmissionRequest>,
    /// Operation is the type of resource operation being checked for admission control
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

/// AdmissionRequest describes the admission.Attributes for the admission request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfoAdmissionRequest {
    /// DryRun indicates that modifications will definitely not be persisted for this request.
    /// Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dryRun")]
    pub dry_run: Option<bool>,
    /// Kind is the fully-qualified type of object being submitted (for example, v1.Pod or autoscaling.v1.Scale)
    pub kind: UpdateRequestContextAdmissionRequestInfoAdmissionRequestKind,
    /// Name is the name of the object as presented in the request.  On a CREATE operation, the client may omit name and
    /// rely on the server to generate the name.  If that is the case, this field will contain an empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the namespace associated with the request (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Object is the object from the incoming request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<BTreeMap<String, serde_json::Value>>,
    /// OldObject is the existing object. Only populated for DELETE and UPDATE requests.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oldObject")]
    pub old_object: Option<BTreeMap<String, serde_json::Value>>,
    /// Operation is the operation being performed. This may be different than the operation
    /// requested. e.g. a patch can result in either a CREATE or UPDATE Operation.
    pub operation: String,
    /// Options is the operation option structure of the operation being performed.
    /// e.g. `meta.k8s.io/v1.DeleteOptions` or `meta.k8s.io/v1.CreateOptions`. This may be
    /// different than the options the caller provided. e.g. for a patch request the performed
    /// Operation might be a CREATE, in which case the Options will a
    /// `meta.k8s.io/v1.CreateOptions` even though the caller provided `meta.k8s.io/v1.PatchOptions`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, serde_json::Value>>,
    /// RequestKind is the fully-qualified type of the original API request (for example, v1.Pod or autoscaling.v1.Scale).
    /// If this is specified and differs from the value in "kind", an equivalent match and conversion was performed.
    /// 
    /// 
    /// For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
    /// `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
    /// an API request to apps/v1beta1 deployments would be converted and sent to the webhook
    /// with `kind: {group:"apps", version:"v1", kind:"Deployment"}` (matching the rule the webhook registered for),
    /// and `requestKind: {group:"apps", version:"v1beta1", kind:"Deployment"}` (indicating the kind of the original API request).
    /// 
    /// 
    /// See documentation for the "matchPolicy" field in the webhook configuration type for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestKind")]
    pub request_kind: Option<UpdateRequestContextAdmissionRequestInfoAdmissionRequestRequestKind>,
    /// RequestResource is the fully-qualified resource of the original API request (for example, v1.pods).
    /// If this is specified and differs from the value in "resource", an equivalent match and conversion was performed.
    /// 
    /// 
    /// For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
    /// `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
    /// an API request to apps/v1beta1 deployments would be converted and sent to the webhook
    /// with `resource: {group:"apps", version:"v1", resource:"deployments"}` (matching the resource the webhook registered for),
    /// and `requestResource: {group:"apps", version:"v1beta1", resource:"deployments"}` (indicating the resource of the original API request).
    /// 
    /// 
    /// See documentation for the "matchPolicy" field in the webhook configuration type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestResource")]
    pub request_resource: Option<UpdateRequestContextAdmissionRequestInfoAdmissionRequestRequestResource>,
    /// RequestSubResource is the name of the subresource of the original API request, if any (for example, "status" or "scale")
    /// If this is specified and differs from the value in "subResource", an equivalent match and conversion was performed.
    /// See documentation for the "matchPolicy" field in the webhook configuration type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestSubResource")]
    pub request_sub_resource: Option<String>,
    /// Resource is the fully-qualified resource being requested (for example, v1.pods)
    pub resource: UpdateRequestContextAdmissionRequestInfoAdmissionRequestResource,
    /// SubResource is the subresource being requested, if any (for example, "status" or "scale")
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subResource")]
    pub sub_resource: Option<String>,
    /// UID is an identifier for the individual request/response. It allows us to distinguish instances of requests which are
    /// otherwise identical (parallel requests, requests when earlier requests did not modify etc)
    /// The UID is meant to track the round trip (request/response) between the KAS and the WebHook, not the user request.
    /// It is suitable for correlating log entries between the webhook and apiserver, for either auditing or debugging.
    pub uid: String,
    /// UserInfo is information about the requesting user
    #[serde(rename = "userInfo")]
    pub user_info: UpdateRequestContextAdmissionRequestInfoAdmissionRequestUserInfo,
}

/// Kind is the fully-qualified type of object being submitted (for example, v1.Pod or autoscaling.v1.Scale)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfoAdmissionRequestKind {
    pub group: String,
    pub kind: String,
    pub version: String,
}

/// RequestKind is the fully-qualified type of the original API request (for example, v1.Pod or autoscaling.v1.Scale).
/// If this is specified and differs from the value in "kind", an equivalent match and conversion was performed.
/// 
/// 
/// For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
/// `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
/// an API request to apps/v1beta1 deployments would be converted and sent to the webhook
/// with `kind: {group:"apps", version:"v1", kind:"Deployment"}` (matching the rule the webhook registered for),
/// and `requestKind: {group:"apps", version:"v1beta1", kind:"Deployment"}` (indicating the kind of the original API request).
/// 
/// 
/// See documentation for the "matchPolicy" field in the webhook configuration type for more details.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfoAdmissionRequestRequestKind {
    pub group: String,
    pub kind: String,
    pub version: String,
}

/// RequestResource is the fully-qualified resource of the original API request (for example, v1.pods).
/// If this is specified and differs from the value in "resource", an equivalent match and conversion was performed.
/// 
/// 
/// For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
/// `apiGroups:["apps"], apiVersions:["v1"], resources: ["deployments"]` and `matchPolicy: Equivalent`,
/// an API request to apps/v1beta1 deployments would be converted and sent to the webhook
/// with `resource: {group:"apps", version:"v1", resource:"deployments"}` (matching the resource the webhook registered for),
/// and `requestResource: {group:"apps", version:"v1beta1", resource:"deployments"}` (indicating the resource of the original API request).
/// 
/// 
/// See documentation for the "matchPolicy" field in the webhook configuration type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfoAdmissionRequestRequestResource {
    pub group: String,
    pub resource: String,
    pub version: String,
}

/// Resource is the fully-qualified resource being requested (for example, v1.pods)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfoAdmissionRequestResource {
    pub group: String,
    pub resource: String,
    pub version: String,
}

/// UserInfo is information about the requesting user
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextAdmissionRequestInfoAdmissionRequestUserInfo {
    /// Any additional information provided by the authenticator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<BTreeMap<String, String>>,
    /// The names of groups this user is a part of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// A unique value that identifies this user across time. If this user is
    /// deleted and another user by the same name is added, they will have
    /// different UIDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// The name that uniquely identifies this user among all active users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// RequestInfo contains permission info carried in an admission request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextUserInfo {
    /// ClusterRoles is a list of possible clusterRoles send the request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterRoles")]
    pub cluster_roles: Option<Vec<String>>,
    /// Roles is a list of possible role send the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// UserInfo is the userInfo carried in the admission request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userInfo")]
    pub user_info: Option<UpdateRequestContextUserInfoUserInfo>,
}

/// UserInfo is the userInfo carried in the admission request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestContextUserInfoUserInfo {
    /// Any additional information provided by the authenticator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<BTreeMap<String, String>>,
    /// The names of groups this user is a part of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// A unique value that identifies this user across time. If this user is
    /// deleted and another user by the same name is added, they will have
    /// different UIDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// The name that uniquely identifies this user among all active users.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// ResourceSpec is the information to identify the trigger resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UpdateRequestRequestType {
    #[serde(rename = "mutate")]
    Mutate,
    #[serde(rename = "generate")]
    Generate,
}

/// ResourceSpec is the information to identify the trigger resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestResource {
    /// APIVersion specifies resource apiVersion.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind specifies resource kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name specifies the resource name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies resource namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// UID specifies the resource uid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Status contains statistics related to update request.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestStatus {
    /// This will track the resources that are updated by the generate Policy.
    /// Will be used during clean up resources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatedResources")]
    pub generated_resources: Option<Vec<UpdateRequestStatusGeneratedResources>>,
    /// Specifies request status message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryCount")]
    pub retry_count: Option<i64>,
    /// State represents state of the update request.
    pub state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UpdateRequestStatusGeneratedResources {
    /// APIVersion specifies resource apiVersion.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind specifies resource kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name specifies the resource name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies resource namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// UID specifies the resource uid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

