// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1/logcollectors.yaml --derive=PartialEq
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

/// Specification of the desired state for Tigera log collection.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "LogCollector", plural = "logcollectors")]
#[kube(status = "LogCollectorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct LogCollectorSpec {
    /// Configuration for importing audit logs from managed kubernetes cluster log sources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalSources")]
    pub additional_sources: Option<LogCollectorAdditionalSources>,
    /// Configuration for exporting flow, audit, and DNS logs to external storage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalStores")]
    pub additional_stores: Option<LogCollectorAdditionalStores>,
    /// Configuration for enabling/disabling process path collection in flowlogs.
    /// If Enabled, this feature sets hostPID to true in order to read process cmdline.
    /// Default: Enabled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectProcessPath")]
    pub collect_process_path: Option<LogCollectorCollectProcessPath>,
    /// EKSLogForwarderDeployment configures the EKSLogForwarderDeployment Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eksLogForwarderDeployment")]
    pub eks_log_forwarder_deployment: Option<LogCollectorEksLogForwarderDeployment>,
    /// FluentdDaemonSet configures the Fluentd DaemonSet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fluentdDaemonSet")]
    pub fluentd_daemon_set: Option<LogCollectorFluentdDaemonSet>,
    /// If running as a multi-tenant management cluster, the namespace in which
    /// the management cluster's tenant services are running.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiTenantManagementClusterNamespace")]
    pub multi_tenant_management_cluster_namespace: Option<String>,
}

/// Configuration for importing audit logs from managed kubernetes cluster log sources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorAdditionalSources {
    /// If specified with EKS Provider in Installation, enables fetching EKS
    /// audit logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eksCloudwatchLog")]
    pub eks_cloudwatch_log: Option<LogCollectorAdditionalSourcesEksCloudwatchLog>,
}

/// If specified with EKS Provider in Installation, enables fetching EKS
/// audit logs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorAdditionalSourcesEksCloudwatchLog {
    /// Cloudwatch audit logs fetching interval in seconds.
    /// Default: 60
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fetchInterval")]
    pub fetch_interval: Option<i32>,
    /// Cloudwatch log-group name containing EKS audit logs.
    #[serde(rename = "groupName")]
    pub group_name: String,
    /// AWS Region EKS cluster is hosted in.
    pub region: String,
    /// Prefix of Cloudwatch log stream containing EKS audit logs in the log-group.
    /// Default: kube-apiserver-audit-
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "streamPrefix")]
    pub stream_prefix: Option<String>,
}

/// Configuration for exporting flow, audit, and DNS logs to external storage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorAdditionalStores {
    /// If specified, enables exporting of flow, audit, and DNS logs to Amazon S3 storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<LogCollectorAdditionalStoresS3>,
    /// If specified, enables exporting of flow, audit, and DNS logs to splunk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splunk: Option<LogCollectorAdditionalStoresSplunk>,
    /// If specified, enables exporting of flow, audit, and DNS logs to syslog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syslog: Option<LogCollectorAdditionalStoresSyslog>,
}

/// If specified, enables exporting of flow, audit, and DNS logs to Amazon S3 storage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorAdditionalStoresS3 {
    /// Name of the S3 bucket to send logs
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// Path in the S3 bucket where to send logs
    #[serde(rename = "bucketPath")]
    pub bucket_path: String,
    /// AWS Region of the S3 bucket
    pub region: String,
}

/// If specified, enables exporting of flow, audit, and DNS logs to splunk.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorAdditionalStoresSplunk {
    /// Location for splunk's http event collector end point. example `https://1.2.3.4:8088`
    pub endpoint: String,
}

/// If specified, enables exporting of flow, audit, and DNS logs to syslog.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorAdditionalStoresSyslog {
    /// Encryption configures traffic encryption to the Syslog server.
    /// Default: None
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<LogCollectorAdditionalStoresSyslogEncryption>,
    /// Location of the syslog server. example: tcp://1.2.3.4:601
    pub endpoint: String,
    /// If no values are provided, the list will be updated to include log types Audit, DNS and Flows.
    /// Default: Audit, DNS, Flows
    #[serde(rename = "logTypes")]
    pub log_types: Vec<String>,
    /// PacketSize defines the maximum size of packets to send to syslog.
    /// In general this is only needed if you notice long logs being truncated.
    /// Default: 1024
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "packetSize")]
    pub packet_size: Option<i32>,
}

/// If specified, enables exporting of flow, audit, and DNS logs to syslog.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCollectorAdditionalStoresSyslogEncryption {
    None,
    #[serde(rename = "TLS")]
    Tls,
}

/// Specification of the desired state for Tigera log collection.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCollectorCollectProcessPath {
    Enabled,
    Disabled,
}

/// EKSLogForwarderDeployment configures the EKSLogForwarderDeployment Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeployment {
    /// Spec is the specification of the EKSLogForwarder Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LogCollectorEksLogForwarderDeploymentSpec>,
}

/// Spec is the specification of the EKSLogForwarder Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpec {
    /// Template describes the EKSLogForwarder Deployment pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<LogCollectorEksLogForwarderDeploymentSpecTemplate>,
}

