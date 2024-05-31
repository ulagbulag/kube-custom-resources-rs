// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/apecloud/kubeblocks/apps.kubeblocks.io/v1alpha1/configurations.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ConfigurationSpec defines the desired state of a Configuration resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.kubeblocks.io", version = "v1alpha1", kind = "Configuration", plural = "configurations")]
#[kube(namespaced)]
#[kube(status = "ConfigurationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ConfigurationSpec {
    /// Specifies the name of the Cluster that this configuration is associated with.
    #[serde(rename = "clusterRef")]
    pub cluster_ref: String,
    /// Represents the name of the Component that this configuration pertains to.
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// ConfigItemDetails is an array of ConfigurationItemDetail objects. 
    ///  Each ConfigurationItemDetail corresponds to a configuration template, which is a ConfigMap that contains multiple configuration files. Each configuration file is stored as a key-value pair within the ConfigMap. 
    ///  The ConfigurationItemDetail includes information such as: 
    ///  - The configuration template (a ConfigMap) - The corresponding ConfigConstraint (constraints and validation rules for the configuration) - Volume mounts (for mounting the configuration files)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configItemDetails")]
    pub config_item_details: Option<Vec<ConfigurationConfigItemDetails>>,
}

/// ConfigurationItemDetail corresponds to settings of a configuration template (a ConfigMap).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetails {
    /// Specifies the user-defined configuration parameters. 
    ///  When provided, the parameter values in `configFileParams` override the default configuration parameters. This allows users to override the default configuration according to their specific needs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configFileParams")]
    pub config_file_params: Option<BTreeMap<String, ConfigurationConfigItemDetailsConfigFileParams>>,
    /// Specifies the name of the configuration template (a ConfigMap), ConfigConstraint, and other miscellaneous options. 
    ///  The configuration template is a ConfigMap that contains multiple configuration files. Each configuration file is stored as a key-value pair within the ConfigMap. 
    ///  ConfigConstraint allows defining constraints and validation rules for configuration parameters. It ensures that the configuration adheres to certain requirements and limitations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configSpec")]
    pub config_spec: Option<ConfigurationConfigItemDetailsConfigSpec>,
    /// Specifies the user-defined configuration template. 
    ///  When provided, the `importTemplateRef` overrides the default configuration template specified in `configSpec.templateRef`. This allows users to customize the configuration template according to their specific requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "importTemplateRef")]
    pub import_template_ref: Option<ConfigurationConfigItemDetailsImportTemplateRef>,
    /// Defines the unique identifier of the configuration template. 
    ///  It must be a string of maximum 63 characters, and can only include lowercase alphanumeric characters, hyphens, and periods. The name must start and end with an alphanumeric character.
    pub name: String,
    /// External controllers can trigger a configuration rerender by modifying this field. 
    ///  Note: Currently, the `payload` field is opaque and its content is not interpreted by the system. Modifying this field will cause a rerender, regardless of the specific content of this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<BTreeMap<String, serde_json::Value>>,
    /// Deprecated: No longer used. Please use 'Payload' instead. Previously represented the version of the configuration template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Specifies the user-defined configuration parameters. 
///  When provided, the parameter values in `configFileParams` override the default configuration parameters. This allows users to override the default configuration according to their specific needs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsConfigFileParams {
    /// Holds the configuration keys and values. This field is a workaround for issues found in kubebuilder and code-generator. Refer to https://github.com/kubernetes-sigs/kubebuilder/issues/528 and https://github.com/kubernetes/code-generator/issues/50 for more details. 
    ///  Represents the content of the configuration file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Represents the updated parameters for a single configuration file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
}

