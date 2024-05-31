// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/slok/sloth/sloth.slok.dev/v1/prometheusservicelevels.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ServiceLevelSpec is the spec for a PrometheusServiceLevel.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sloth.slok.dev", version = "v1", kind = "PrometheusServiceLevel", plural = "prometheusservicelevels")]
#[kube(namespaced)]
#[kube(status = "PrometheusServiceLevelStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PrometheusServiceLevelSpec {
    /// Labels are the Prometheus labels that will have all the recording and alerting rules generated for the service SLOs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Service is the application of the SLOs.
    pub service: String,
    /// SLOs are the SLOs of the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slos: Option<Vec<PrometheusServiceLevelSlos>>,
}

/// SLO is the configuration/declaration of the service level objective of a service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlos {
    /// Alerting is the configuration with all the things related with the SLO alerts.
    pub alerting: PrometheusServiceLevelSlosAlerting,
    /// Description is the description of the SLO.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Labels are the Prometheus labels that will have all the recording and alerting rules for this specific SLO. These labels are merged with the previous level labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name is the name of the SLO.
    pub name: String,
    /// Objective is target of the SLO the percentage (0, 100] (e.g 99.9).
    pub objective: f64,
    /// SLI is the indicator (service level indicator) for this specific SLO.
    pub sli: PrometheusServiceLevelSlosSli,
}

/// Alerting is the configuration with all the things related with the SLO alerts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosAlerting {
    /// Annotations are the Prometheus annotations that will have all the alerts generated by this SLO.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels are the Prometheus labels that will have all the alerts generated by this SLO.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name is the name used by the alerts generated for this SLO.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Page alert refers to the critical alert (check multiwindow-multiburn alerts).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pageAlert")]
    pub page_alert: Option<PrometheusServiceLevelSlosAlertingPageAlert>,
    /// TicketAlert alert refers to the warning alert (check multiwindow-multiburn alerts).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ticketAlert")]
    pub ticket_alert: Option<PrometheusServiceLevelSlosAlertingTicketAlert>,
}

/// Page alert refers to the critical alert (check multiwindow-multiburn alerts).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosAlertingPageAlert {
    /// Annotations are the Prometheus annotations for the specific alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Disable disables the alert and makes Sloth not generating this alert. This can be helpful for example to disable ticket(warning) alerts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// Labels are the Prometheus labels for the specific alert. For example can be useful to route the Page alert to specific Slack channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// TicketAlert alert refers to the warning alert (check multiwindow-multiburn alerts).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosAlertingTicketAlert {
    /// Annotations are the Prometheus annotations for the specific alert.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Disable disables the alert and makes Sloth not generating this alert. This can be helpful for example to disable ticket(warning) alerts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// Labels are the Prometheus labels for the specific alert. For example can be useful to route the Page alert to specific Slack channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// SLI is the indicator (service level indicator) for this specific SLO.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosSli {
    /// Events is the events SLI type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<PrometheusServiceLevelSlosSliEvents>,
    /// Plugin is the pluggable SLI type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<PrometheusServiceLevelSlosSliPlugin>,
    /// Raw is the raw SLI type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<PrometheusServiceLevelSlosSliRaw>,
}

/// Events is the events SLI type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosSliEvents {
    /// ErrorQuery is a Prometheus query that will get the number/count of events that we consider that are bad for the SLO (e.g "http 5xx", "latency > 250ms"...). Requires the usage of `{{.window}}` template variable.
    #[serde(rename = "errorQuery")]
    pub error_query: String,
    /// TotalQuery is a Prometheus query that will get the total number/count of events for the SLO (e.g "all http requests"...). Requires the usage of `{{.window}}` template variable.
    #[serde(rename = "totalQuery")]
    pub total_query: String,
}

/// Plugin is the pluggable SLI type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosSliPlugin {
    /// Name is the name of the plugin that needs to load.
    pub id: String,
    /// Options are the options used for the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, String>>,
}

/// Raw is the raw SLI type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelSlosSliRaw {
    /// ErrorRatioQuery is a Prometheus query that will get the raw error ratio (0-1) for the SLO.
    #[serde(rename = "errorRatioQuery")]
    pub error_ratio_query: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrometheusServiceLevelStatus {
    /// LastPromOpRulesGeneration tells the last atemp made for a successful SLO rules generate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPromOpRulesSuccessfulGenerated")]
    pub last_prom_op_rules_successful_generated: Option<String>,
    /// ObservedGeneration tells the generation was acted on, normally this is required to stop an infinite loop when the status is updated because it sends a watch updated event to the watchers of the K8s object.
    #[serde(rename = "observedGeneration")]
    pub observed_generation: i64,
    /// ProcessedSLOs tells how many SLOs haven been processed for Prometheus operator.
    #[serde(rename = "processedSLOs")]
    pub processed_sl_os: i64,
    /// PromOpRulesGenerated tells if the rules for prometheus operator CRD have been generated.
    #[serde(rename = "promOpRulesGenerated")]
    pub prom_op_rules_generated: bool,
    /// PromOpRulesGeneratedSLOs tells how many SLOs have been processed and generated for Prometheus operator successfully.
    #[serde(rename = "promOpRulesGeneratedSLOs")]
    pub prom_op_rules_generated_sl_os: i64,
}

