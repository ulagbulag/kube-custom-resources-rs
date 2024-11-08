// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/hive/hive.openshift.io/v1/machinepools.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// MachinePoolSpec defines the desired state of MachinePool
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "MachinePool", plural = "machinepools")]
#[kube(namespaced)]
#[kube(status = "MachinePoolStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MachinePoolSpec {
    /// Autoscaling is the details for auto-scaling the machine pool.
    /// Replicas and autoscaling cannot be used together.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoscaling: Option<MachinePoolAutoscaling>,
    /// ClusterDeploymentRef references the cluster deployment to which this
    /// machine pool belongs.
    #[serde(rename = "clusterDeploymentRef")]
    pub cluster_deployment_ref: MachinePoolClusterDeploymentRef,
    /// Map of label string keys and values that will be applied to the created MachineSet's
    /// MachineSpec. This affects the labels that will end up on the *Nodes* (in contrast with
    /// the MachineLabels field). This list will overwrite any modifications made to Node labels
    /// on an ongoing basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Map of label string keys and values that will be applied to the created MachineSet's
    /// MachineTemplateSpec. This affects the labels that will end up on the *Machines* (in
    /// contrast with the Labels field). This list will overwrite any modifications made to
    /// Machine labels on an ongoing basis. Note: We ignore entries that conflict with
    /// generated labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineLabels")]
    pub machine_labels: Option<BTreeMap<String, String>>,
    /// Name is the name of the machine pool.
    pub name: String,
    /// Platform is configuration for machine pool specific to the platform.
    /// When using a MachinePool to control the default worker machines
    /// created by installer, these must match the values provided in the
    /// install-config.
    pub platform: MachinePoolPlatform,
    /// Replicas is the count of machines for this machine pool.
    /// Replicas and autoscaling cannot be used together.
    /// Default is 1, if autoscaling is not used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    /// List of taints that will be applied to the created MachineSet's MachineSpec.
    /// This list will overwrite any modifications made to Node taints on an ongoing basis.
    /// In case of duplicate entries, first encountered taint Value will be preserved,
    /// and the rest collapsed on the corresponding MachineSets.
    /// Note that taints are uniquely identified based on key+effect, not just key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<MachinePoolTaints>>,
}

/// Autoscaling is the details for auto-scaling the machine pool.
/// Replicas and autoscaling cannot be used together.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolAutoscaling {
    /// MaxReplicas is the maximum number of replicas for the machine pool.
    #[serde(rename = "maxReplicas")]
    pub max_replicas: i32,
    /// MinReplicas is the minimum number of replicas for the machine pool.
    #[serde(rename = "minReplicas")]
    pub min_replicas: i32,
}

/// ClusterDeploymentRef references the cluster deployment to which this
/// machine pool belongs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolClusterDeploymentRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Platform is configuration for machine pool specific to the platform.
/// When using a MachinePool to control the default worker machines
/// created by installer, these must match the values provided in the
/// install-config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatform {
    /// AWS is the configuration used when installing on AWS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws: Option<MachinePoolPlatformAws>,
    /// Azure is the configuration used when installing on Azure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<MachinePoolPlatformAzure>,
    /// GCP is the configuration used when installing on GCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcp: Option<MachinePoolPlatformGcp>,
    /// IBMCloud is the configuration used when installing on IBM Cloud.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibmcloud: Option<MachinePoolPlatformIbmcloud>,
    /// OpenStack is the configuration used when installing on OpenStack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openstack: Option<MachinePoolPlatformOpenstack>,
    /// Ovirt is the configuration used when installing on oVirt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ovirt: Option<MachinePoolPlatformOvirt>,
    /// VSphere is the configuration used when installing on vSphere
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vsphere: Option<MachinePoolPlatformVsphere>,
}

