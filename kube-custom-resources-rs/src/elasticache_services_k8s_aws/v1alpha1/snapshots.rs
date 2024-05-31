// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/elasticache-controller/elasticache.services.k8s.aws/v1alpha1/snapshots.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// SnapshotSpec defines the desired state of Snapshot.
/// 
/// 
/// Represents a copy of an entire Redis cluster as of the time when the snapshot
/// was taken.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "elasticache.services.k8s.aws", version = "v1alpha1", kind = "Snapshot", plural = "snapshots")]
#[kube(namespaced)]
#[kube(status = "SnapshotStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SnapshotSpec {
    /// The identifier of an existing cluster. The snapshot is created from this
    /// cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheClusterID")]
    pub cache_cluster_id: Option<String>,
    /// The ID of the KMS key used to encrypt the snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// The identifier of an existing replication group. The snapshot is created
    /// from this replication group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationGroupID")]
    pub replication_group_id: Option<String>,
    /// A name for the snapshot being created.
    #[serde(rename = "snapshotName")]
    pub snapshot_name: String,
    /// The name of an existing snapshot from which to make a copy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceSnapshotName")]
    pub source_snapshot_name: Option<String>,
    /// A list of tags to be added to this resource. A tag is a key-value pair. A
    /// tag key must be accompanied by a tag value, although null is accepted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<SnapshotTags>>,
}