/// Template describes the EKSLogForwarder Deployment pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplate {
    /// Spec is the EKSLogForwarder Deployment's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LogCollectorEksLogForwarderDeploymentSpecTemplateSpec>,
}

/// Spec is the EKSLogForwarder Deployment's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpec {
    /// Containers is a list of EKSLogForwarder containers.
    /// If specified, this overrides the specified EKSLogForwarder Deployment containers.
    /// If omitted, the EKSLogForwarder Deployment will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainers>>,
    /// InitContainers is a list of EKSLogForwarder init containers.
    /// If specified, this overrides the specified EKSLogForwarder Deployment init containers.
    /// If omitted, the EKSLogForwarder Deployment will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainers>>,
}

/// EKSLogForwarderDeploymentContainer is a EKSLogForwarder Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainers {
    /// Name is an enum which identifies the EKSLogForwarder Deployment container by name.
    /// Supported values are: eks-log-forwarder
    pub name: LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named EKSLogForwarder Deployment container's resources.
    /// If omitted, the EKSLogForwarder Deployment will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainersResources>,
}

/// EKSLogForwarderDeploymentContainer is a EKSLogForwarder Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainersName {
    #[serde(rename = "eks-log-forwarder")]
    EksLogForwarder,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named EKSLogForwarder Deployment container's resources.
/// If omitted, the EKSLogForwarder Deployment will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainersResourcesClaims>>,
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
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpecContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// EKSLogForwarderDeploymentInitContainer is a EKSLogForwarder Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the EKSLogForwarder Deployment init container by name.
    /// Supported values are: eks-log-forwarder-startup
    pub name: LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named EKSLogForwarder Deployment init container's resources.
    /// If omitted, the EKSLogForwarder Deployment will use its default value for this init container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainersResources>,
}

/// EKSLogForwarderDeploymentInitContainer is a EKSLogForwarder Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainersName {
    #[serde(rename = "eks-log-forwarder-startup")]
    EksLogForwarderStartup,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named EKSLogForwarder Deployment init container's resources.
/// If omitted, the EKSLogForwarder Deployment will use its default value for this init container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainersResourcesClaims>>,
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
pub struct LogCollectorEksLogForwarderDeploymentSpecTemplateSpecInitContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// FluentdDaemonSet configures the Fluentd DaemonSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSet {
    /// Spec is the specification of the Fluentd DaemonSet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LogCollectorFluentdDaemonSetSpec>,
}

/// Spec is the specification of the Fluentd DaemonSet.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpec {
    /// Template describes the Fluentd DaemonSet pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<LogCollectorFluentdDaemonSetSpecTemplate>,
}

/// Template describes the Fluentd DaemonSet pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpecTemplate {
    /// Spec is the Fluentd DaemonSet's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<LogCollectorFluentdDaemonSetSpecTemplateSpec>,
}

/// Spec is the Fluentd DaemonSet's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpec {
    /// Containers is a list of Fluentd DaemonSet containers.
    /// If specified, this overrides the specified Fluentd DaemonSet containers.
    /// If omitted, the Fluentd DaemonSet will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<LogCollectorFluentdDaemonSetSpecTemplateSpecContainers>>,
    /// InitContainers is a list of Fluentd DaemonSet init containers.
    /// If specified, this overrides the specified Fluentd DaemonSet init containers.
    /// If omitted, the Fluentd DaemonSet will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainers>>,
}

/// FluentdDaemonSetContainer is a Fluentd DaemonSet container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpecContainers {
    /// Name is an enum which identifies the Fluentd DaemonSet container by name.
    /// Supported values are: fluentd
    pub name: LogCollectorFluentdDaemonSetSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named Fluentd DaemonSet container's resources.
    /// If omitted, the Fluentd DaemonSet will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<LogCollectorFluentdDaemonSetSpecTemplateSpecContainersResources>,
}

/// FluentdDaemonSetContainer is a Fluentd DaemonSet container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCollectorFluentdDaemonSetSpecTemplateSpecContainersName {
    #[serde(rename = "fluentd")]
    Fluentd,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named Fluentd DaemonSet container's resources.
/// If omitted, the Fluentd DaemonSet will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<LogCollectorFluentdDaemonSetSpecTemplateSpecContainersResourcesClaims>>,
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
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpecContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// FluentdDaemonSetInitContainer is a Fluentd DaemonSet init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the Fluentd DaemonSet init container by name.
    /// Supported values are: tigera-fluentd-prometheus-tls-key-cert-provisioner
    pub name: LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named Fluentd DaemonSet init container's resources.
    /// If omitted, the Fluentd DaemonSet will use its default value for this init container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainersResources>,
}

/// FluentdDaemonSetInitContainer is a Fluentd DaemonSet init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainersName {
    #[serde(rename = "tigera-fluentd-prometheus-tls-key-cert-provisioner")]
    TigeraFluentdPrometheusTlsKeyCertProvisioner,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named Fluentd DaemonSet init container's resources.
/// If omitted, the Fluentd DaemonSet will use its default value for this init container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainersResourcesClaims>>,
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
pub struct LogCollectorFluentdDaemonSetSpecTemplateSpecInitContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// Most recently observed state for Tigera log collection.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LogCollectorStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of
    /// Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

