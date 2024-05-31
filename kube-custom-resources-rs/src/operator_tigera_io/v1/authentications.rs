// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1/authentications.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// AuthenticationSpec defines the desired state of Authentication
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "Authentication", plural = "authentications")]
#[kube(status = "AuthenticationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct AuthenticationSpec {
    /// DexDeployment configures the Dex Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dexDeployment")]
    pub dex_deployment: Option<AuthenticationDexDeployment>,
    /// If specified, GroupsPrefix is prepended to each group obtained from the identity provider. Note that
    /// Kibana does not support a groups prefix, so this prefix is removed from Kubernetes Groups when translating log access
    /// ClusterRoleBindings into Elastic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupsPrefix")]
    pub groups_prefix: Option<String>,
    /// LDAP contains the configuration needed to setup LDAP authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap: Option<AuthenticationLdap>,
    /// ManagerDomain is the domain name of the Manager
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerDomain")]
    pub manager_domain: Option<String>,
    /// OIDC contains the configuration needed to setup OIDC authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oidc: Option<AuthenticationOidc>,
    /// Openshift contains the configuration needed to setup Openshift OAuth authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openshift: Option<AuthenticationOpenshift>,
    /// If specified, UsernamePrefix is prepended to each user obtained from the identity provider. Note that
    /// Kibana does not support a user prefix, so this prefix is removed from Kubernetes User when translating log access
    /// ClusterRoleBindings into Elastic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usernamePrefix")]
    pub username_prefix: Option<String>,
}

/// DexDeployment configures the Dex Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeployment {
    /// Spec is the specification of the Dex Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<AuthenticationDexDeploymentSpec>,
}

/// Spec is the specification of the Dex Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpec {
    /// Template describes the Dex Deployment pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<AuthenticationDexDeploymentSpecTemplate>,
}

/// Template describes the Dex Deployment pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplate {
    /// Spec is the Dex Deployment's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<AuthenticationDexDeploymentSpecTemplateSpec>,
}

/// Spec is the Dex Deployment's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpec {
    /// Containers is a list of Dex containers.
    /// If specified, this overrides the specified Dex Deployment containers.
    /// If omitted, the Dex Deployment will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AuthenticationDexDeploymentSpecTemplateSpecContainers>>,
    /// InitContainers is a list of Dex init containers.
    /// If specified, this overrides the specified Dex Deployment init containers.
    /// If omitted, the Dex Deployment will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<AuthenticationDexDeploymentSpecTemplateSpecInitContainers>>,
}

/// DexDeploymentContainer is a Dex Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpecContainers {
    /// Name is an enum which identifies the Dex Deployment container by name.
    /// Supported values are: tigera-dex
    pub name: AuthenticationDexDeploymentSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named Dex Deployment container's resources.
    /// If omitted, the Dex Deployment will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AuthenticationDexDeploymentSpecTemplateSpecContainersResources>,
}

/// DexDeploymentContainer is a Dex Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthenticationDexDeploymentSpecTemplateSpecContainersName {
    #[serde(rename = "tigera-dex")]
    TigeraDex,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named Dex Deployment container's resources.
/// If omitted, the Dex Deployment will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<AuthenticationDexDeploymentSpecTemplateSpecContainersResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpecContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// DexDeploymentInitContainer is a Dex Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the Dex Deployment init container by name.
    /// Supported values are: tigera-dex-tls-key-cert-provisioner
    pub name: AuthenticationDexDeploymentSpecTemplateSpecInitContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named Dex Deployment init container's resources.
    /// If omitted, the Dex Deployment will use its default value for this init container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<AuthenticationDexDeploymentSpecTemplateSpecInitContainersResources>,
}

/// DexDeploymentInitContainer is a Dex Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthenticationDexDeploymentSpecTemplateSpecInitContainersName {
    #[serde(rename = "tigera-dex-tls-key-cert-provisioner")]
    TigeraDexTlsKeyCertProvisioner,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named Dex Deployment init container's resources.
/// If omitted, the Dex Deployment will use its default value for this init container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<AuthenticationDexDeploymentSpecTemplateSpecInitContainersResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationDexDeploymentSpecTemplateSpecInitContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// LDAP contains the configuration needed to setup LDAP authentication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationLdap {
    /// Group search configuration to find the groups that a user is in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupSearch")]
    pub group_search: Option<AuthenticationLdapGroupSearch>,
    /// The host and port of the LDAP server. Example: ad.example.com:636
    pub host: String,
    /// StartTLS whether to enable the startTLS feature for establishing TLS on an existing LDAP session.
    /// If true, the ldap:// protocol is used and then issues a StartTLS command, otherwise, connections will use
    /// the ldaps:// protocol.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTLS")]
    pub start_tls: Option<bool>,
    /// User entry search configuration to match the credentials with a user.
    #[serde(rename = "userSearch")]
    pub user_search: AuthenticationLdapUserSearch,
}

