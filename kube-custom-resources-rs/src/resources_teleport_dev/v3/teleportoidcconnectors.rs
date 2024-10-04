// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v3/teleportoidcconnectors.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// OIDCConnector resource definition v3 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v3", kind = "TeleportOIDCConnector", plural = "teleportoidcconnectors")]
#[kube(namespaced)]
#[kube(status = "TeleportOIDCConnectorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TeleportOIDCConnectorSpec {
    /// ACR is an Authentication Context Class Reference value. The meaning of the ACR value is context-specific and varies for identity providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acr_values: Option<String>,
    /// AllowUnverifiedEmail tells the connector to accept OIDC users with unverified emails.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_unverified_email: Option<bool>,
    /// ClaimsToRoles specifies a dynamic mapping from claims to roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims_to_roles: Option<Vec<TeleportOIDCConnectorClaimsToRoles>>,
    /// ClientID is the id of the authentication client (Teleport Auth server).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ClientRedirectSettings defines which client redirect URLs are allowed for non-browser SSO logins other than the standard localhost ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_redirect_settings: Option<TeleportOIDCConnectorClientRedirectSettings>,
    /// ClientSecret is used to authenticate the client. This field supports secret lookup. See the operator documentation for more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Display is the friendly name for this provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// GoogleAdminEmail is the email of a google admin to impersonate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_admin_email: Option<String>,
    /// GoogleServiceAccount is a string containing google service account credentials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_service_account: Option<String>,
    /// GoogleServiceAccountURI is a path to a google service account uri.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub google_service_account_uri: Option<String>,
    /// IssuerURL is the endpoint of the provider, e.g. https://accounts.google.com.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    /// MaxAge is the amount of time that user logins are valid for. If a user logs in, but then does not login again within this time period, they will be forced to re-authenticate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<String>,
    /// MFASettings contains settings to enable SSO MFA checks through this auth connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa: Option<TeleportOIDCConnectorMfa>,
    /// Prompt is an optional OIDC prompt. An empty string omits prompt. If not specified, it defaults to select_account for backwards compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Provider is the external identity provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// RedirectURLs is a list of callback URLs which the identity provider can use to redirect the client back to the Teleport Proxy to complete authentication. This list should match the URLs on the provider's side. The URL used for a given auth request will be chosen to match the requesting Proxy's public address. If there is no match, the first url in the list will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<Vec<String>>,
    /// Scope specifies additional scopes set by provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    /// UsernameClaim specifies the name of the claim from the OIDC connector to be used as the user's username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOIDCConnectorClaimsToRoles {
    /// Claim is a claim name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
    /// Roles is a list of static teleport roles to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Value is a claim value to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ClientRedirectSettings defines which client redirect URLs are allowed for non-browser SSO logins other than the standard localhost ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOIDCConnectorClientRedirectSettings {
    /// a list of hostnames allowed for https client redirect URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_https_hostnames: Option<Vec<String>>,
    /// a list of CIDRs allowed for HTTP or HTTPS client redirect URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_allowed_cidr_ranges: Option<Vec<String>>,
}

/// MFASettings contains settings to enable SSO MFA checks through this auth connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOIDCConnectorMfa {
    /// AcrValues are Authentication Context Class Reference values. The meaning of the ACR value is context-specific and varies for identity providers. Some identity providers support MFA specific contexts, such Okta with its "phr" (phishing-resistant) ACR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acr_values: Option<String>,
    /// ClientID is the OIDC OAuth app client ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ClientSecret is the OIDC OAuth app client secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Enabled specified whether this OIDC connector supports MFA checks. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Prompt is an optional OIDC prompt. An empty string omits prompt. If not specified, it defaults to select_account for backwards compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

/// Status defines the observed state of the Teleport resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOIDCConnectorStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