/// AWS is the configuration used when installing on AWS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAws {
    /// AdditionalSecurityGroupIDs contains IDs of additional security groups for machines, where each ID
    /// is presented in the format sg-xxxx.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalSecurityGroupIDs")]
    pub additional_security_group_i_ds: Option<Vec<String>>,
    /// EC2MetadataOptions defines metadata service interaction options for EC2 instances in the machine pool.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataService")]
    pub metadata_service: Option<MachinePoolPlatformAwsMetadataService>,
    /// EC2RootVolume defines the storage for ec2 instance.
    #[serde(rename = "rootVolume")]
    pub root_volume: MachinePoolPlatformAwsRootVolume,
    /// SpotMarketOptions allows users to configure instances to be run using AWS Spot instances.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spotMarketOptions")]
    pub spot_market_options: Option<MachinePoolPlatformAwsSpotMarketOptions>,
    /// Subnets is the list of IDs of subnets to which to attach the machines.
    /// There must be exactly one subnet for each availability zone used.
    /// These subnets may be public or private.
    /// As a special case, for consistency with install-config, you may specify exactly one
    /// private and one public subnet for each availability zone. In this case, the public
    /// subnets will be filtered out and only the private subnets will be used.
    /// If empty/omitted, we will look for subnets in each availability zone tagged with
    /// Name=<clusterID>-private-<az> (legacy terraform) or <clusterID>-subnet-private-<az>
    /// (CAPA).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// InstanceType defines the ec2 instance type.
    /// eg. m4-large
    #[serde(rename = "type")]
    pub r#type: String,
    /// UserTags contains the user defined tags to be supplied for the ec2 instance.
    /// Note that these will be merged with ClusterDeployment.Spec.Platform.AWS.UserTags, with
    /// this field taking precedence when keys collide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userTags")]
    pub user_tags: Option<BTreeMap<String, String>>,
    /// Zones is list of availability zones that can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

/// EC2MetadataOptions defines metadata service interaction options for EC2 instances in the machine pool.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAwsMetadataService {
    /// Authentication determines whether or not the host requires the use of authentication when interacting with the metadata service.
    /// When using authentication, this enforces v2 interaction method (IMDSv2) with the metadata service.
    /// When omitted, this means the user has no opinion and the value is left to the platform to choose a good
    /// default, which is subject to change over time. The current default is optional.
    /// At this point this field represents `HttpTokens` parameter from `InstanceMetadataOptionsRequest` structure in AWS EC2 API
    /// https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_InstanceMetadataOptionsRequest.html
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}

/// EC2RootVolume defines the storage for ec2 instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAwsRootVolume {
    /// IOPS defines the iops for the storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// The KMS key that will be used to encrypt the EBS volume.
    /// If no key is provided the default KMS key for the account will be used.
    /// https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetEbsDefaultKmsKeyId.html
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyARN")]
    pub kms_key_arn: Option<String>,
    /// Size defines the size of the storage.
    pub size: i64,
    /// Type defines the type of the storage.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// SpotMarketOptions allows users to configure instances to be run using AWS Spot instances.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAwsSpotMarketOptions {
    /// The maximum price the user is willing to pay for their instances
    /// Default: On-Demand price
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPrice")]
    pub max_price: Option<String>,
}

/// Azure is the configuration used when installing on Azure.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAzure {
    /// ComputeSubnet specifies an existing subnet for use by compute nodes.
    /// If omitted, the default (${infraID}-worker-subnet) will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "computeSubnet")]
    pub compute_subnet: Option<String>,
    /// NetworkResourceGroupName specifies the network resource group that contains an existing VNet.
    /// Ignored unless VirtualNetwork is also specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkResourceGroupName")]
    pub network_resource_group_name: Option<String>,
    /// OSDisk defines the storage for instance.
    #[serde(rename = "osDisk")]
    pub os_disk: MachinePoolPlatformAzureOsDisk,
    /// OSImage defines the image to use for the OS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "osImage")]
    pub os_image: Option<MachinePoolPlatformAzureOsImage>,
    /// InstanceType defines the azure instance type.
    /// eg. Standard_DS_V2
    #[serde(rename = "type")]
    pub r#type: String,
    /// VirtualNetwork specifies the name of an existing VNet for the Machines to use
    /// If omitted, the default (${infraID}-vnet) will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualNetwork")]
    pub virtual_network: Option<String>,
    /// VMNetworkingType specifies whether to enable accelerated networking.
    /// Accelerated networking enables single root I/O virtualization (SR-IOV) to a VM, greatly improving its
    /// networking performance.
    /// eg. values: "Accelerated", "Basic"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vmNetworkingType")]
    pub vm_networking_type: Option<MachinePoolPlatformAzureVmNetworkingType>,
    /// Zones is list of availability zones that can be used.
    /// eg. ["1", "2", "3"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

