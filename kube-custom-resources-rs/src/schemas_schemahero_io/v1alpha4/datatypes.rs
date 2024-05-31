// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/schemahero/schemahero/schemas.schemahero.io/v1alpha4/datatypes.yaml --derive=Default --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DataTypeSpec defines the desired state of Type
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "schemas.schemahero.io", version = "v1alpha4", kind = "DataType", plural = "datatypes")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DataTypeSpec {
    pub database: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<DataTypeSchema>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataTypeSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cassandra: Option<DataTypeSchemaCassandra>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataTypeSchemaCassandra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DataTypeSchemaCassandraFields>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDeleted")]
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataTypeSchemaCassandraFields {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// DataTypeStatus defines the observed state of Type
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DataTypeStatus {
}

