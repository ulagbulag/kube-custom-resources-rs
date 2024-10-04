// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/stackabletech/secret-operator/secrets.stackable.tech/v1alpha1/secretclasses.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// A [SecretClass](https://docs.stackable.tech/home/nightly/secret-operator/secretclass) is a cluster-global Kubernetes resource that defines a category of secrets that the Secret Operator knows how to provision.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "secrets.stackable.tech", version = "v1alpha1", kind = "SecretClass", plural = "secretclasses")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SecretClassSpec {
    /// Each SecretClass is associated with a single [backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend), which dictates the mechanism for issuing that kind of Secret.
    pub backend: SecretClassBackend,
}

/// Each SecretClass is associated with a single [backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend), which dictates the mechanism for issuing that kind of Secret.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackend {
    /// The [`autoTls` backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-autotls) issues a TLS certificate signed by the Secret Operator. The certificate authority can be provided by the administrator, or managed automatically by the Secret Operator.
    /// 
    /// A new certificate and key pair will be generated and signed for each Pod, keys or certificates are never reused.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoTls")]
    pub auto_tls: Option<SecretClassBackendAutoTls>,
    /// The [`experimentalCertManager` backend][1] injects a TLS certificate issued by [cert-manager](https://cert-manager.io/).
    /// 
    /// A new certificate will be requested the first time it is used by a Pod, it will be reused after that (subject to cert-manager renewal rules).
    /// 
    /// [1]: https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-certmanager
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentalCertManager")]
    pub experimental_cert_manager: Option<SecretClassBackendExperimentalCertManager>,
    /// The [`k8sSearch` backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-k8ssearch) can be used to mount Secrets across namespaces into Pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "k8sSearch")]
    pub k8s_search: Option<SecretClassBackendK8sSearch>,
    /// The [`kerberosKeytab` backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-kerberoskeytab) creates a Kerberos keytab file for a selected realm. The Kerberos KDC and administrator credentials must be provided by the administrator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kerberosKeytab")]
    pub kerberos_keytab: Option<SecretClassBackendKerberosKeytab>,
}

/// The [`autoTls` backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-autotls) issues a TLS certificate signed by the Secret Operator. The certificate authority can be provided by the administrator, or managed automatically by the Secret Operator.
/// 
/// A new certificate and key pair will be generated and signed for each Pod, keys or certificates are never reused.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendAutoTls {
    /// Configures the certificate authority used to issue Pod certificates.
    pub ca: SecretClassBackendAutoTlsCa,
    /// Maximum lifetime the created certificates are allowed to have. In case consumers request a longer lifetime than allowed by this setting, the lifetime will be the minimum of both, so this setting takes precedence. The default value is 15 days.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxCertificateLifetime")]
    pub max_certificate_lifetime: Option<String>,
}

/// Configures the certificate authority used to issue Pod certificates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendAutoTlsCa {
    /// Whether the certificate authority should be managed by Secret Operator, including being generated if it does not already exist.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoGenerate")]
    pub auto_generate: Option<bool>,
    /// The lifetime of each generated certificate authority.
    /// 
    /// Should always be more than double `maxCertificateLifetime`.
    /// 
    /// If `autoGenerate: true` then the Secret Operator will prepare a new CA certificate the old CA approaches expiration. If `autoGenerate: false` then the Secret Operator will log a warning instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificateLifetime")]
    pub ca_certificate_lifetime: Option<String>,
    /// The algorithm used to generate a key pair and required configuration settings. Currently only RSA and a key length of 2048, 3072 or 4096 bits can be configured.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyGeneration")]
    pub key_generation: Option<SecretClassBackendAutoTlsCaKeyGeneration>,
    /// Reference (name and namespace) to a Kubernetes Secret object where the CA certificate and key is stored in the keys `ca.crt` and `ca.key` respectively.
    pub secret: SecretClassBackendAutoTlsCaSecret,
}