/// OSDisk defines the storage for instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAzureOsDisk {
    /// DiskEncryptionSet defines a disk encryption set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskEncryptionSet")]
    pub disk_encryption_set: Option<MachinePoolPlatformAzureOsDiskDiskEncryptionSet>,
    /// DiskSizeGB defines the size of disk in GB.
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i32,
    /// DiskType defines the type of disk.
    /// For control plane nodes, the valid values are Premium_LRS and StandardSSD_LRS.
    /// Default is Premium_LRS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskType")]
    pub disk_type: Option<MachinePoolPlatformAzureOsDiskDiskType>,
}

/// DiskEncryptionSet defines a disk encryption set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAzureOsDiskDiskEncryptionSet {
    /// Name is the name of the disk encryption set.
    pub name: String,
    /// ResourceGroup defines the Azure resource group used by the disk
    /// encryption set.
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    /// SubscriptionID defines the Azure subscription the disk encryption
    /// set is in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subscriptionId")]
    pub subscription_id: Option<String>,
}

/// OSDisk defines the storage for instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachinePoolPlatformAzureOsDiskDiskType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "StandardSSD_LRS")]
    StandardSsdLrs,
}

/// OSImage defines the image to use for the OS.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformAzureOsImage {
    /// Offer is the offer of the image.
    pub offer: String,
    /// Publisher is the publisher of the image.
    pub publisher: String,
    /// SKU is the SKU of the image.
    pub sku: String,
    /// Version is the version of the image.
    pub version: String,
}

/// Azure is the configuration used when installing on Azure.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachinePoolPlatformAzureVmNetworkingType {
    Accelerated,
    Basic,
}

/// GCP is the configuration used when installing on GCP.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformGcp {
    /// NetworkProjectID specifies which project the network and subnets exist in when
    /// they are not in the main ProjectID.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkProjectID")]
    pub network_project_id: Option<String>,
    /// OnHostMaintenance determines the behavior when a maintenance event occurs that might cause the instance to reboot.
    /// This is required to be set to "Terminate" if you want to provision machine with attached GPUs.
    /// Otherwise, allowed values are "Migrate" and "Terminate".
    /// If omitted, the platform chooses a default, which is subject to change over time, currently that default is "Migrate".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onHostMaintenance")]
    pub on_host_maintenance: Option<MachinePoolPlatformGcpOnHostMaintenance>,
    /// OSDisk defines the storage for instances.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "osDisk")]
    pub os_disk: Option<MachinePoolPlatformGcpOsDisk>,
    /// SecureBoot Defines whether the instance should have secure boot enabled.
    /// Verifies the digital signature of all boot components, and halts the boot process if signature verification fails.
    /// If omitted, the platform chooses a default, which is subject to change over time. Currently that default is "Disabled".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secureBoot")]
    pub secure_boot: Option<MachinePoolPlatformGcpSecureBoot>,
    /// ServiceAccount is the email of a gcp service account to be attached to worker nodes
    /// in order to provide the permissions required by the cloud provider. For the default
    /// worker MachinePool, it is the user's responsibility to match this to the value
    /// provided in the install-config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// InstanceType defines the GCP instance type.
    /// eg. n1-standard-4
    #[serde(rename = "type")]
    pub r#type: String,
    /// userTags has additional keys and values that we will add as tags to the providerSpec of
    /// MachineSets that we creates on GCP. Tag key and tag value should be the shortnames of the
    /// tag key and tag value resource. Consumer is responsible for using this only for spokes
    /// where custom tags are supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userTags")]
    pub user_tags: Option<Vec<MachinePoolPlatformGcpUserTags>>,
    /// Zones is list of availability zones that can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

