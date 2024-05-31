// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/clusternet/clusternet/apps.clusternet.io/v1alpha1/subscriptions.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// SubscriptionSpec defines the desired state of Subscription
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "apps.clusternet.io", version = "v1alpha1", kind = "Subscription", plural = "subscriptions")]
#[kube(namespaced)]
#[kube(status = "SubscriptionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct SubscriptionSpec {
    /// ClusterTolerations tolerates any matched taints of ManagedCluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterTolerations")]
    pub cluster_tolerations: Option<Vec<SubscriptionClusterTolerations>>,
    /// Dividing scheduling config params. Present only if SchedulingStrategy = Dividing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dividingScheduling")]
    pub dividing_scheduling: Option<SubscriptionDividingScheduling>,
    /// Feeds
    pub feeds: Vec<SubscriptionFeeds>,
    /// PreemptionPolicy is the Policy for preempting subscriptions with lower priority.
    /// One of Never, PreemptLowerPriority.
    /// Defaults to PreemptLowerPriority if unset.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preemptionPolicy")]
    pub preemption_policy: Option<SubscriptionPreemptionPolicy>,
    /// The priority value. clusternet-scheduler use this field to find the
    /// priority of the subscription.
    /// The higher the value, the higher the priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// If specified, the Subscription will be handled by specified scheduler.
    /// If not specified, the Subscription will be handled by default scheduler.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulerName")]
    pub scheduler_name: Option<String>,
    /// If specified, the Subscription will be handled with SchedulingBySubGroup.
    /// Used together with SubGroupStrategy in every Subscriber.
    /// Can work with all supported SchedulingStrategy, such as Replication, Dividing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulingBySubGroup")]
    pub scheduling_by_sub_group: Option<bool>,
    /// If specified, the Subscription will be handled with specified SchedulingStrategy.
    /// Otherwise, with generic SchedulingStrategy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulingStrategy")]
    pub scheduling_strategy: Option<SubscriptionSchedulingStrategy>,
    /// Subscribers subscribes
    pub subscribers: Vec<SubscriptionSubscribers>,
}

/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionClusterTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects.
    /// When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys.
    /// If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are Exists and Equal. Defaults to Equal.
    /// Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be
    /// of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default,
    /// it is not set, which means tolerate the taint forever (do not evict). Zero and
    /// negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to.
    /// If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Dividing scheduling config params. Present only if SchedulingStrategy = Dividing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingScheduling {
    /// DynamicDividing describes how to divide replicas into target clusters dynamically.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicDividing")]
    pub dynamic_dividing: Option<SubscriptionDividingSchedulingDynamicDividing>,
    /// Type of dividing replica scheduling.
    #[serde(rename = "type")]
    pub r#type: SubscriptionDividingSchedulingType,
}

/// DynamicDividing describes how to divide replicas into target clusters dynamically.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividing {
    /// MaxClusters describes the upper bound number of target clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxClusters")]
    pub max_clusters: Option<i32>,
    /// MinClusters describes the lower bound number of target clusters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minClusters")]
    pub min_clusters: Option<i32>,
    /// PreferredClusters describes the assigning preference. If we have a preference for cluster group A
    /// compared to cluster group B (i.e., group A has a larger Weight), desired replicas will be assigned
    /// to cluster group A as many as possible, while the rest ones will be assigned to cluster group B.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredClusters")]
    pub preferred_clusters: Option<Vec<SubscriptionDividingSchedulingDynamicDividingPreferredClusters>>,
    /// Type of dynamic dividing replica strategy.
    pub strategy: SubscriptionDividingSchedulingDynamicDividingStrategy,
    /// TopologySpreadConstraints describes how a group of replicas ought to spread across topology
    /// domains. Scheduler will schedule pods in a way which abides by the constraints.
    /// All topologySpreadConstraints are ANDed.
    /// Present only for spread divided scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologySpreadConstraints")]
    pub topology_spread_constraints: Option<Vec<SubscriptionDividingSchedulingDynamicDividingTopologySpreadConstraints>>,
}

/// An empty preferred scheduling term matches all objects with implicit weight 0
/// (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingPreferredClusters {
    /// A node selector term, associated with the corresponding weight.
    pub preference: SubscriptionDividingSchedulingDynamicDividingPreferredClustersPreference,
    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    pub weight: i32,
}