/// The algorithm used to generate a key pair and required configuration settings. Currently only RSA and a key length of 2048, 3072 or 4096 bits can be configured.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendAutoTlsCaKeyGeneration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsa: Option<SecretClassBackendAutoTlsCaKeyGenerationRsa>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendAutoTlsCaKeyGenerationRsa {
    /// The amount of bits used for generating the RSA keypair. Currently, `2048`, `3072` and `4096` are supported. Defaults to `2048` bits.
    pub length: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretClassBackendAutoTlsCaKeyGenerationRsaLength {
    #[serde(rename = "2048")]
    r#_2048,
    #[serde(rename = "3072")]
    r#_3072,
    #[serde(rename = "4096")]
    r#_4096,
}

/// Reference (name and namespace) to a Kubernetes Secret object where the CA certificate and key is stored in the keys `ca.crt` and `ca.key` respectively.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendAutoTlsCaSecret {
    /// Name of the Secret being referred to.
    pub name: String,
    /// Namespace of the Secret being referred to.
    pub namespace: String,
}

/// The [`experimentalCertManager` backend][1] injects a TLS certificate issued by [cert-manager](https://cert-manager.io/).
/// 
/// A new certificate will be requested the first time it is used by a Pod, it will be reused after that (subject to cert-manager renewal rules).
/// 
/// [1]: https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-certmanager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SecretClassBackendExperimentalCertManager {
    /// The default lifetime of certificates.
    /// 
    /// Defaults to 1 day. This may need to be increased for external issuers that impose rate limits (such as Let's Encrypt).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultCertificateLifetime")]
    pub default_certificate_lifetime: Option<String>,
    /// A reference to the cert-manager issuer that the certificates should be requested from.
    pub issuer: SecretClassBackendExperimentalCertManagerIssuer,
}

/// A reference to the cert-manager issuer that the certificates should be requested from.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SecretClassBackendExperimentalCertManagerIssuer {
    /// The kind of the issuer, Issuer or ClusterIssuer.
    /// 
    /// If Issuer then it must be in the same namespace as the Pods using it.
    pub kind: SecretClassBackendExperimentalCertManagerIssuerKind,
    /// The name of the issuer.
    pub name: String,
}

/// A reference to the cert-manager issuer that the certificates should be requested from.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretClassBackendExperimentalCertManagerIssuerKind {
    Issuer,
    ClusterIssuer,
}

/// The [`k8sSearch` backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-k8ssearch) can be used to mount Secrets across namespaces into Pods.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendK8sSearch {
    /// Configures the namespace searched for Secret objects.
    #[serde(rename = "searchNamespace")]
    pub search_namespace: SecretClassBackendK8sSearchSearchNamespace,
}

/// Configures the namespace searched for Secret objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendK8sSearchSearchNamespace {
    /// The Secret objects are located in a single global namespace. Should be used for secrets that are provisioned by the cluster administrator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Secret objects are located in the same namespace as the Pod object. Should be used for Secrets that are provisioned by the application administrator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pod: Option<SecretClassBackendK8sSearchSearchNamespacePod>,
}

/// The Secret objects are located in the same namespace as the Pod object. Should be used for Secrets that are provisioned by the application administrator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendK8sSearchSearchNamespacePod {
}

/// The [`kerberosKeytab` backend](https://docs.stackable.tech/home/nightly/secret-operator/secretclass#backend-kerberoskeytab) creates a Kerberos keytab file for a selected realm. The Kerberos KDC and administrator credentials must be provided by the administrator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytab {
    /// Kerberos admin configuration settings.
    pub admin: SecretClassBackendKerberosKeytabAdmin,
    /// Reference (`name` and `namespace`) to a K8s Secret object where a keytab with administrative privileges is stored in the key `keytab`.
    #[serde(rename = "adminKeytabSecret")]
    pub admin_keytab_secret: SecretClassBackendKerberosKeytabAdminKeytabSecret,
    /// The admin principal.
    #[serde(rename = "adminPrincipal")]
    pub admin_principal: String,
    /// The hostname of the Kerberos Key Distribution Center (KDC). This should be provided by the Kerberos administrator.
    pub kdc: String,
    /// The name of the Kerberos realm. This should be provided by the Kerberos administrator.
    #[serde(rename = "realmName")]
    pub realm_name: String,
}

