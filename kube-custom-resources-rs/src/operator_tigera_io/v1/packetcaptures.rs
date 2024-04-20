// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1/packetcaptures.yaml --derive=PartialEq
// kopium version: 0.18.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

/// Specification of the desired state for the PacketCapture.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "PacketCapture", plural = "packetcaptures")]
#[kube(status = "PacketCaptureStatus")]
#[kube(schema = "disabled")]
pub struct PacketCaptureSpec {
    /// PacketCaptureDeployment configures the PacketCapture Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "packetCaptureDeployment")]
    pub packet_capture_deployment: Option<PacketCapturePacketCaptureDeployment>,
}

/// PacketCaptureDeployment configures the PacketCapture Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeployment {
    /// Spec is the specification of the PacketCapture Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PacketCapturePacketCaptureDeploymentSpec>,
}

/// Spec is the specification of the PacketCapture Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpec {
    /// Template describes the PacketCapture Deployment pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PacketCapturePacketCaptureDeploymentSpecTemplate>,
}

/// Template describes the PacketCapture Deployment pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpecTemplate {
    /// Spec is the PacketCapture Deployment's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PacketCapturePacketCaptureDeploymentSpecTemplateSpec>,
}

/// Spec is the PacketCapture Deployment's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpec {
    /// Containers is a list of PacketCapture containers.
    /// If specified, this overrides the specified PacketCapture Deployment containers.
    /// If omitted, the PacketCapture Deployment will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainers>>,
    /// InitContainers is a list of PacketCapture init containers.
    /// If specified, this overrides the specified PacketCapture Deployment init containers.
    /// If omitted, the PacketCapture Deployment will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainers>>,
}

/// PacketCaptureDeploymentContainer is a PacketCapture Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainers {
    /// Name is an enum which identifies the PacketCapture Deployment container by name.
    /// Supported values are: tigera-packetcapture-server
    pub name: PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named PacketCapture Deployment container's resources.
    /// If omitted, the PacketCapture Deployment will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainersResources>,
}

/// PacketCaptureDeploymentContainer is a PacketCapture Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainersName {
    #[serde(rename = "tigera-packetcapture-server")]
    TigeraPacketcaptureServer,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named PacketCapture Deployment container's resources.
/// If omitted, the PacketCapture Deployment will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainersResourcesClaims>>,
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
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpecContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// PacketCaptureDeploymentInitContainer is a PacketCapture Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the PacketCapture Deployment init container by name.
    /// Supported values are: tigera-packetcapture-server-tls-key-cert-provisioner
    pub name: PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named PacketCapture Deployment init container's resources.
    /// If omitted, the PacketCapture Deployment will use its default value for this init container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainersResources>,
}

/// PacketCaptureDeploymentInitContainer is a PacketCapture Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainersName {
    #[serde(rename = "tigera-packetcapture-server-tls-key-cert-provisioner")]
    TigeraPacketcaptureServerTlsKeyCertProvisioner,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named PacketCapture Deployment init container's resources.
/// If omitted, the PacketCapture Deployment will use its default value for this init container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainersResourcesClaims>>,
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
pub struct PacketCapturePacketCaptureDeploymentSpecTemplateSpecInitContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// Most recently observed state for the PacketCapture.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PacketCaptureStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of
    /// Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

