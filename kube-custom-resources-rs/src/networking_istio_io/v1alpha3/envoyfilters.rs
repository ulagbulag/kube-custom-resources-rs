// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/istio/istio/networking.istio.io/v1alpha3/envoyfilters.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Customizing Envoy configuration generated by Istio. See more details at: https://istio.io/docs/reference/config/networking/envoy-filter.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "EnvoyFilter", plural = "envoyfilters")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EnvoyFilterSpec {
    /// One or more patches with match conditions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configPatches")]
    pub config_patches: Option<Vec<EnvoyFilterConfigPatches>>,
    /// Priority defines the order in which patch sets are applied within a context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRefs")]
    pub target_refs: Option<Vec<EnvoyFilterTargetRefs>>,
    /// Criteria used to select the specific set of pods/VMs on which this patch configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<EnvoyFilterWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatches {
    /// Specifies where in the Envoy configuration, the patch should be applied.
    /// 
    /// Valid Options: LISTENER, FILTER_CHAIN, NETWORK_FILTER, HTTP_FILTER, ROUTE_CONFIGURATION, VIRTUAL_HOST, HTTP_ROUTE, CLUSTER, EXTENSION_CONFIG, BOOTSTRAP, LISTENER_FILTER
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyTo")]
    pub apply_to: Option<EnvoyFilterConfigPatchesApplyTo>,
    /// Match on listener/route configuration/cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<EnvoyFilterConfigPatchesMatch>,
    /// The patch to apply along with the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<EnvoyFilterConfigPatchesPatch>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyFilterConfigPatchesApplyTo {
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "LISTENER")]
    Listener,
    #[serde(rename = "FILTER_CHAIN")]
    FilterChain,
    #[serde(rename = "NETWORK_FILTER")]
    NetworkFilter,
    #[serde(rename = "HTTP_FILTER")]
    HttpFilter,
    #[serde(rename = "ROUTE_CONFIGURATION")]
    RouteConfiguration,
    #[serde(rename = "VIRTUAL_HOST")]
    VirtualHost,
    #[serde(rename = "HTTP_ROUTE")]
    HttpRoute,
    #[serde(rename = "CLUSTER")]
    Cluster,
    #[serde(rename = "EXTENSION_CONFIG")]
    ExtensionConfig,
    #[serde(rename = "BOOTSTRAP")]
    Bootstrap,
    #[serde(rename = "LISTENER_FILTER")]
    ListenerFilter,
}

/// Match on listener/route configuration/cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatch {
    /// Match on envoy cluster attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<EnvoyFilterConfigPatchesMatchCluster>,
    /// The specific config generation context to match on.
    /// 
    /// Valid Options: ANY, SIDECAR_INBOUND, SIDECAR_OUTBOUND, GATEWAY
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<EnvoyFilterConfigPatchesMatchContext>,
    /// Match on envoy listener attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<EnvoyFilterConfigPatchesMatchListener>,
    /// Match on properties associated with a proxy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<EnvoyFilterConfigPatchesMatchProxy>,
    /// Match on envoy HTTP route configuration attributes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeConfiguration")]
    pub route_configuration: Option<EnvoyFilterConfigPatchesMatchRouteConfiguration>,
}

/// Match on envoy cluster attributes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchCluster {
    /// The exact name of the cluster to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The service port for which this cluster was generated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
    /// The fully qualified service name for this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The subset associated with the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Match on listener/route configuration/cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyFilterConfigPatchesMatchContext {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "SIDECAR_INBOUND")]
    SidecarInbound,
    #[serde(rename = "SIDECAR_OUTBOUND")]
    SidecarOutbound,
    #[serde(rename = "GATEWAY")]
    Gateway,
}

/// Match on envoy listener attributes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchListener {
    /// Match a specific filter chain in a listener.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterChain")]
    pub filter_chain: Option<EnvoyFilterConfigPatchesMatchListenerFilterChain>,
    /// Match a specific listener filter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenerFilter")]
    pub listener_filter: Option<String>,
    /// Match a specific listener by its name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portName")]
    pub port_name: Option<String>,
    /// The service port/gateway port to which traffic is being sent/received.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
}