/// A node selector term, associated with the corresponding weight.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingPreferredClustersPreference {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SubscriptionDividingSchedulingDynamicDividingPreferredClustersPreferenceMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<SubscriptionDividingSchedulingDynamicDividingPreferredClustersPreferenceMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingPreferredClustersPreferenceMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingPreferredClustersPreferenceMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. If the operator is Gt or Lt, the values
    /// array must have a single element, which will be interpreted as an integer.
    /// This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// DynamicDividing describes how to divide replicas into target clusters dynamically.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionDividingSchedulingDynamicDividingStrategy {
    Spread,
    Binpack,
}

/// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingTopologySpreadConstraints {
    /// LabelSelector is used to find matching pods.
    /// Pods that match this label selector are counted to determine the number of pods
    /// in their corresponding topology domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<SubscriptionDividingSchedulingDynamicDividingTopologySpreadConstraintsLabelSelector>,
    /// MatchLabelKeys is a set of pod label keys to select the pods over which
    /// spreading will be calculated. The keys are used to lookup values from the
    /// incoming pod labels, those key-value labels are ANDed with labelSelector
    /// to select the group of existing pods over which spreading will be calculated
    /// for the incoming pod. The same key is forbidden to exist in both MatchLabelKeys and LabelSelector.
    /// MatchLabelKeys cannot be set when LabelSelector isn't set.
    /// Keys that don't exist in the incoming pod labels will
    /// be ignored. A null or empty list means only match against labelSelector.
    /// 
    /// 
    /// This is a beta field and requires the MatchLabelKeysInPodTopologySpread feature gate to be enabled (enabled by default).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    /// MaxSkew describes the degree to which pods may be unevenly distributed.
    /// When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference
    /// between the number of matching pods in the target topology and the global minimum.
    /// The global minimum is the minimum number of matching pods in an eligible domain
    /// or zero if the number of eligible domains is less than MinDomains.
    /// For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same
    /// labelSelector spread as 2/2/1:
    /// In this case, the global minimum is 1.
    /// | zone1 | zone2 | zone3 |
    /// |  P P  |  P P  |   P   |
    /// - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 2/2/2;
    /// scheduling it onto zone1(zone2) would make the ActualSkew(3-1) on zone1(zone2)
    /// violate MaxSkew(1).
    /// - if MaxSkew is 2, incoming pod can be scheduled onto any zone.
    /// When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence
    /// to topologies that satisfy it.
    /// It's a required field. Default value is 1 and 0 is not allowed.
    #[serde(rename = "maxSkew")]
    pub max_skew: i32,
    /// MinDomains indicates a minimum number of eligible domains.
    /// When the number of eligible domains with matching topology keys is less than minDomains,
    /// Pod Topology Spread treats "global minimum" as 0, and then the calculation of Skew is performed.
    /// And when the number of eligible domains with matching topology keys equals or greater than minDomains,
    /// this value has no effect on scheduling.
    /// As a result, when the number of eligible domains is less than minDomains,
    /// scheduler won't schedule more than maxSkew Pods to those domains.
    /// If value is nil, the constraint behaves as if MinDomains is equal to 1.
    /// Valid values are integers greater than 0.
    /// When value is not nil, WhenUnsatisfiable must be DoNotSchedule.
    /// 
    /// 
    /// For example, in a 3-zone cluster, MaxSkew is set to 2, MinDomains is set to 5 and pods with the same
    /// labelSelector spread as 2/2/2:
    /// | zone1 | zone2 | zone3 |
    /// |  P P  |  P P  |  P P  |
    /// The number of domains is less than 5(MinDomains), so "global minimum" is treated as 0.
    /// In this situation, new pod with the same labelSelector cannot be scheduled,
    /// because computed skew will be 3(3 - 0) if new Pod is scheduled to any of the three zones,
    /// it will violate MaxSkew.
    /// 
    /// 
    /// This is a beta field and requires the MinDomainsInPodTopologySpread feature gate to be enabled (enabled by default).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minDomains")]
    pub min_domains: Option<i32>,
    /// NodeAffinityPolicy indicates how we will treat Pod's nodeAffinity/nodeSelector
    /// when calculating pod topology spread skew. Options are:
    /// - Honor: only nodes matching nodeAffinity/nodeSelector are included in the calculations.
    /// - Ignore: nodeAffinity/nodeSelector are ignored. All nodes are included in the calculations.
    /// 
    /// 
    /// If this value is nil, the behavior is equivalent to the Honor policy.
    /// This is a beta-level feature default enabled by the NodeInclusionPolicyInPodTopologySpread feature flag.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinityPolicy")]
    pub node_affinity_policy: Option<String>,
    /// NodeTaintsPolicy indicates how we will treat node taints when calculating
    /// pod topology spread skew. Options are:
    /// - Honor: nodes without taints, along with tainted nodes for which the incoming pod
    /// has a toleration, are included.
    /// - Ignore: node taints are ignored. All nodes are included.
    /// 
    /// 
    /// If this value is nil, the behavior is equivalent to the Ignore policy.
    /// This is a beta-level feature default enabled by the NodeInclusionPolicyInPodTopologySpread feature flag.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeTaintsPolicy")]
    pub node_taints_policy: Option<String>,
    /// TopologyKey is the key of node labels. Nodes that have a label with this key
    /// and identical values are considered to be in the same topology.
    /// We consider each <key, value> as a "bucket", and try to put balanced number
    /// of pods into each bucket.
    /// We define a domain as a particular instance of a topology.
    /// Also, we define an eligible domain as a domain whose nodes meet the requirements of
    /// nodeAffinityPolicy and nodeTaintsPolicy.
    /// e.g. If TopologyKey is "kubernetes.io/hostname", each Node is a domain of that topology.
    /// And, if TopologyKey is "topology.kubernetes.io/zone", each zone is a domain of that topology.
    /// It's a required field.
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
    /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy
    /// the spread constraint.
    /// - DoNotSchedule (default) tells the scheduler not to schedule it.
    /// - ScheduleAnyway tells the scheduler to schedule the pod in any location,
    ///   but giving higher precedence to topologies that would help reduce the
    ///   skew.
    /// A constraint is considered "Unsatisfiable" for an incoming pod
    /// if and only if every possible node assignment for that pod would violate
    /// "MaxSkew" on some topology.
    /// For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same
    /// labelSelector spread as 3/1/1:
    /// | zone1 | zone2 | zone3 |
    /// | P P P |   P   |   P   |
    /// If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled
    /// to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies
    /// MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler
    /// won't make it *more* imbalanced.
    /// It's a required field.
    #[serde(rename = "whenUnsatisfiable")]
    pub when_unsatisfiable: String,
}