/// GCP is the configuration used when installing on GCP.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachinePoolPlatformGcpOnHostMaintenance {
    Migrate,
    Terminate,
}

/// OSDisk defines the storage for instances.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformGcpOsDisk {
    /// DiskSizeGB defines the size of disk in GB.
    /// Defaulted internally to 128.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskSizeGB")]
    pub disk_size_gb: Option<i64>,
    /// DiskType defines the type of disk.
    /// The valid values are pd-standard and pd-ssd.
    /// Defaulted internally to pd-ssd.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskType")]
    pub disk_type: Option<MachinePoolPlatformGcpOsDiskDiskType>,
    /// EncryptionKey defines the KMS key to be used to encrypt the disk.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionKey")]
    pub encryption_key: Option<MachinePoolPlatformGcpOsDiskEncryptionKey>,
}

/// OSDisk defines the storage for instances.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachinePoolPlatformGcpOsDiskDiskType {
    #[serde(rename = "pd-ssd")]
    PdSsd,
    #[serde(rename = "pd-standard")]
    PdStandard,
}

/// EncryptionKey defines the KMS key to be used to encrypt the disk.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformGcpOsDiskEncryptionKey {
    /// KMSKey is a reference to a KMS Key to use for the encryption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKey")]
    pub kms_key: Option<MachinePoolPlatformGcpOsDiskEncryptionKeyKmsKey>,
    /// KMSKeyServiceAccount is the service account being used for the
    /// encryption request for the given KMS key. If absent, the Compute
    /// Engine default service account is used.
    /// See https://cloud.google.com/compute/docs/access/service-accounts#compute_engine_service_account
    /// for details on the default service account.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyServiceAccount")]
    pub kms_key_service_account: Option<String>,
}

/// KMSKey is a reference to a KMS Key to use for the encryption.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformGcpOsDiskEncryptionKeyKmsKey {
    /// KeyRing is the name of the KMS Key Ring which the KMS Key belongs to.
    #[serde(rename = "keyRing")]
    pub key_ring: String,
    /// Location is the GCP location in which the Key Ring exists.
    pub location: String,
    /// Name is the name of the customer managed encryption key to be used for the disk encryption.
    pub name: String,
    /// ProjectID is the ID of the Project in which the KMS Key Ring exists.
    /// Defaults to the VM ProjectID if not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectID")]
    pub project_id: Option<String>,
}

/// GCP is the configuration used when installing on GCP.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachinePoolPlatformGcpSecureBoot {
    Enabled,
    Disabled,
}

/// UserTag is a tag to apply to GCP resources created for the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformGcpUserTags {
    /// key is the key part of the tag. A tag key can have a maximum of 63 characters and
    /// cannot be empty. Tag key must begin and end with an alphanumeric character, and
    /// must contain only uppercase, lowercase alphanumeric characters, and the following
    /// special characters `._-`.
    pub key: String,
    /// parentID is the ID of the hierarchical resource where the tags are defined,
    /// e.g. at the Organization or the Project level. To find the Organization ID or Project ID refer to the following pages:
    /// https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id,
    /// https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects.
    /// An OrganizationID must consist of decimal numbers, and cannot have leading zeroes.
    /// A ProjectID must be 6 to 30 characters in length, can only contain lowercase letters,
    /// numbers, and hyphens, and must start with a letter, and cannot end with a hyphen.
    #[serde(rename = "parentID")]
    pub parent_id: String,
    /// value is the value part of the tag. A tag value can have a maximum of 63 characters
    /// and cannot be empty. Tag value must begin and end with an alphanumeric character, and
    /// must contain only uppercase, lowercase alphanumeric characters, and the following
    /// special characters `_-.@%=+:,*#&(){}[]` and spaces.
    pub value: String,
}

