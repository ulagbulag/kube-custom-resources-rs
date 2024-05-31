// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/grafana/loki/loki.grafana.com/v1/rulerconfigs.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RulerConfigSpec defines the desired state of Ruler
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "loki.grafana.com", version = "v1", kind = "RulerConfig", plural = "rulerconfigs")]
#[kube(namespaced)]
#[kube(status = "RulerConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct RulerConfigSpec {
    /// Defines alert manager configuration to notify on firing alerts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alertmanager: Option<RulerConfigAlertmanager>,
    /// Interval on how frequently to evaluate rules.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evaluationInterval")]
    pub evaluation_interval: Option<String>,
    /// Overrides defines the config overrides to be applied per-tenant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<BTreeMap<String, RulerConfigOverrides>>,
    /// Interval on how frequently to poll for new rule definitions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollInterval")]
    pub poll_interval: Option<String>,
    /// Defines a remote write endpoint to write recording rule metrics.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteWrite")]
    pub remote_write: Option<RulerConfigRemoteWrite>,
}

/// Defines alert manager configuration to notify on firing alerts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanager {
    /// Client configuration for reaching the alertmanager endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<RulerConfigAlertmanagerClient>,
    /// Defines the configuration for DNS-based discovery of AlertManager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery: Option<RulerConfigAlertmanagerDiscovery>,
    /// If enabled, then requests to Alertmanager use the v2 API.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableV2")]
    pub enable_v2: Option<bool>,
    /// List of AlertManager URLs to send notifications to. Each Alertmanager URL is treated as
    /// a separate group in the configuration. Multiple Alertmanagers in HA per group can be
    /// supported by using DNS resolution (See EnableDNSDiscovery).
    pub endpoints: Vec<String>,
    /// Additional labels to add to all alerts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalLabels")]
    pub external_labels: Option<BTreeMap<String, String>>,
    /// URL for alerts return path.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalUrl")]
    pub external_url: Option<String>,
    /// Defines the configuration for the notification queue to AlertManager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notificationQueue")]
    pub notification_queue: Option<RulerConfigAlertmanagerNotificationQueue>,
    /// List of alert relabel configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relabelConfigs")]
    pub relabel_configs: Option<Vec<RulerConfigAlertmanagerRelabelConfigs>>,
}

/// Client configuration for reaching the alertmanager endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerClient {
    /// Basic authentication configuration for reaching the alertmanager endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<RulerConfigAlertmanagerClientBasicAuth>,
    /// Header authentication configuration for reaching the alertmanager endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerAuth")]
    pub header_auth: Option<RulerConfigAlertmanagerClientHeaderAuth>,
    /// TLS configuration for reaching the alertmanager endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<RulerConfigAlertmanagerClientTls>,
}

/// Basic authentication configuration for reaching the alertmanager endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerClientBasicAuth {
    /// The subject's password for the basic authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The subject's username for the basic authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Header authentication configuration for reaching the alertmanager endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerClientHeaderAuth {
    /// The credentials for the header authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// The credentials file for the Header authentication configuration. It is mutually exclusive with `credentials`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsFile")]
    pub credentials_file: Option<String>,
    /// The authentication type for the header authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// TLS configuration for reaching the alertmanager endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerClientTls {
    /// The CA certificate file path for the TLS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caPath")]
    pub ca_path: Option<String>,
    /// The client-side certificate file path for the TLS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certPath")]
    pub cert_path: Option<String>,
    /// The client-side key file path for the TLS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyPath")]
    pub key_path: Option<String>,
    /// The server name to validate in the alertmanager server certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<String>,
}

/// Defines the configuration for DNS-based discovery of AlertManager hosts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerDiscovery {
    /// Use DNS SRV records to discover Alertmanager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableSRV")]
    pub enable_srv: Option<bool>,
    /// How long to wait between refreshing DNS resolutions of Alertmanager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<String>,
}

