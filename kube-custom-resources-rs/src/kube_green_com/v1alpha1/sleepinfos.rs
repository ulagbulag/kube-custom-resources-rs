// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kube-green/kube-green/kube-green.com/v1alpha1/sleepinfos.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// SleepInfoSpec defines the desired state of SleepInfo
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kube-green.com", version = "v1alpha1", kind = "SleepInfo", plural = "sleepinfos")]
#[kube(namespaced)]
#[kube(status = "SleepInfoStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SleepInfoSpec {
    /// ExcludeRef define the resource to exclude from the sleep.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeRef")]
    pub exclude_ref: Option<Vec<SleepInfoExcludeRef>>,
    /// IncludeRef define the resource to include from the sleep.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeRef")]
    pub include_ref: Option<Vec<SleepInfoIncludeRef>>,
    /// Patches is a list of json 6902 patches to apply to the target resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<SleepInfoPatches>>,
    /// Hours:Minutes
    /// 
    /// 
    /// Accept cron schedule for both hour and minute.
    /// For example, *:*/2 is set to configure a run every even minute.
    #[serde(rename = "sleepAt")]
    pub sleep_at: String,
    /// If SuspendCronjobs is set to true, on sleep the cronjobs of the namespace will be suspended.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "suspendCronJobs")]
    pub suspend_cron_jobs: Option<bool>,
    /// If SuspendDeployments is set to false, on sleep the deployment of the namespace will not be suspended. By default Deployment will be suspended.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "suspendDeployments")]
    pub suspend_deployments: Option<bool>,
    /// If SuspendStatefulSets is set to false, on sleep the statefulset of the namespace will not be suspended. By default StatefulSet will be suspended.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "suspendStatefulsets")]
    pub suspend_statefulsets: Option<bool>,
    /// Time zone to set the schedule, in IANA time zone identifier.
    /// It is not required, default to UTC.
    /// For example, for the Italy time zone set Europe/Rome.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
    /// Hours:Minutes
    /// 
    /// 
    /// Accept cron schedule for both hour and minute.
    /// For example, *:*/2 is set to configure a run every even minute.
    /// It is not required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wakeUpAt")]
    pub wake_up_at: Option<String>,
    /// Weekdays are in cron notation.
    /// 
    /// 
    /// For example, to configure a schedule from monday to friday, set it to "1-5"
    pub weekdays: String,
}

/// Common type to use for both IncludeRef and ExcludeRef to prevent duplication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SleepInfoExcludeRef {
    /// ApiVersion of the kubernetes resources.
    /// Supported api version is "apps/v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the kubernetes resources of the specific version.
    /// Supported kind are "Deployment" and "CronJob".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// MatchLabels which identify the kubernetes resource by labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// Name which identify the kubernetes resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Common type to use for both IncludeRef and ExcludeRef to prevent duplication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SleepInfoIncludeRef {
    /// ApiVersion of the kubernetes resources.
    /// Supported api version is "apps/v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the kubernetes resources of the specific version.
    /// Supported kind are "Deployment" and "CronJob".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// MatchLabels which identify the kubernetes resource by labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// Name which identify the kubernetes resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SleepInfoPatches {
    /// Patch is the json6902 patch to apply to the target resource.
    pub patch: String,
    /// Target is the target resource to patch.
    pub target: SleepInfoPatchesTarget,
}

/// Target is the target resource to patch.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SleepInfoPatchesTarget {
    /// Group of the Kubernetes resources.
    pub group: String,
    /// Kind of the Kubernetes resources.
    pub kind: String,
}

/// SleepInfoStatus defines the observed state of SleepInfo
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SleepInfoStatus {
    /// Information when was the last time the run was successfully scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScheduleTime")]
    pub last_schedule_time: Option<String>,
    /// The operation type handled in last schedule. SLEEP or WAKE_UP are the
    /// possibilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