/// Match a specific filter chain in a listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChain {
    /// Applies only to sidecars.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationProtocols")]
    pub application_protocols: Option<String>,
    /// The destination_port value used by a filter chain's match condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationPort")]
    pub destination_port: Option<i64>,
    /// The name of a specific filter to apply the patch to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<EnvoyFilterConfigPatchesMatchListenerFilterChainFilter>,
    /// The name assigned to the filter chain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The SNI value used by a filter chain's match condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    /// Applies only to `SIDECAR_INBOUND` context.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
}

/// The name of a specific filter to apply the patch to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChainFilter {
    /// The filter name to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The next level filter within this filter to match upon.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subFilter")]
    pub sub_filter: Option<EnvoyFilterConfigPatchesMatchListenerFilterChainFilterSubFilter>,
}

/// The next level filter within this filter to match upon.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChainFilterSubFilter {
    /// The filter name to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Match on properties associated with a proxy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchProxy {
    /// Match on the node metadata supplied by a proxy when connecting to Istio Pilot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// A regular expression in golang regex format (RE2) that can be used to select proxies using a specific version of istio proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyVersion")]
    pub proxy_version: Option<String>,
}

/// Match on envoy HTTP route configuration attributes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfiguration {
    /// The Istio gateway config's namespace/name for which this route configuration was generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Route configuration name to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Applicable only for GATEWAY context.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portName")]
    pub port_name: Option<String>,
    /// The service port number or gateway server port number for which this route configuration was generated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
    /// Match a specific virtual host in a route configuration and apply the patch to the virtual host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhost: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhost>,
}

/// Match a specific virtual host in a route configuration and apply the patch to the virtual host.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfigurationVhost {
    /// The VirtualHosts objects generated by Istio are named as host:port, where the host typically corresponds to the VirtualService's host field or the hostname of a service in the registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Match a specific route within the virtual host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRoute>,
}

/// Match a specific route within the virtual host.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRoute {
    /// Match a route with specific action type.
    /// 
    /// Valid Options: ANY, ROUTE, REDIRECT, DIRECT_RESPONSE
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRouteAction>,
    /// The Route objects generated by default are named as default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Match a specific route within the virtual host.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRouteAction {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ROUTE")]
    Route,
    #[serde(rename = "REDIRECT")]
    Redirect,
    #[serde(rename = "DIRECT_RESPONSE")]
    DirectResponse,
}

/// The patch to apply along with the operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterConfigPatchesPatch {
    /// Determines the filter insertion order.
    /// 
    /// Valid Options: AUTHN, AUTHZ, STATS
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterClass")]
    pub filter_class: Option<EnvoyFilterConfigPatchesPatchFilterClass>,
    /// Determines how the patch should be applied.
    /// 
    /// Valid Options: MERGE, ADD, REMOVE, INSERT_BEFORE, INSERT_AFTER, INSERT_FIRST, REPLACE
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<EnvoyFilterConfigPatchesPatchOperation>,
    /// The JSON config of the object being patched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, serde_json::Value>>,
}

/// The patch to apply along with the operation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyFilterConfigPatchesPatchFilterClass {
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "AUTHN")]
    Authn,
    #[serde(rename = "AUTHZ")]
    Authz,
    #[serde(rename = "STATS")]
    Stats,
}

/// The patch to apply along with the operation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyFilterConfigPatchesPatchOperation {
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "INSERT_BEFORE")]
    InsertBefore,
    #[serde(rename = "INSERT_AFTER")]
    InsertAfter,
    #[serde(rename = "INSERT_FIRST")]
    InsertFirst,
    #[serde(rename = "REPLACE")]
    Replace,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterTargetRefs {
    /// group is the group of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// kind is kind of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// name is the name of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace is the namespace of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Criteria used to select the specific set of pods/VMs on which this patch configuration should be applied.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyFilterWorkloadSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which the configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