/// Defines the configuration for the notification queue to AlertManager hosts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerNotificationQueue {
    /// Capacity of the queue for notifications to be sent to the Alertmanager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// Minimum duration between alert and restored "for" state. This is maintained
    /// only for alerts with configured "for" time greater than the grace period.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forGracePeriod")]
    pub for_grace_period: Option<String>,
    /// Max time to tolerate outage for restoring "for" state of alert.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forOutageTolerance")]
    pub for_outage_tolerance: Option<String>,
    /// Minimum amount of time to wait before resending an alert to Alertmanager.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resendDelay")]
    pub resend_delay: Option<String>,
    /// HTTP timeout duration when sending notifications to the Alertmanager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion.
/// It defines `<metric_relabel_configs>` and `<alert_relabel_configs>` sections of Prometheus configuration.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigAlertmanagerRelabelConfigs {
    /// Action to perform based on regex matching. Default is 'replace'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulerConfigAlertmanagerRelabelConfigsAction>,
    /// Modulus to take of the hash of the source label values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// Regular expression against which the extracted value is matched. Default is '(.*)'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Replacement value against which a regex replace is performed if the
    /// regular expression matches. Regex capture groups are available. Default is '$1'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Separator placed between concatenated source label values. default is ';'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// The source labels select values from existing labels. Their content is concatenated
    /// using the configured separator and matched against the configured regular expression
    /// for the replace, keep, and drop actions.
    #[serde(rename = "sourceLabels")]
    pub source_labels: Vec<String>,
    /// Label to which the resulting value is written in a replace action.
    /// It is mandatory for replace actions. Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion.
/// It defines `<metric_relabel_configs>` and `<alert_relabel_configs>` sections of Prometheus configuration.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RulerConfigAlertmanagerRelabelConfigsAction {
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "hashmod")]
    Hashmod,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "labeldrop")]
    Labeldrop,
    #[serde(rename = "labelkeep")]
    Labelkeep,
    #[serde(rename = "labelmap")]
    Labelmap,
    #[serde(rename = "replace")]
    Replace,
}

/// Overrides defines the config overrides to be applied per-tenant.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverrides {
    /// AlertManagerOverrides defines the overrides to apply to the alertmanager config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alertmanager: Option<RulerConfigOverridesAlertmanager>,
}

/// AlertManagerOverrides defines the overrides to apply to the alertmanager config.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanager {
    /// Client configuration for reaching the alertmanager endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<RulerConfigOverridesAlertmanagerClient>,
    /// Defines the configuration for DNS-based discovery of AlertManager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery: Option<RulerConfigOverridesAlertmanagerDiscovery>,
    /// If enabled, then requests to Alertmanager use the v2 API.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableV2")]
    pub enable_v2: Option<bool>,
    /// List of AlertManager URLs to send notifications to. Each Alertmanager URL is treated as
    /// a separate group in the configuration. Multiple Alertmanagers in HA per group can be
    /// supported by using DNS resolution (See EnableDNSDiscovery).
    pub endpoints: Vec<String>,
    /// Additional labels to add to all alerts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalLabels")]
    pub external_labels: Option<BTreeMap<String, String>>,
    /// URL for alerts return path.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalUrl")]
    pub external_url: Option<String>,
    /// Defines the configuration for the notification queue to AlertManager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notificationQueue")]
    pub notification_queue: Option<RulerConfigOverridesAlertmanagerNotificationQueue>,
    /// List of alert relabel configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relabelConfigs")]
    pub relabel_configs: Option<Vec<RulerConfigOverridesAlertmanagerRelabelConfigs>>,
}

/// Client configuration for reaching the alertmanager endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerClient {
    /// Basic authentication configuration for reaching the alertmanager endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "basicAuth")]
    pub basic_auth: Option<RulerConfigOverridesAlertmanagerClientBasicAuth>,
    /// Header authentication configuration for reaching the alertmanager endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerAuth")]
    pub header_auth: Option<RulerConfigOverridesAlertmanagerClientHeaderAuth>,
    /// TLS configuration for reaching the alertmanager endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<RulerConfigOverridesAlertmanagerClientTls>,
}