/// IBMCloud is the configuration used when installing on IBM Cloud.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformIbmcloud {
    /// BootVolume is the configuration for the machine's boot volume.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootVolume")]
    pub boot_volume: Option<MachinePoolPlatformIbmcloudBootVolume>,
    /// DedicatedHosts is the configuration for the machine's dedicated host and profile.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedHosts")]
    pub dedicated_hosts: Option<Vec<MachinePoolPlatformIbmcloudDedicatedHosts>>,
    /// InstanceType is the VSI machine profile.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// Zones is the list of availability zones used for machines in the pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

/// BootVolume is the configuration for the machine's boot volume.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformIbmcloudBootVolume {
    /// EncryptionKey is the CRN referencing a Key Protect or Hyper Protect
    /// Crypto Services key to use for volume encryption. If not specified, a
    /// provider managed encryption key will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionKey")]
    pub encryption_key: Option<String>,
}

/// DedicatedHost stores the configuration for the machine's dedicated host platform.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformIbmcloudDedicatedHosts {
    /// Name is the name of the dedicated host to provision the machine on. If
    /// specified, machines will be created on pre-existing dedicated host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Profile is the profile ID for the dedicated host. If specified, new
    /// dedicated host will be created for machines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

/// OpenStack is the configuration used when installing on OpenStack.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformOpenstack {
    /// AdditionalSecurityGroupIDs contains IDs of additional security groups for machines, where each ID
    /// is presented in the format sg-xxxx.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalSecurityGroupIDs")]
    pub additional_security_group_i_ds: Option<Vec<String>>,
    /// Flavor defines the OpenStack Nova flavor.
    /// eg. m1.large
    /// The json key here differs from the installer which uses both "computeFlavor" and type "type" depending on which
    /// type you're looking at, and the resulting field on the MachineSet is "flavor". We are opting to stay consistent
    /// with the end result.
    pub flavor: String,
    /// RootVolume defines the root volume for instances in the machine pool.
    /// The instances use ephemeral disks if not set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootVolume")]
    pub root_volume: Option<MachinePoolPlatformOpenstackRootVolume>,
}

/// RootVolume defines the root volume for instances in the machine pool.
/// The instances use ephemeral disks if not set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformOpenstackRootVolume {
    /// Size defines the size of the volume in gibibytes (GiB).
    /// Required
    pub size: i64,
    /// Type defines the type of the volume.
    /// Required
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Ovirt is the configuration used when installing on oVirt.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformOvirt {
    /// CPU defines the VM CPU.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<MachinePoolPlatformOvirtCpu>,
    /// MemoryMB is the size of a VM's memory in MiBs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryMB")]
    pub memory_mb: Option<i32>,
    /// OSDisk is the the root disk of the node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "osDisk")]
    pub os_disk: Option<MachinePoolPlatformOvirtOsDisk>,
    /// VMType defines the workload type of the VM.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vmType")]
    pub vm_type: Option<MachinePoolPlatformOvirtVmType>,
}

/// CPU defines the VM CPU.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformOvirtCpu {
    /// Cores is the number of cores per socket.
    /// Total CPUs is (Sockets * Cores)
    pub cores: i32,
    /// Sockets is the number of sockets for a VM.
    /// Total CPUs is (Sockets * Cores)
    pub sockets: i32,
}

/// OSDisk is the the root disk of the node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformOvirtOsDisk {
    /// SizeGB size of the bootable disk in GiB.
    #[serde(rename = "sizeGB")]
    pub size_gb: i64,
}

/// Ovirt is the configuration used when installing on oVirt.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MachinePoolPlatformOvirtVmType {
    #[serde(rename = "")]
    KopiumEmpty,
    #[serde(rename = "desktop")]
    Desktop,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "high_performance")]
    HighPerformance,
}

