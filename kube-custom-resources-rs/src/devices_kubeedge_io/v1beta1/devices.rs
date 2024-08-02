// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeedge/kubeedge/devices.kubeedge.io/v1beta1/devices.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DeviceSpec represents a single device instance.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "devices.kubeedge.io", version = "v1beta1", kind = "Device", plural = "devices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DeviceSpec {
    /// Required: DeviceModelRef is reference to the device model used as a template to create the device instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceModelRef")]
    pub device_model_ref: Option<DeviceDeviceModelRef>,
    /// NodeName is a request to schedule this device onto a specific node. If it is non-empty, the scheduler simply schedules this device onto that node, assuming that it fits resource requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeName")]
    pub node_name: Option<String>,
    /// List of properties which describe the device properties. properties list item must be unique by properties.Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<DeviceProperties>>,
    /// Required: The protocol configuration used to connect to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<DeviceProtocol>,
}

/// Required: DeviceModelRef is reference to the device model used as a template to create the device instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDeviceModelRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DeviceProperty describes the specifics all the properties of the device.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProperties {
    /// Define how frequent mapper will collect from device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectCycle")]
    pub collect_cycle: Option<i64>,
    /// The desired property value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<DevicePropertiesDesired>,
    /// Required: The device property name to be accessed. It must be unique. Note: If you need to use the built-in stream data processing function, you need to define Name as saveFrame or saveVideo
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// PushMethod represents the protocol used to push data, please ensure that the mapper can access the destination address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pushMethod")]
    pub push_method: Option<DevicePropertiesPushMethod>,
    /// Define how frequent mapper will report the value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportCycle")]
    pub report_cycle: Option<i64>,
    /// whether be reported to the cloud
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportToCloud")]
    pub report_to_cloud: Option<bool>,
    /// Visitors are intended to be consumed by device mappers which connect to devices and collect data / perform actions on the device. Required: Protocol relevant config details about the how to access the device property.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visitors: Option<DevicePropertiesVisitors>,
}

/// The desired property value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesDesired {
    /// Additional metadata like timestamp when the value was reported etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Required: The value for this property.
    pub value: String,
}

/// PushMethod represents the protocol used to push data, please ensure that the mapper can access the destination address.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethod {
    /// DBMethod represents the method used to push data to database, please ensure that the mapper can access the destination address.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbMethod")]
    pub db_method: Option<DevicePropertiesPushMethodDbMethod>,
    /// HTTP Push method configuration for http
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<DevicePropertiesPushMethodHttp>,
    /// MQTT Push method configuration for mqtt
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mqtt: Option<DevicePropertiesPushMethodMqtt>,
}

/// DBMethod represents the method used to push data to database, please ensure that the mapper can access the destination address.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethod {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "TDEngine")]
    pub td_engine: Option<DevicePropertiesPushMethodDbMethodTdEngine>,
    /// method configuration for database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influxdb2: Option<DevicePropertiesPushMethodDbMethodInfluxdb2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mysql: Option<DevicePropertiesPushMethodDbMethodMysql>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redis: Option<DevicePropertiesPushMethodDbMethodRedis>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodTdEngine {
    /// tdengineClientConfig of tdengine database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "TDEngineClientConfig")]
    pub td_engine_client_config: Option<DevicePropertiesPushMethodDbMethodTdEngineTdEngineClientConfig>,
}

/// tdengineClientConfig of tdengine database
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodTdEngineTdEngineClientConfig {
    /// addr of tdEngine database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// dbname of tdEngine database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbName")]
    pub db_name: Option<String>,
}

/// method configuration for database
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodInfluxdb2 {
    /// Config of influx database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "influxdb2ClientConfig")]
    pub influxdb2_client_config: Option<DevicePropertiesPushMethodDbMethodInfluxdb2Influxdb2ClientConfig>,
    /// config of device data when push to influx database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "influxdb2DataConfig")]
    pub influxdb2_data_config: Option<DevicePropertiesPushMethodDbMethodInfluxdb2Influxdb2DataConfig>,
}