/// Kerberos admin configuration settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdmin {
    /// Credentials should be provisioned in a Microsoft Active Directory domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDirectory")]
    pub active_directory: Option<SecretClassBackendKerberosKeytabAdminActiveDirectory>,
    /// Credentials should be provisioned in a MIT Kerberos Admin Server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mit: Option<SecretClassBackendKerberosKeytabAdminMit>,
}

/// Credentials should be provisioned in a Microsoft Active Directory domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdminActiveDirectory {
    /// Allows samAccountName generation for new accounts to be customized. Note that setting this field (even if empty) makes the Secret Operator take over the generation duty from the domain controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentalGenerateSamAccountName")]
    pub experimental_generate_sam_account_name: Option<SecretClassBackendKerberosKeytabAdminActiveDirectoryExperimentalGenerateSamAccountName>,
    /// An AD LDAP server, such as the AD Domain Controller. This must match the server’s FQDN, or GSSAPI authentication will fail.
    #[serde(rename = "ldapServer")]
    pub ldap_server: String,
    /// Reference (name and namespace) to a Kubernetes Secret object containing the TLS CA (in `ca.crt`) that the LDAP server’s certificate should be authenticated against.
    #[serde(rename = "ldapTlsCaSecret")]
    pub ldap_tls_ca_secret: SecretClassBackendKerberosKeytabAdminActiveDirectoryLdapTlsCaSecret,
    /// Reference (name and namespace) to a Kubernetes Secret object where workload passwords will be stored. This must not be accessible to end users.
    #[serde(rename = "passwordCacheSecret")]
    pub password_cache_secret: SecretClassBackendKerberosKeytabAdminActiveDirectoryPasswordCacheSecret,
    /// The root Distinguished Name (DN) for AD-managed schemas, typically `CN=Schema,CN=Configuration,{domain_dn}`.
    #[serde(rename = "schemaDistinguishedName")]
    pub schema_distinguished_name: String,
    /// The root Distinguished Name (DN) where service accounts should be provisioned, typically `CN=Users,{domain_dn}`.
    #[serde(rename = "userDistinguishedName")]
    pub user_distinguished_name: String,
}

/// Allows samAccountName generation for new accounts to be customized. Note that setting this field (even if empty) makes the Secret Operator take over the generation duty from the domain controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdminActiveDirectoryExperimentalGenerateSamAccountName {
    /// A prefix to be prepended to generated samAccountNames.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The total length of generated samAccountNames, _including_ `prefix`. Must be larger than the length of `prefix`, but at most `20`.
    /// 
    /// Note that this should be as large as possible, to minimize the risk of collisions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalLength")]
    pub total_length: Option<u8>,
}

/// Reference (name and namespace) to a Kubernetes Secret object containing the TLS CA (in `ca.crt`) that the LDAP server’s certificate should be authenticated against.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdminActiveDirectoryLdapTlsCaSecret {
    /// Name of the Secret being referred to.
    pub name: String,
    /// Namespace of the Secret being referred to.
    pub namespace: String,
}

/// Reference (name and namespace) to a Kubernetes Secret object where workload passwords will be stored. This must not be accessible to end users.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdminActiveDirectoryPasswordCacheSecret {
    /// Name of the Secret being referred to.
    pub name: String,
    /// Namespace of the Secret being referred to.
    pub namespace: String,
}

/// Credentials should be provisioned in a MIT Kerberos Admin Server.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdminMit {
    /// The hostname of the Kerberos Admin Server. This should be provided by the Kerberos administrator.
    #[serde(rename = "kadminServer")]
    pub kadmin_server: String,
}

/// Reference (`name` and `namespace`) to a K8s Secret object where a keytab with administrative privileges is stored in the key `keytab`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretClassBackendKerberosKeytabAdminKeytabSecret {
    /// Name of the Secret being referred to.
    pub name: String,
    /// Namespace of the Secret being referred to.
    pub namespace: String,
}

