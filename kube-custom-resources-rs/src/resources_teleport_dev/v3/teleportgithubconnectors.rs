// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/gravitational/teleport/resources.teleport.dev/v3/teleportgithubconnectors.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GithubConnector resource definition v3 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v3", kind = "TeleportGithubConnector", plural = "teleportgithubconnectors")]
#[kube(namespaced)]
#[kube(status = "TeleportGithubConnectorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TeleportGithubConnectorSpec {
    /// APIEndpointURL is the URL of the API endpoint of the Github instance this connector is for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_endpoint_url: Option<String>,
    /// ClientID is the Github OAuth app client ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ClientRedirectSettings defines which client redirect URLs are allowed for non-browser SSO logins other than the standard localhost ones.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_redirect_settings: Option<TeleportGithubConnectorClientRedirectSettings>,
    /// ClientSecret is the Github OAuth app client secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Display is the connector display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// EndpointURL is the URL of the GitHub instance this connector is for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// RedirectURL is the authorization callback URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// TeamsToRoles maps Github team memberships onto allowed roles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams_to_roles: Option<Vec<TeleportGithubConnectorTeamsToRoles>>,
}

/// ClientRedirectSettings defines which client redirect URLs are allowed for non-browser SSO logins other than the standard localhost ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportGithubConnectorClientRedirectSettings {
    /// a list of hostnames allowed for https client redirect URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_https_hostnames: Option<Vec<String>>,
    /// a list of CIDRs allowed for HTTP or HTTPS client redirect URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_allowed_cidr_ranges: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportGithubConnectorTeamsToRoles {
    /// Organization is a Github organization a user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Roles is a list of allowed logins for this org/team.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Team is a team within the organization a user belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

/// Status defines the observed state of the Teleport resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportGithubConnectorStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