/// Basic authentication configuration for reaching the alertmanager endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerClientBasicAuth {
    /// The subject's password for the basic authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// The subject's username for the basic authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Header authentication configuration for reaching the alertmanager endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerClientHeaderAuth {
    /// The credentials for the header authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// The credentials file for the Header authentication configuration. It is mutually exclusive with `credentials`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsFile")]
    pub credentials_file: Option<String>,
    /// The authentication type for the header authentication configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// TLS configuration for reaching the alertmanager endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerClientTls {
    /// The CA certificate file path for the TLS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caPath")]
    pub ca_path: Option<String>,
    /// The client-side certificate file path for the TLS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certPath")]
    pub cert_path: Option<String>,
    /// The client-side key file path for the TLS configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyPath")]
    pub key_path: Option<String>,
    /// The server name to validate in the alertmanager server certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverName")]
    pub server_name: Option<String>,
}

/// Defines the configuration for DNS-based discovery of AlertManager hosts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerDiscovery {
    /// Use DNS SRV records to discover Alertmanager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableSRV")]
    pub enable_srv: Option<bool>,
    /// How long to wait between refreshing DNS resolutions of Alertmanager hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshInterval")]
    pub refresh_interval: Option<String>,
}

/// Defines the configuration for the notification queue to AlertManager hosts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerNotificationQueue {
    /// Capacity of the queue for notifications to be sent to the Alertmanager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// Minimum duration between alert and restored "for" state. This is maintained
    /// only for alerts with configured "for" time greater than the grace period.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forGracePeriod")]
    pub for_grace_period: Option<String>,
    /// Max time to tolerate outage for restoring "for" state of alert.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forOutageTolerance")]
    pub for_outage_tolerance: Option<String>,
    /// Minimum amount of time to wait before resending an alert to Alertmanager.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resendDelay")]
    pub resend_delay: Option<String>,
    /// HTTP timeout duration when sending notifications to the Alertmanager.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion.
/// It defines `<metric_relabel_configs>` and `<alert_relabel_configs>` sections of Prometheus configuration.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigOverridesAlertmanagerRelabelConfigs {
    /// Action to perform based on regex matching. Default is 'replace'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulerConfigOverridesAlertmanagerRelabelConfigsAction>,
    /// Modulus to take of the hash of the source label values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// Regular expression against which the extracted value is matched. Default is '(.*)'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Replacement value against which a regex replace is performed if the
    /// regular expression matches. Regex capture groups are available. Default is '$1'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Separator placed between concatenated source label values. default is ';'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// The source labels select values from existing labels. Their content is concatenated
    /// using the configured separator and matched against the configured regular expression
    /// for the replace, keep, and drop actions.
    #[serde(rename = "sourceLabels")]
    pub source_labels: Vec<String>,
    /// Label to which the resulting value is written in a replace action.
    /// It is mandatory for replace actions. Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion.
/// It defines `<metric_relabel_configs>` and `<alert_relabel_configs>` sections of Prometheus configuration.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RulerConfigOverridesAlertmanagerRelabelConfigsAction {
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "hashmod")]
    Hashmod,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "labeldrop")]
    Labeldrop,
    #[serde(rename = "labelkeep")]
    Labelkeep,
    #[serde(rename = "labelmap")]
    Labelmap,
    #[serde(rename = "replace")]
    Replace,
}

/// Defines a remote write endpoint to write recording rule metrics.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigRemoteWrite {
    /// Defines the configuration for remote write client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<RulerConfigRemoteWriteClient>,
    /// Enable remote-write functionality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Defines the configuration for remote write client queue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<RulerConfigRemoteWriteQueue>,
    /// Minimum period to wait between refreshing remote-write reconfigurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshPeriod")]
    pub refresh_period: Option<String>,
}

