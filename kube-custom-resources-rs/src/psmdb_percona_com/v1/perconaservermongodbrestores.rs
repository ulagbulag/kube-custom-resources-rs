// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/percona/percona-server-mongodb-operator/psmdb.percona.com/v1/perconaservermongodbrestores.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "psmdb.percona.com", version = "v1", kind = "PerconaServerMongoDBRestore", plural = "perconaservermongodbrestores")]
#[kube(namespaced)]
#[kube(status = "PerconaServerMongoDBRestoreStatus")]
#[kube(schema = "disabled")]
pub struct PerconaServerMongoDBRestoreSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupName")]
    pub backup_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSource")]
    pub backup_source: Option<PerconaServerMongoDBRestoreBackupSource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pitr: Option<PerconaServerMongoDBRestorePitr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBRestoreBackupSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<PerconaServerMongoDBRestoreBackupSourceAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransition")]
    pub last_transition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pbmName")]
    pub pbm_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pbmPod")]
    pub pbm_pod: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replsetNames")]
    pub replset_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<PerconaServerMongoDBRestoreBackupSourceS3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBRestoreBackupSourceAzure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBRestoreBackupSourceS3 {
    pub bucket: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insecureSkipTLSVerify")]
    pub insecure_skip_tls_verify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUploadParts")]
    pub max_upload_parts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverSideEncryption")]
    pub server_side_encryption: Option<PerconaServerMongoDBRestoreBackupSourceS3ServerSideEncryption>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uploadPartSize")]
    pub upload_part_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBRestoreBackupSourceS3ServerSideEncryption {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseAlgorithm")]
    pub sse_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseCustomerAlgorithm")]
    pub sse_customer_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseCustomerKey")]
    pub sse_customer_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBRestorePitr {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBRestoreStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransition")]
    pub last_transition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pbmName")]
    pub pbm_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pitrTarget")]
    pub pitr_target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