/// Group search configuration to find the groups that a user is in.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationLdapGroupSearch {
    /// BaseDN to start the search from. For example "cn=groups,dc=example,dc=com"
    #[serde(rename = "baseDN")]
    pub base_dn: String,
    /// Optional filter to apply when searching the directory.
    /// For example "(objectClass=posixGroup)"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// The attribute of the group that represents its name. This attribute can be used to apply RBAC to a user group.
    #[serde(rename = "nameAttribute")]
    pub name_attribute: String,
    /// Following list contains field pairs that are used to match a user to a group. It adds an additional
    /// requirement to the filter that an attribute in the group must match the user's
    /// attribute value.
    #[serde(rename = "userMatchers")]
    pub user_matchers: Vec<AuthenticationLdapGroupSearchUserMatchers>,
}

/// UserMatch when the value of a UserAttribute and a GroupAttribute match, a user belongs to the group.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationLdapGroupSearchUserMatchers {
    /// The attribute of a group that links it to a user.
    #[serde(rename = "groupAttribute")]
    pub group_attribute: String,
    /// The attribute of a user that links it to a group.
    #[serde(rename = "userAttribute")]
    pub user_attribute: String,
}

/// User entry search configuration to match the credentials with a user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationLdapUserSearch {
    /// BaseDN to start the search from. For example "cn=users,dc=example,dc=com"
    #[serde(rename = "baseDN")]
    pub base_dn: String,
    /// Optional filter to apply when searching the directory. For example "(objectClass=person)"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// A mapping of the attribute that is used as the username. This attribute can be used to apply RBAC to a user.
    /// Default: uid
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameAttribute")]
    pub name_attribute: Option<String>,
}

/// OIDC contains the configuration needed to setup OIDC authentication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationOidc {
    /// Some providers do not include the claim "email_verified" when there is no verification in the user enrollment
    /// process or if they are acting as a proxy for another identity provider. By default those tokens are deemed invalid.
    /// To skip this check, set the value to "InsecureSkip".
    /// Default: Verify
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emailVerification")]
    pub email_verification: Option<AuthenticationOidcEmailVerification>,
    /// GroupsClaim specifies which claim to use from the OIDC provider as the group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupsClaim")]
    pub groups_claim: Option<String>,
    /// Deprecated. Please use Authentication.Spec.GroupsPrefix instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupsPrefix")]
    pub groups_prefix: Option<String>,
    /// IssuerURL is the URL to the OIDC provider.
    #[serde(rename = "issuerURL")]
    pub issuer_url: String,
    /// PromptTypes is an optional list of string values that specifies whether the identity provider prompts the end user
    /// for re-authentication and consent. See the RFC for more information on prompt types:
    /// https://openid.net/specs/openid-connect-core-1_0.html.
    /// Default: "Consent"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "promptTypes")]
    pub prompt_types: Option<Vec<String>>,
    /// RequestedScopes is a list of scopes to request from the OIDC provider. If not provided, the following scopes are
    /// requested: ["openid", "email", "profile", "groups", "offline_access"].
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestedScopes")]
    pub requested_scopes: Option<Vec<String>>,
    /// Default: "Dex"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<AuthenticationOidcType>,
    /// UsernameClaim specifies which claim to use from the OIDC provider as the username.
    #[serde(rename = "usernameClaim")]
    pub username_claim: String,
    /// Deprecated. Please use Authentication.Spec.UsernamePrefix instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usernamePrefix")]
    pub username_prefix: Option<String>,
}

/// OIDC contains the configuration needed to setup OIDC authentication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthenticationOidcEmailVerification {
    Verify,
    InsecureSkip,
}

/// OIDC contains the configuration needed to setup OIDC authentication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthenticationOidcType {
    Dex,
    Tigera,
}

/// Openshift contains the configuration needed to setup Openshift OAuth authentication.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationOpenshift {
    /// IssuerURL is the URL to the Openshift OAuth provider. Ex.: https://api.my-ocp-domain.com:6443
    #[serde(rename = "issuerURL")]
    pub issuer_url: String,
}

/// AuthenticationStatus defines the observed state of Authentication
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthenticationStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of
    /// Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