/// LabelSelector is used to find matching pods.
/// Pods that match this label selector are counted to determine the number of pods
/// in their corresponding topology domain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingTopologySpreadConstraintsLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SubscriptionDividingSchedulingDynamicDividingTopologySpreadConstraintsLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionDividingSchedulingDynamicDividingTopologySpreadConstraintsLabelSelectorMatchExpressions {
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

/// Dividing scheduling config params. Present only if SchedulingStrategy = Dividing.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionDividingSchedulingType {
    Static,
    Dynamic,
}

/// Feed defines the resource to be selected.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionFeeds {
    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Kind is a string value representing the REST resource this object represents.
    /// In CamelCase.
    pub kind: String,
    /// Name of the target resource.
    pub name: String,
    /// Namespace of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// SubscriptionSpec defines the desired state of Subscription
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionPreemptionPolicy {
    PreemptLowerPriority,
    PreemptNever,
}

/// SubscriptionSpec defines the desired state of Subscription
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SubscriptionSchedulingStrategy {
    Replication,
    Dividing,
}

/// Subscriber defines
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionSubscribers {
    /// ClusterAffinity is a label query over managed clusters by labels.
    #[serde(rename = "clusterAffinity")]
    pub cluster_affinity: SubscriptionSubscribersClusterAffinity,
    /// SubGroupStrategy defines the subgroup strategy for the clusters matched by this subscriber.
    /// During the scheduling, all the matching clusters will be treated as a subgroup instead of individual clusters.
    /// With subgroup, we can describe clusters with different regions, zones, etc.
    /// Present only when SchedulingBySubGroup is set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subGroupStrategy")]
    pub sub_group_strategy: Option<SubscriptionSubscribersSubGroupStrategy>,
    /// Static weight of subscriber when dividing replicas.
    /// Present only for static divided scheduling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

/// ClusterAffinity is a label query over managed clusters by labels.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionSubscribersClusterAffinity {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SubscriptionSubscribersClusterAffinityMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionSubscribersClusterAffinityMatchExpressions {
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

/// SubGroupStrategy defines the subgroup strategy for the clusters matched by this subscriber.
/// During the scheduling, all the matching clusters will be treated as a subgroup instead of individual clusters.
/// With subgroup, we can describe clusters with different regions, zones, etc.
/// Present only when SchedulingBySubGroup is set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionSubscribersSubGroupStrategy {
    /// MinClusters is the minimum number of clusters to be selected in this subgroup.
    /// If this value is more than the total number of clusters in this subgroup, then all clusters will be selected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minClusters")]
    pub min_clusters: Option<i32>,
}

