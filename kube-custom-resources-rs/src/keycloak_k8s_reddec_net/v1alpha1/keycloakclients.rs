// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/reddec/keycloak-ext-operator/keycloak.k8s.reddec.net/v1alpha1/keycloakclients.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// KeycloakClientSpec defines the desired state of KeycloakClient
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keycloak.k8s.reddec.net", version = "v1alpha1", kind = "KeycloakClient", plural = "keycloakclients")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KeycloakClientSpec {
    /// Annotations (optional) to add to the target secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Domain which will be used for redirect callback.
    pub domain: String,
    /// Labels (optional) to add to the target secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Realm name.
    pub realm: String,
    /// Secret name where to store credentials. Optional, if not set - CRD name will be used. Contains: clientID, clientSecret, realm, discoveryURL, realmURL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// KeycloakClientStatus defines the observed state of KeycloakClient
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KeycloakClientStatus {
}

