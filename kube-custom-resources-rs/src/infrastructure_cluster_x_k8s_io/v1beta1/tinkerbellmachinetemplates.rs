// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tinkerbell/cluster-api-provider-tinkerbell/infrastructure.cluster.x-k8s.io/v1beta1/tinkerbellmachinetemplates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// TinkerbellMachineTemplateSpec defines the desired state of TinkerbellMachineTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "TinkerbellMachineTemplate", plural = "tinkerbellmachinetemplates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TinkerbellMachineTemplateSpec {
    /// TinkerbellMachineTemplateResource describes the data needed to create am TinkerbellMachine from a template.
    pub template: TinkerbellMachineTemplateTemplate,
}

/// TinkerbellMachineTemplateResource describes the data needed to create am TinkerbellMachine from a template.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplate {
    /// Spec is the specification of the desired behavior of the machine.
    pub spec: TinkerbellMachineTemplateTemplateSpec,
}

/// Spec is the specification of the desired behavior of the machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpec {
    /// HardwareAffinity allows filtering for hardware.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hardwareAffinity")]
    pub hardware_affinity: Option<TinkerbellMachineTemplateTemplateSpecHardwareAffinity>,
    /// Those fields are set programmatically, but they cannot be re-constructed from "state of the world", so
    /// we put them in spec instead of status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hardwareName")]
    pub hardware_name: Option<String>,
    /// ImageLookupBaseRegistry is the base Registry URL that is used for pulling images,
    /// if not set, the default will be to use ghcr.io/tinkerbell/cluster-api-provider-tinkerbell.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupBaseRegistry")]
    pub image_lookup_base_registry: Option<String>,
    /// ImageLookupFormat is the URL naming format to use for machine images when
    /// a machine does not specify. When set, this will be used for all cluster machines
    /// unless a machine specifies a different ImageLookupFormat. Supports substitutions
    /// for {{.BaseRegistry}}, {{.OSDistro}}, {{.OSVersion}} and {{.KubernetesVersion}} with
    /// the basse URL, OS distribution, OS version, and kubernetes version, respectively.
    /// BaseRegistry will be the value in ImageLookupBaseRegistry or ghcr.io/tinkerbell/cluster-api-provider-tinkerbell
    /// (the default), OSDistro will be the value in ImageLookupOSDistro or ubuntu (the default),
    /// OSVersion will be the value in ImageLookupOSVersion or default based on the OSDistro
    /// (if known), and the kubernetes version as defined by the packages produced by
    /// kubernetes/release: v1.13.0, v1.12.5-mybuild.1, or v1.17.3. For example, the default
    /// image format of {{.BaseRegistry}}/{{.OSDistro}}-{{.OSVersion}}:{{.KubernetesVersion}}.gz will
    /// attempt to pull the image from that location. See also: https://golang.org/pkg/text/template/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupFormat")]
    pub image_lookup_format: Option<String>,
    /// ImageLookupOSDistro is the name of the OS distro to use when fetching machine images,
    /// if not set it will default to ubuntu.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupOSDistro")]
    pub image_lookup_os_distro: Option<String>,
    /// ImageLookupOSVersion is the version of the OS distribution to use when fetching machine
    /// images. If not set it will default based on ImageLookupOSDistro.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageLookupOSVersion")]
    pub image_lookup_os_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    /// TemplateOverride overrides the default Tinkerbell template used by CAPT.
    /// You can learn more about Tinkerbell templates here: https://tinkerbell.org/docs/concepts/templates/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateOverride")]
    pub template_override: Option<String>,
}

/// HardwareAffinity allows filtering for hardware.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinity {
    /// Preferred are the preferred hardware affinity terms. Hardware matching these terms are preferred according to the
    /// weights provided, but are not required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred: Option<Vec<TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferred>>,
    /// Required are the required hardware affinity terms.  The terms are OR'd together, hardware must match one term to
    /// be considered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<TinkerbellMachineTemplateTemplateSpecHardwareAffinityRequired>>,
}

/// WeightedHardwareAffinityTerm is a HardwareAffinityTerm with an associated weight.  The weights of all the matched
/// WeightedHardwareAffinityTerm fields are added per-hardware to find the most preferred hardware.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferred {
    /// HardwareAffinityTerm is the term associated with the corresponding weight.
    #[serde(rename = "hardwareAffinityTerm")]
    pub hardware_affinity_term: TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferredHardwareAffinityTerm,
    /// Weight associated with matching the corresponding hardwareAffinityTerm, in the range 1-100.
    pub weight: i32,
}

/// HardwareAffinityTerm is the term associated with the corresponding weight.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferredHardwareAffinityTerm {
    /// LabelSelector is used to select for particular hardware by label.
    #[serde(rename = "labelSelector")]
    pub label_selector: TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferredHardwareAffinityTermLabelSelector,
}

/// LabelSelector is used to select for particular hardware by label.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferredHardwareAffinityTermLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferredHardwareAffinityTermLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityPreferredHardwareAffinityTermLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// HardwareAffinityTerm is used to select for a particular existing hardware resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityRequired {
    /// LabelSelector is used to select for particular hardware by label.
    #[serde(rename = "labelSelector")]
    pub label_selector: TinkerbellMachineTemplateTemplateSpecHardwareAffinityRequiredLabelSelector,
}

/// LabelSelector is used to select for particular hardware by label.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityRequiredLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TinkerbellMachineTemplateTemplateSpecHardwareAffinityRequiredLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TinkerbellMachineTemplateTemplateSpecHardwareAffinityRequiredLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