/// SubscriptionStatus defines the observed state of Subscription
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionStatus {
    /// AggregatedStatuses shows the aggregated statuses of feeds that are running in each child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aggregatedStatuses")]
    pub aggregated_statuses: Option<Vec<SubscriptionStatusAggregatedStatuses>>,
    /// Namespaced names of targeted clusters that Subscription binds to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindingClusters")]
    pub binding_clusters: Option<Vec<String>>,
    /// Total number of completed releases targeted by this Subscription.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completedReleases")]
    pub completed_releases: Option<i64>,
    /// Total number of Helm releases desired by this Subscription.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredReleases")]
    pub desired_releases: Option<i64>,
    /// Desired replicas of targeted clusters for each feed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<BTreeMap<String, i64>>,
    /// SpecHash calculates the hash value of current SubscriptionSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specHash")]
    pub spec_hash: Option<i64>,
}

/// AggregatedStatus contains aggregated status of current feed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionStatusAggregatedStatuses {
    /// APIVersion defines the versioned schema of this representation of an object.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// FeedStatusDetails shows the feed statuses in each child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "feedStatusDetails")]
    pub feed_status_details: Option<Vec<SubscriptionStatusAggregatedStatusesFeedStatusDetails>>,
    /// FeedStatusSummary aggregates the feed statuses from each child cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "feedStatusSummary")]
    pub feed_status_summary: Option<SubscriptionStatusAggregatedStatusesFeedStatusSummary>,
    /// Kind is a string value representing the REST resource this object represents.
    /// In CamelCase.
    pub kind: String,
    /// Name of the target resource.
    pub name: String,
    /// Namespace of the target resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// FeedStatusPerCluster shows the feed status running in current cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionStatusAggregatedStatusesFeedStatusDetails {
    /// Available indicates whether the feed status is synced successfully to corresponding Description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// ClusterID indicates the id of current cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterId")]
    pub cluster_id: Option<String>,
    /// ClusterName is the cluster name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// ReplicaStatus indicates the replica status of workload-type feed, such as Deployment/StatefulSet/Job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaStatus")]
    pub replica_status: Option<SubscriptionStatusAggregatedStatusesFeedStatusDetailsReplicaStatus>,
}

/// ReplicaStatus indicates the replica status of workload-type feed, such as Deployment/StatefulSet/Job.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionStatusAggregatedStatusesFeedStatusDetailsReplicaStatus {
    /// The number of pending and running pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// currentReplicas is the number of Pods created by the workload controller from the StatefulSet version
    /// indicated by currentRevision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentReplicas")]
    pub current_replicas: Option<i32>,
    /// The number of pods which reached phase Failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// The generation observed by the workload controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyReplicas is the number of pods targeted by this workload with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this workload (their labels match the selector).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The number of pods which reached phase Succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
    /// Total number of unavailable pods targeted by this workload. This is the total number of
    /// pods that are still required for the workload to have 100% available capacity. They may
    /// either be pods that are running but not yet available or pods that still have not been created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableReplicas")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this workload that have the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReplicas")]
    pub updated_replicas: Option<i32>,
}

/// FeedStatusSummary aggregates the feed statuses from each child cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionStatusAggregatedStatusesFeedStatusSummary {
    /// Available indicates whether the feed status is synced successfully to corresponding Description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// ReplicaStatus indicates the replica status of workload-type feed, such as Deployment/StatefulSet/Job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaStatus")]
    pub replica_status: Option<SubscriptionStatusAggregatedStatusesFeedStatusSummaryReplicaStatus>,
}

/// ReplicaStatus indicates the replica status of workload-type feed, such as Deployment/StatefulSet/Job.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionStatusAggregatedStatusesFeedStatusSummaryReplicaStatus {
    /// The number of pending and running pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this workload.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availableReplicas")]
    pub available_replicas: Option<i32>,
    /// currentReplicas is the number of Pods created by the workload controller from the StatefulSet version
    /// indicated by currentRevision.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentReplicas")]
    pub current_replicas: Option<i32>,
    /// The number of pods which reached phase Failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// The generation observed by the workload controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// readyReplicas is the number of pods targeted by this workload with a Ready Condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this workload (their labels match the selector).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The number of pods which reached phase Succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
    /// Total number of unavailable pods targeted by this workload. This is the total number of
    /// pods that are still required for the workload to have 100% available capacity. They may
    /// either be pods that are running but not yet available or pods that still have not been created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unavailableReplicas")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this workload that have the desired template spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReplicas")]
    pub updated_replicas: Option<i32>,
}