/// Config of influx database
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodInfluxdb2Influxdb2ClientConfig {
    /// Bucket of the user in influx database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// Org of the user in influx database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
    /// Url of influx database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// config of device data when push to influx database
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodInfluxdb2Influxdb2DataConfig {
    /// FieldKey of the user data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldKey")]
    pub field_key: Option<String>,
    /// Measurement of the user data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement: Option<String>,
    /// the tag of device data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodMysql {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mysqlClientConfig")]
    pub mysql_client_config: Option<DevicePropertiesPushMethodDbMethodMysqlMysqlClientConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodMysqlMysqlClientConfig {
    /// mysql address,like localhost:3306
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// database name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// user name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodRedis {
    /// RedisClientConfig of redis database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redisClientConfig")]
    pub redis_client_config: Option<DevicePropertiesPushMethodDbMethodRedisRedisClientConfig>,
}

/// RedisClientConfig of redis database
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodDbMethodRedisRedisClientConfig {
    /// Addr of Redis database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// Db of Redis database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db: Option<i64>,
    /// MinIdleConns of Redis database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minIdleConns")]
    pub min_idle_conns: Option<i64>,
    /// Poolsize of Redis database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poolsize: Option<i64>,
}

/// HTTP Push method configuration for http
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodHttp {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostName")]
    pub host_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestPath")]
    pub request_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// MQTT Push method configuration for mqtt
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesPushMethodMqtt {
    /// broker address, like mqtt://127.0.0.1:1883
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// qos of mqtt publish param
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// Is the message retained
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retained: Option<bool>,
    /// publish topic for mqtt
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// Visitors are intended to be consumed by device mappers which connect to devices and collect data / perform actions on the device. Required: Protocol relevant config details about the how to access the device property.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertiesVisitors {
    /// Required: The configData of customized protocol
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configData")]
    pub config_data: Option<BTreeMap<String, serde_json::Value>>,
    /// Required: name of customized protocol
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolName")]
    pub protocol_name: Option<String>,
}

/// Required: The protocol configuration used to connect to the device.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocol {
    /// Any config data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configData")]
    pub config_data: Option<BTreeMap<String, serde_json::Value>>,
    /// Unique protocol name Required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolName")]
    pub protocol_name: Option<String>,
}

/// DeviceStatus reports the device state and the desired/reported values of twin attributes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatus {
    /// Optional: The last time the device was online.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastOnlineTime")]
    pub last_online_time: Option<String>,
    /// Optional: The state of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// A list of device twins containing desired/reported desired/reported values of twin properties. Optional: A passive device won't have twin properties and this list could be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twins: Option<Vec<DeviceStatusTwins>>,
}

/// Twin provides a logical representation of control properties (writable properties in the device model). The properties can have a Desired state and a Reported state. The cloud configures the `Desired`state of a device property and this configuration update is pushed to the edge node. The mapper sends a command to the device to change this property value as per the desired state . It receives the `Reported` state of the property once the previous operation is complete and sends the reported state to the cloud. Offline device interaction in the edge is possible via twin properties for control/command operations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusTwins {
    /// The meaning of here is to indicate desired value of `deviceProperty.Desired` that the mapper has received in current cycle. Useful in cases that people want to check whether the mapper is working appropriately and its internal status is up-to-date. This value should be only updated by devicecontroller upstream.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedDesired")]
    pub observed_desired: Option<DeviceStatusTwinsObservedDesired>,
    /// Required: The property name for which the desired/reported values are specified. This property should be present in the device model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propertyName")]
    pub property_name: Option<String>,
    /// Required: the reported property value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reported: Option<DeviceStatusTwinsReported>,
}

/// The meaning of here is to indicate desired value of `deviceProperty.Desired` that the mapper has received in current cycle. Useful in cases that people want to check whether the mapper is working appropriately and its internal status is up-to-date. This value should be only updated by devicecontroller upstream.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusTwinsObservedDesired {
    /// Additional metadata like timestamp when the value was reported etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Required: The value for this property.
    pub value: String,
}

/// Required: the reported property value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusTwinsReported {
    /// Additional metadata like timestamp when the value was reported etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Required: The value for this property.
    pub value: String,
}