/// VSphere is the configuration used when installing on vSphere
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformVsphere {
    /// NumCoresPerSocket is the number of cores per socket in a vm. The number
    /// of vCPUs on the vm will be NumCPUs/NumCoresPerSocket.
    #[serde(rename = "coresPerSocket")]
    pub cores_per_socket: i32,
    /// NumCPUs is the total number of virtual processor cores to assign a vm.
    pub cpus: i32,
    /// Memory is the size of a VM's memory in MB.
    #[serde(rename = "memoryMB")]
    pub memory_mb: i64,
    /// OSDisk defines the storage for instance.
    #[serde(rename = "osDisk")]
    pub os_disk: MachinePoolPlatformVsphereOsDisk,
    /// ResourcePool is the name of the resource pool that will be used for virtual machines.
    /// If it is not present, a default value will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePool")]
    pub resource_pool: Option<String>,
}

/// OSDisk defines the storage for instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolPlatformVsphereOsDisk {
    /// DiskSizeGB defines the size of disk in GB.
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i32,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// MachinePoolStatus defines the observed state of MachinePool
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolStatus {
    /// Conditions includes more detailed status for the cluster deployment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ControlledByReplica indicates which replica of the hive-machinepool StatefulSet is responsible
    /// for this MachinePool. Note that this value indicates the replica that most recently handled the
    /// MachinePool. If the hive-machinepool statefulset is scaled up or down, the controlling replica
    /// can change, potentially causing logs to be spread across multiple pods.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlledByReplica")]
    pub controlled_by_replica: Option<i64>,
    /// MachineSets is the status of the machine sets for the machine pool on the remote cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "machineSets")]
    pub machine_sets: Option<Vec<MachinePoolStatusMachineSets>>,
    /// OwnedLabels lists the keys of labels this MachinePool created on the remote MachineSet's
    /// MachineSpec. (In contrast with OwnedMachineLabels.)
    /// Used to identify labels to remove from the remote MachineSet when they are absent from
    /// the MachinePool's spec.labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownedLabels")]
    pub owned_labels: Option<Vec<String>>,
    /// OwnedMachineLabels lists the keys of labels this MachinePool created on the remote
    /// MachineSet's MachineTemplateSpec. (In contrast with OwnedLabels.)
    /// Used to identify labels to remove from the remote MachineSet when they are absent from
    /// the MachinePool's spec.machineLabels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownedMachineLabels")]
    pub owned_machine_labels: Option<Vec<String>>,
    /// OwnedTaints lists identifiers of taints this MachinePool created on the remote MachineSet.
    /// Used to identify taints to remove from the remote MachineSet when they are absent from
    /// the MachinePool's spec.taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownedTaints")]
    pub owned_taints: Option<Vec<MachinePoolStatusOwnedTaints>>,
    /// Replicas is the current number of replicas for the machine pool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}

/// MachineSetStatus is the status of a machineset in the remote cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolStatusMachineSets {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMessage")]
    pub error_message: Option<String>,
    /// In the event that there is a terminal problem reconciling the
    /// replicas, both ErrorReason and ErrorMessage will be set. ErrorReason
    /// will be populated with a succinct value suitable for machine
    /// interpretation, while ErrorMessage will contain a more verbose
    /// string suitable for logging and human consumption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorReason")]
    pub error_reason: Option<String>,
    /// MaxReplicas is the maximum number of replicas for the machine set.
    #[serde(rename = "maxReplicas")]
    pub max_replicas: i32,
    /// MinReplicas is the minimum number of replicas for the machine set.
    #[serde(rename = "minReplicas")]
    pub min_replicas: i32,
    /// Name is the name of the machine set.
    pub name: String,
    /// The number of ready replicas for this MachineSet. A machine is considered ready
    /// when the node has been created and is "Ready". It is transferred as-is from the
    /// MachineSet from remote cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    /// Replicas is the current number of replicas for the machine set.
    pub replicas: i32,
}

/// TaintIdentifier uniquely identifies a Taint. (It turns out taints are mutually exclusive by
/// key+effect, not simply by key.)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachinePoolStatusOwnedTaints {
    /// Effect matches corev1.Taint.Effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key matches corev1.Taint.Key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