/// Specifies the name of the configuration template (a ConfigMap), ConfigConstraint, and other miscellaneous options. 
///  The configuration template is a ConfigMap that contains multiple configuration files. Each configuration file is stored as a key-value pair within the ConfigMap. 
///  ConfigConstraint allows defining constraints and validation rules for configuration parameters. It ensures that the configuration adheres to certain requirements and limitations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsConfigSpec {
    /// Specifies the containers to inject the ConfigMap parameters as environment variables. 
    ///  This is useful when application images accept parameters through environment variables and generate the final configuration file in the startup script based on these variables. 
    ///  This field allows users to specify a list of container names, and KubeBlocks will inject the environment variables converted from the ConfigMap into these designated containers. This provides a flexible way to pass the configuration items from the ConfigMap to the container without modifying the image. 
    ///  Deprecated: `asEnvFrom` has been deprecated since 0.9.0 and will be removed in 0.10.0. Use `injectEnvTo` instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "asEnvFrom")]
    pub as_env_from: Option<Vec<String>>,
    /// Specifies the name of the referenced configuration constraints object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintRef")]
    pub constraint_ref: Option<String>,
    /// The operator attempts to set default file permissions for scripts (0555) and configurations (0444). However, certain database engines may require different file permissions. You can specify the desired file permissions here. 
    ///  Must be specified as an octal value between 0000 and 0777 (inclusive), or as a decimal value between 0 and 511 (inclusive). YAML supports both octal and decimal values for file permissions. 
    ///  Please note that this setting only affects the permissions of the files themselves. Directories within the specified path are not impacted by this setting. It's important to be aware that this setting might conflict with other options that influence the file mode, such as fsGroup. In such cases, the resulting file mode may have additional bits set. Refers to documents of k8s.ConfigMapVolumeSource.defaultMode for more information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultMode")]
    pub default_mode: Option<i32>,
    /// Specifies the containers to inject the ConfigMap parameters as environment variables. 
    ///  This is useful when application images accept parameters through environment variables and generate the final configuration file in the startup script based on these variables. 
    ///  This field allows users to specify a list of container names, and KubeBlocks will inject the environment variables converted from the ConfigMap into these designated containers. This provides a flexible way to pass the configuration items from the ConfigMap to the container without modifying the image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "injectEnvTo")]
    pub inject_env_to: Option<Vec<String>>,
    /// Specifies the configuration files within the ConfigMap that support dynamic updates. 
    ///  A configuration template (provided in the form of a ConfigMap) may contain templates for multiple configuration files. Each configuration file corresponds to a key in the ConfigMap. Some of these configuration files may support dynamic modification and reloading without requiring a pod restart. 
    ///  If empty or omitted, all configuration files in the ConfigMap are assumed to support dynamic updates, and ConfigConstraint applies to all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    /// Specifies the secondary rendered config spec for pod-specific customization. 
    ///  The template is rendered inside the pod (by the "config-manager" sidecar container) and merged with the main template's render result to generate the final configuration file. 
    ///  This field is intended to handle scenarios where different pods within the same Component have varying configurations. It allows for pod-specific customization of the configuration. 
    ///  Note: This field will be deprecated in future versions, and the functionality will be moved to `cluster.spec.componentSpecs[*].instances[*]`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "legacyRenderedConfigSpec")]
    pub legacy_rendered_config_spec: Option<ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpec>,
    /// Specifies the name of the configuration template.
    pub name: String,
    /// Specifies the namespace of the referenced configuration template ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specifies whether the configuration needs to be re-rendered after v-scale or h-scale operations to reflect changes. 
    ///  In some scenarios, the configuration may need to be updated to reflect the changes in resource allocation or cluster topology. Examples: 
    ///  - Redis: adjust maxmemory after v-scale operation. - MySQL: increase max connections after v-scale operation. - Zookeeper: update zoo.cfg with new node addresses after h-scale operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reRenderResourceTypes")]
    pub re_render_resource_types: Option<Vec<String>>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateRef")]
    pub template_ref: Option<String>,
    /// Refers to the volume name of PodTemplate. The configuration file produced through the configuration template will be mounted to the corresponding volume. Must be a DNS_LABEL name. The volume name must be defined in podSpec.containers[*].volumeMounts.
    #[serde(rename = "volumeName")]
    pub volume_name: String,
}

