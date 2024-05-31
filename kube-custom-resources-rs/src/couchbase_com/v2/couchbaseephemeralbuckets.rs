// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/couchbase-partners/helm-charts/couchbase.com/v2/couchbaseephemeralbuckets.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// CouchbaseEphemeralBucketSpec is the specification for an ephemeral Couchbase bucket resource, and allows the bucket to be customized.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "couchbase.com", version = "v2", kind = "CouchbaseEphemeralBucket", plural = "couchbaseephemeralbuckets")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CouchbaseEphemeralBucketSpec {
    /// CompressionMode defines how Couchbase server handles document compression.  When off, documents are stored in memory, and transferred to the client uncompressed. When passive, documents are stored compressed in memory, and transferred to the client compressed when requested.  When active, documents are stored compresses in memory and when transferred to the client.  This field must be "off", "passive" or "active", defaulting to "passive".  Be aware "off" in YAML 1.2 is a boolean, so must be quoted as a string in configuration files.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionMode")]
    pub compression_mode: Option<CouchbaseEphemeralBucketCompressionMode>,
    /// ConflictResolution defines how XDCR handles concurrent write conflicts.  Sequence number based resolution selects the document with the highest sequence number as the most recent. Timestamp based resolution selects the document that was written to most recently as the most recent.  This field must be "seqno" (sequence based), or "lww" (timestamp based), defaulting to "seqno".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conflictResolution")]
    pub conflict_resolution: Option<CouchbaseEphemeralBucketConflictResolution>,
    /// EnableFlush defines whether a client can delete all documents in a bucket. This field defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableFlush")]
    pub enable_flush: Option<bool>,
    /// EvictionPolicy controls how Couchbase handles memory exhaustion.  No eviction means that Couchbase server will make this bucket read-only when memory is exhausted in order to avoid data loss.  NRU eviction will delete documents that haven't been used recently in order to free up memory. This field must be "noEviction" or "nruEviction", defaulting to "noEviction".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionPolicy")]
    pub eviction_policy: Option<CouchbaseEphemeralBucketEvictionPolicy>,
    /// IOPriority controls how many threads a bucket has, per pod, to process reads and writes. This field must be "low" or "high", defaulting to "low".  Modification of this field will cause a temporary service disruption as threads are restarted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ioPriority")]
    pub io_priority: Option<CouchbaseEphemeralBucketIoPriority>,
    /// MaxTTL defines how long a document is permitted to exist for, without modification, until it is automatically deleted.  This is a default and maximum time-to-live and may be set to a lower value by the client.  If the client specifies a higher value, then it is truncated to the maximum durability.  Documents are removed by Couchbase, after they have expired, when either accessed, the expiry pager is run, or the bucket is compacted.  When set to 0, then documents are not expired by default.  This field must be a duration in the range 0-2147483648s, defaulting to 0.  More info: https://golang.org/pkg/time/#ParseDuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTTL")]
    pub max_ttl: Option<String>,
    /// MemoryQuota is a memory limit to the size of a bucket.  When this limit is exceeded, documents will be evicted from memory defined by the eviction policy.  The memory quota is defined per Couchbase pod running the data service.  This field defaults to, and must be greater than or equal to 100Mi.  More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#resource-units-in-kubernetes
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryQuota")]
    pub memory_quota: Option<String>,
    /// MiniumumDurability defines how durable a document write is by default, and can be made more durable by the client.  This feature enables ACID transactions. When none, Couchbase server will respond when the document is in memory, it will become eventually consistent across the cluster.  When majority, Couchbase server will respond when the document is replicated to at least half of the pods running the data service in the cluster.  This field must be either "none" or "majority", defaulting to "none".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minimumDurability")]
    pub minimum_durability: Option<CouchbaseEphemeralBucketMinimumDurability>,
    /// Name is the name of the bucket within Couchbase server.  By default the Operator will use the `metadata.name` field to define the bucket name.  The `metadata.name` field only supports a subset of the supported character set.  When specified, this field overrides `metadata.name`.  Legal bucket names have a maximum length of 100 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "-_%\.".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replicas defines how many copies of documents Couchbase server maintains.  This directly affects how fault tolerant a Couchbase cluster is.  With a single replica, the cluster can tolerate one data pod going down and still service requests without data loss.  The number of replicas also affect memory use.  With a single replica, the effective memory quota for documents is halved, with two replicas it is one third.  The number of replicas must be between 0 and 3, defaulting to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// Scopes defines whether the Operator manages scopes for the bucket or not, and the set of scopes defined for the bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<CouchbaseEphemeralBucketScopes>,
}

/// CouchbaseEphemeralBucketSpec is the specification for an ephemeral Couchbase bucket resource, and allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseEphemeralBucketCompressionMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "passive")]
    Passive,
    #[serde(rename = "active")]
    Active,
}

/// CouchbaseEphemeralBucketSpec is the specification for an ephemeral Couchbase bucket resource, and allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseEphemeralBucketConflictResolution {
    #[serde(rename = "seqno")]
    Seqno,
    #[serde(rename = "lww")]
    Lww,
}

/// CouchbaseEphemeralBucketSpec is the specification for an ephemeral Couchbase bucket resource, and allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseEphemeralBucketEvictionPolicy {
    #[serde(rename = "noEviction")]
    NoEviction,
    #[serde(rename = "nruEviction")]
    NruEviction,
}

/// CouchbaseEphemeralBucketSpec is the specification for an ephemeral Couchbase bucket resource, and allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseEphemeralBucketIoPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}

/// CouchbaseEphemeralBucketSpec is the specification for an ephemeral Couchbase bucket resource, and allows the bucket to be customized.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseEphemeralBucketMinimumDurability {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "majority")]
    Majority,
}

/// Scopes defines whether the Operator manages scopes for the bucket or not, and the set of scopes defined for the bucket.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseEphemeralBucketScopes {
    /// Managed defines whether scopes are managed for this bucket. This field is `false` by default, and the Operator will take no actions that will affect scopes and collections in this bucket.  The default scope and collection will be present.  When set to `true`, the Operator will manage user defined scopes, and optionally, their collections as defined by the `CouchbaseScope`, `CouchbaseScopeGroup`, `CouchbaseCollection` and `CouchbaseCollectionGroup` resource documentation.  If this field is set to `false` while the  already managed, then the Operator will leave whatever configuration is already present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// Resources is an explicit list of named resources that will be considered for inclusion in this bucket.  If a resource reference doesn't match a resource, then no error conditions are raised due to undefined resource creation ordering and eventual consistency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CouchbaseEphemeralBucketScopesResources>>,
    /// Selector allows resources to be implicitly considered for inclusion in this bucket.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<CouchbaseEphemeralBucketScopesSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseEphemeralBucketScopesResources {
    /// Kind indicates the kind of resource that is being referenced.  A scope can only reference `CouchbaseScope` and `CouchbaseScopeGroup` resource kinds.  This field defaults to `CouchbaseScope` if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<CouchbaseEphemeralBucketScopesResourcesKind>,
    /// Name is the name of the Kubernetes resource name that is being referenced. Legal scope names have a maximum length of 251 characters and may be composed of any character from "a-z", "A-Z", "0-9" and "_-%".
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CouchbaseEphemeralBucketScopesResourcesKind {
    CouchbaseScope,
    CouchbaseScopeGroup,
}

/// Selector allows resources to be implicitly considered for inclusion in this bucket.  More info: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.21/#labelselector-v1-meta
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseEphemeralBucketScopesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CouchbaseEphemeralBucketScopesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CouchbaseEphemeralBucketScopesSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