/// A tag that can be added to an ElastiCache cluster or replication group. Tags
/// are composed of a Key/Value pair. You can use tags to categorize and track
/// all your ElastiCache resources, with the exception of global replication
/// group. When you add or remove tags on replication groups, those actions will
/// be replicated to all nodes in the replication group. A tag with a null Value
/// is permitted.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// SnapshotStatus defines the observed state of Snapshot
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<SnapshotStatusAckResourceMetadata>,
    /// If you are running Redis engine version 6.0 or later, set this parameter
    /// to yes if you want to opt-in to the next auto minor version upgrade campaign.
    /// This parameter is disabled for previous versions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// Indicates the status of automatic failover for the source Redis replication
    /// group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "automaticFailover")]
    pub automatic_failover: Option<String>,
    /// The date and time when the source cluster was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheClusterCreateTime")]
    pub cache_cluster_create_time: Option<String>,
    /// The name of the compute and memory capacity node type for the source cluster.
    /// 
    /// 
    /// The following node types are supported by ElastiCache. Generally speaking,
    /// the current generation types provide more memory and computational power
    /// at lower cost when compared to their equivalent previous generation counterparts.
    /// 
    /// 
    ///    * General purpose: Current generation: M6g node types (available only
    ///    for Redis engine version 5.0.6 onward and for Memcached engine version
    ///    1.5.16 onward). cache.m6g.large, cache.m6g.xlarge, cache.m6g.2xlarge,
    ///    cache.m6g.4xlarge, cache.m6g.8xlarge, cache.m6g.12xlarge, cache.m6g.16xlarge
    ///    For region availability, see Supported Node Types (https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion)
    ///    M5 node types: cache.m5.large, cache.m5.xlarge, cache.m5.2xlarge, cache.m5.4xlarge,
    ///    cache.m5.12xlarge, cache.m5.24xlarge M4 node types: cache.m4.large, cache.m4.xlarge,
    ///    cache.m4.2xlarge, cache.m4.4xlarge, cache.m4.10xlarge T4g node types (available
    ///    only for Redis engine version 5.0.6 onward and Memcached engine version
    ///    1.5.16 onward): cache.t4g.micro, cache.t4g.small, cache.t4g.medium T3
    ///    node types: cache.t3.micro, cache.t3.small, cache.t3.medium T2 node types:
    ///    cache.t2.micro, cache.t2.small, cache.t2.medium Previous generation: (not
    ///    recommended. Existing clusters are still supported but creation of new
    ///    clusters is not supported for these types.) T1 node types: cache.t1.micro
    ///    M1 node types: cache.m1.small, cache.m1.medium, cache.m1.large, cache.m1.xlarge
    ///    M3 node types: cache.m3.medium, cache.m3.large, cache.m3.xlarge, cache.m3.2xlarge
    /// 
    /// 
    ///    * Compute optimized: Previous generation: (not recommended. Existing clusters
    ///    are still supported but creation of new clusters is not supported for
    ///    these types.) C1 node types: cache.c1.xlarge
    /// 
    /// 
    ///    * Memory optimized with data tiering: Current generation: R6gd node types
    ///    (available only for Redis engine version 6.2 onward). cache.r6gd.xlarge,
    ///    cache.r6gd.2xlarge, cache.r6gd.4xlarge, cache.r6gd.8xlarge, cache.r6gd.12xlarge,
    ///    cache.r6gd.16xlarge
    /// 
    /// 
    ///    * Memory optimized: Current generation: R6g node types (available only
    ///    for Redis engine version 5.0.6 onward and for Memcached engine version
    ///    1.5.16 onward). cache.r6g.large, cache.r6g.xlarge, cache.r6g.2xlarge,
    ///    cache.r6g.4xlarge, cache.r6g.8xlarge, cache.r6g.12xlarge, cache.r6g.16xlarge
    ///    For region availability, see Supported Node Types (https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion)
    ///    For region availability, see Supported Node Types (https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/CacheNodes.SupportedTypes.html#CacheNodes.SupportedTypesByRegion)
    ///    R5 node types: cache.r5.large, cache.r5.xlarge, cache.r5.2xlarge, cache.r5.4xlarge,
    ///    cache.r5.12xlarge, cache.r5.24xlarge R4 node types: cache.r4.large, cache.r4.xlarge,
    ///    cache.r4.2xlarge, cache.r4.4xlarge, cache.r4.8xlarge, cache.r4.16xlarge
    ///    Previous generation: (not recommended. Existing clusters are still supported
    ///    but creation of new clusters is not supported for these types.) M2 node
    ///    types: cache.m2.xlarge, cache.m2.2xlarge, cache.m2.4xlarge R3 node types:
    ///    cache.r3.large, cache.r3.xlarge, cache.r3.2xlarge, cache.r3.4xlarge, cache.r3.8xlarge
    /// 
    /// 
    /// Additional node type info
    /// 
    /// 
    ///    * All current generation instance types are created in Amazon VPC by default.
    /// 
    /// 
    ///    * Redis append-only files (AOF) are not supported for T1 or T2 instances.
    /// 
    /// 
    ///    * Redis Multi-AZ with automatic failover is not supported on T1 instances.
    /// 
    /// 
    ///    * Redis configuration variables appendonly and appendfsync are not supported
    ///    on Redis version 2.8.22 and later.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheNodeType")]
    pub cache_node_type: Option<String>,
    /// The cache parameter group that is associated with the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheParameterGroupName")]
    pub cache_parameter_group_name: Option<String>,
    /// The name of the cache subnet group associated with the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheSubnetGroupName")]
    pub cache_subnet_group_name: Option<String>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Enables data tiering. Data tiering is only supported for replication groups
    /// using the r6gd node type. This parameter must be set to true when using r6gd
    /// nodes. For more information, see Data tiering (https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/data-tiering.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataTiering")]
    pub data_tiering: Option<String>,
    /// The name of the cache engine (memcached or redis) used by the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// The version of the cache engine version that is used by the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<String>,
    /// A list of the cache nodes in the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<SnapshotStatusNodeSnapshots>>,
    /// The number of cache nodes in the source cluster.
    /// 
    /// 
    /// For clusters running Redis, this value must be 1. For clusters running Memcached,
    /// this value must be between 1 and 40.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numCacheNodes")]
    pub num_cache_nodes: Option<i64>,
    /// The number of node groups (shards) in this snapshot. When restoring from
    /// a snapshot, the number of node groups (shards) in the snapshot and in the
    /// restored replication group must be the same.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numNodeGroups")]
    pub num_node_groups: Option<i64>,
    /// The port number used by each cache nodes in the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// The name of the Availability Zone in which the source cluster is located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredAvailabilityZone")]
    pub preferred_availability_zone: Option<String>,
    /// Specifies the weekly time range during which maintenance on the cluster is
    /// performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi
    /// (24H Clock UTC). The minimum maintenance window is a 60 minute period.
    /// 
    /// 
    /// Valid values for ddd are:
    /// 
    /// 
    ///    * sun
    /// 
    /// 
    ///    * mon
    /// 
    /// 
    ///    * tue
    /// 
    /// 
    ///    * wed
    /// 
    /// 
    ///    * thu
    /// 
    /// 
    ///    * fri
    /// 
    /// 
    ///    * sat
    /// 
    /// 
    /// Example: sun:23:00-mon:01:30
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredMaintenanceWindow")]
    pub preferred_maintenance_window: Option<String>,
    /// The ARN (Amazon Resource Name) of the preferred outpost.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredOutpostARN")]
    pub preferred_outpost_arn: Option<String>,
    /// A description of the source replication group.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicationGroupDescription")]
    pub replication_group_description: Option<String>,
    /// For an automatic snapshot, the number of days for which ElastiCache retains
    /// the snapshot before deleting it.
    /// 
    /// 
    /// For manual snapshots, this field reflects the SnapshotRetentionLimit for
    /// the source cluster when the snapshot was created. This field is otherwise
    /// ignored: Manual snapshots do not expire, and can only be deleted using the
    /// DeleteSnapshot operation.
    /// 
    /// 
    /// Important If the value of SnapshotRetentionLimit is set to zero (0), backups
    /// are turned off.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotRetentionLimit")]
    pub snapshot_retention_limit: Option<i64>,
    /// Indicates whether the snapshot is from an automatic backup (automated) or
    /// was created manually (manual).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotSource")]
    pub snapshot_source: Option<String>,
    /// The status of the snapshot. Valid values: creating | available | restoring
    /// | copying | deleting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotStatus")]
    pub snapshot_status: Option<String>,
    /// The daily time range during which ElastiCache takes daily snapshots of the
    /// source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotWindow")]
    pub snapshot_window: Option<String>,
    /// The Amazon Resource Name (ARN) for the topic used by the source cluster for
    /// publishing notifications.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicARN")]
    pub topic_arn: Option<String>,
    /// The Amazon Virtual Private Cloud identifier (VPC ID) of the cache subnet
    /// group for the source cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Represents an individual cache node in a snapshot of a cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotStatusNodeSnapshots {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheClusterID")]
    pub cache_cluster_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheNodeCreateTime")]
    pub cache_node_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheNodeID")]
    pub cache_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheSize")]
    pub cache_size: Option<String>,
    /// Node group (shard) configuration options. Each node group (shard) configuration
    /// has the following: Slots, PrimaryAvailabilityZone, ReplicaAvailabilityZones,
    /// ReplicaCount.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeGroupConfiguration")]
    pub node_group_configuration: Option<SnapshotStatusNodeSnapshotsNodeGroupConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeGroupID")]
    pub node_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotCreateTime")]
    pub snapshot_create_time: Option<String>,
}

/// Node group (shard) configuration options. Each node group (shard) configuration
/// has the following: Slots, PrimaryAvailabilityZone, ReplicaAvailabilityZones,
/// ReplicaCount.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnapshotStatusNodeSnapshotsNodeGroupConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeGroupID")]
    pub node_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryAvailabilityZone")]
    pub primary_availability_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryOutpostARN")]
    pub primary_outpost_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaAvailabilityZones")]
    pub replica_availability_zones: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaCount")]
    pub replica_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaOutpostARNs")]
    pub replica_outpost_ar_ns: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
}