/// Defines the configuration for remote write client.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigRemoteWriteClient {
    /// Additional HTTP headers to be sent along with each remote write request.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalHeaders")]
    pub additional_headers: Option<BTreeMap<String, String>>,
    /// Type of authorzation to use to access the remote write endpoint
    pub authorization: RulerConfigRemoteWriteClientAuthorization,
    /// Name of a secret in the namespace configured for authorization secrets.
    #[serde(rename = "authorizationSecretName")]
    pub authorization_secret_name: String,
    /// Configure whether HTTP requests follow HTTP 3xx redirects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "followRedirects")]
    pub follow_redirects: Option<bool>,
    /// Name of the remote write config, which if specified must be unique among remote write configs.
    pub name: String,
    /// Optional proxy URL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyUrl")]
    pub proxy_url: Option<String>,
    /// List of remote write relabel configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relabelConfigs")]
    pub relabel_configs: Option<Vec<RulerConfigRemoteWriteClientRelabelConfigs>>,
    /// Timeout for requests to the remote write endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// The URL of the endpoint to send samples to.
    pub url: String,
}

/// Defines the configuration for remote write client.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RulerConfigRemoteWriteClientAuthorization {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "header")]
    Header,
}

/// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion.
/// It defines `<metric_relabel_configs>` and `<alert_relabel_configs>` sections of Prometheus configuration.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigRemoteWriteClientRelabelConfigs {
    /// Action to perform based on regex matching. Default is 'replace'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<RulerConfigRemoteWriteClientRelabelConfigsAction>,
    /// Modulus to take of the hash of the source label values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modulus: Option<i64>,
    /// Regular expression against which the extracted value is matched. Default is '(.*)'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Replacement value against which a regex replace is performed if the
    /// regular expression matches. Regex capture groups are available. Default is '$1'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    /// Separator placed between concatenated source label values. default is ';'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// The source labels select values from existing labels. Their content is concatenated
    /// using the configured separator and matched against the configured regular expression
    /// for the replace, keep, and drop actions.
    #[serde(rename = "sourceLabels")]
    pub source_labels: Vec<String>,
    /// Label to which the resulting value is written in a replace action.
    /// It is mandatory for replace actions. Regex capture groups are available.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetLabel")]
    pub target_label: Option<String>,
}

/// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion.
/// It defines `<metric_relabel_configs>` and `<alert_relabel_configs>` sections of Prometheus configuration.
/// More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RulerConfigRemoteWriteClientRelabelConfigsAction {
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "hashmod")]
    Hashmod,
    #[serde(rename = "keep")]
    Keep,
    #[serde(rename = "labeldrop")]
    Labeldrop,
    #[serde(rename = "labelkeep")]
    Labelkeep,
    #[serde(rename = "labelmap")]
    Labelmap,
    #[serde(rename = "replace")]
    Replace,
}

/// Defines the configuration for remote write client queue.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigRemoteWriteQueue {
    /// Maximum time a sample will wait in buffer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "batchSendDeadline")]
    pub batch_send_deadline: Option<String>,
    /// Number of samples to buffer per shard before we block reading of more
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// Maximum retry delay.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBackOffPeriod")]
    pub max_back_off_period: Option<String>,
    /// Maximum number of samples per send.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSamplesPerSend")]
    pub max_samples_per_send: Option<i32>,
    /// Maximum number of shards, i.e. amount of concurrency.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxShards")]
    pub max_shards: Option<i32>,
    /// Initial retry delay. Gets doubled for every retry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minBackOffPeriod")]
    pub min_back_off_period: Option<String>,
    /// Minimum number of shards, i.e. amount of concurrency.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minShards")]
    pub min_shards: Option<i32>,
}

/// RulerConfigStatus defines the observed state of RulerConfig
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RulerConfigStatus {
    /// Conditions of the RulerConfig health.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