/// Specifies the secondary rendered config spec for pod-specific customization. 
///  The template is rendered inside the pod (by the "config-manager" sidecar container) and merged with the main template's render result to generate the final configuration file. 
///  This field is intended to handle scenarios where different pods within the same Component have varying configurations. It allows for pod-specific customization of the configuration. 
///  Note: This field will be deprecated in future versions, and the functionality will be moved to `cluster.spec.componentSpecs[*].instances[*]`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpec {
    /// Specifies the namespace of the referenced configuration template ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Defines the strategy for merging externally imported templates into component templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpecPolicy>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(rename = "templateRef")]
    pub template_ref: String,
}

/// Specifies the secondary rendered config spec for pod-specific customization. 
///  The template is rendered inside the pod (by the "config-manager" sidecar container) and merged with the main template's render result to generate the final configuration file. 
///  This field is intended to handle scenarios where different pods within the same Component have varying configurations. It allows for pod-specific customization of the configuration. 
///  Note: This field will be deprecated in future versions, and the functionality will be moved to `cluster.spec.componentSpecs[*].instances[*]`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationConfigItemDetailsConfigSpecLegacyRenderedConfigSpecPolicy {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "none")]
    None,
}

/// Specifies the user-defined configuration template. 
///  When provided, the `importTemplateRef` overrides the default configuration template specified in `configSpec.templateRef`. This allows users to customize the configuration template according to their specific requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationConfigItemDetailsImportTemplateRef {
    /// Specifies the namespace of the referenced configuration template ConfigMap object. An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Defines the strategy for merging externally imported templates into component templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ConfigurationConfigItemDetailsImportTemplateRefPolicy>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(rename = "templateRef")]
    pub template_ref: String,
}

/// Specifies the user-defined configuration template. 
///  When provided, the `importTemplateRef` overrides the default configuration template specified in `configSpec.templateRef`. This allows users to customize the configuration template according to their specific requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationConfigItemDetailsImportTemplateRefPolicy {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "none")]
    None,
}

/// ConfigurationStatus represents the observed state of a Configuration resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationStatus {
    /// Provides detailed status information for opsRequest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Provides the status of each component undergoing reconfiguration.
    #[serde(rename = "configurationStatus")]
    pub configuration_status: Vec<ConfigurationStatusConfigurationStatus>,
    /// Provides a description of any abnormal status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Represents the latest generation observed for this ClusterDefinition. It corresponds to the ConfigConstraint's generation, which is updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationStatusConfigurationStatus {
    /// Represents the last completed revision of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastDoneRevision")]
    pub last_done_revision: Option<String>,
    /// Provides a description of any abnormal status. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Specifies the name of the configuration template. It is a required field and must be a string of maximum 63 characters. The name should only contain lowercase alphanumeric characters, hyphens, or periods. It should start and end with an alphanumeric character.
    pub name: String,
    /// Indicates the current status of the configuration item. 
    ///  Possible values include "Creating", "Init", "Running", "Pending", "Merged", "MergeFailed", "FailedAndPause", "Upgrading", "Deleting", "FailedAndRetry", "Finished".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ConfigurationStatusConfigurationStatusPhase>,
    /// Provides detailed information about the execution of the configuration change. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcileDetail")]
    pub reconcile_detail: Option<ConfigurationStatusConfigurationStatusReconcileDetail>,
    /// Represents the updated revision of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateRevision")]
    pub update_revision: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ConfigurationStatusConfigurationStatusPhase {
    Creating,
    Init,
    Running,
    Pending,
    Merged,
    MergeFailed,
    FailedAndPause,
    Upgrading,
    Deleting,
    FailedAndRetry,
    Finished,
}

/// Provides detailed information about the execution of the configuration change. This field is optional.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConfigurationStatusConfigurationStatusReconcileDetail {
    /// Represents the current revision of the configuration item.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentRevision")]
    pub current_revision: Option<String>,
    /// Represents the error message generated when the execution of configuration changes fails.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errMessage")]
    pub err_message: Option<String>,
    /// Represents the outcome of the most recent execution.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "execResult")]
    pub exec_result: Option<String>,
    /// Represents the total number of pods that require execution of configuration changes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expectedCount")]
    pub expected_count: Option<i32>,
    /// Represents the policy applied during the most recent execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Represents the number of pods where configuration changes were successfully applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "succeedCount")]
    pub succeed_count: Option<i32>,
}

