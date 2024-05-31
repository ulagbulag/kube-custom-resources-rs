// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluent/fluent-operator/fluentd.fluent.io/v1alpha1/clusterfilters.yaml --derive=PartialEq
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ClusterFilterSpec defines the desired state of ClusterFilter
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "fluentd.fluent.io", version = "v1alpha1", kind = "ClusterFilter", plural = "clusterfilters")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ClusterFilterSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ClusterFilterFilters>>,
}

/// Filter defines all available filter plugins and their parameters.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFilters {
    /// Custom plugin type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customPlugin")]
    pub custom_plugin: Option<ClusterFilterFiltersCustomPlugin>,
    /// The filter_grep filter plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grep: Option<ClusterFilterFiltersGrep>,
    /// The @log_level parameter specifies the plugin-specific logging level
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    /// The filter_parser filter plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parser: Option<ClusterFilterFiltersParser>,
    /// The filter_record_transformer filter plugin
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordTransformer")]
    pub record_transformer: Option<ClusterFilterFiltersRecordTransformer>,
    /// The filter_stdout filter plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout: Option<ClusterFilterFiltersStdout>,
    /// Which tag to be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Custom plugin type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersCustomPlugin {
    pub config: String,
}

/// The filter_grep filter plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<ClusterFilterFiltersGrepAnd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<ClusterFilterFiltersGrepExclude>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<ClusterFilterFiltersGrepOr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<Vec<ClusterFilterFiltersGrepRegexp>>,
}

/// And defines the parameters for the "and" plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepAnd {
    /// Exclude defines the parameters for the exclude plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<ClusterFilterFiltersGrepAndExclude>,
    /// Regexp defines the parameters for the regexp plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<ClusterFilterFiltersGrepAndRegexp>,
}

/// Exclude defines the parameters for the exclude plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepAndExclude {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// Regexp defines the parameters for the regexp plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepAndRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// Exclude defines the parameters for the exclude plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepExclude {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// Or defines the parameters for the "or" plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepOr {
    /// Exclude defines the parameters for the exclude plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<ClusterFilterFiltersGrepOrExclude>,
    /// Regexp defines the parameters for the regexp plugin
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regexp: Option<ClusterFilterFiltersGrepOrRegexp>,
}

/// Exclude defines the parameters for the exclude plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepOrExclude {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// Regexp defines the parameters for the regexp plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepOrRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// Regexp defines the parameters for the regexp plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersGrepRegexp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// The filter_parser filter plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersParser {
    /// Emits invalid record to @ERROR label. Invalid cases are: key does not exist;the format is not matched;an unexpected error. If you want to ignore these errors, set false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emitInvalidRecordToError")]
    pub emit_invalid_record_to_error: Option<bool>,
    /// Stores the parsed values as a hash value in a field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hashValueField")]
    pub hash_value_field: Option<String>,
    /// Stores the parsed values with the specified key name prefix.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "injectKeyPrefix")]
    pub inject_key_prefix: Option<String>,
    /// Specifies the field name in the record to parse. Required parameter. i.e: If set keyName to log, {"key":"value","log":"{\"time\":1622473200,\"user\":1}"} => {"user":1}
    #[serde(rename = "keyName")]
    pub key_name: String,
    /// Parse defines various parameters for the parse plugin
    pub parse: ClusterFilterFiltersParserParse,
    /// Removes key_name field when parsing is succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "removeKeyNameField")]
    pub remove_key_name_field: Option<bool>,
    /// If true, invalid string is replaced with safe characters and re-parse it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replaceInvalidSequence")]
    pub replace_invalid_sequence: Option<bool>,
    /// Keeps the original key-value pair in the parsed result. Default is false. i.e: If set keyName to log, reverseData to true, {"key":"value","log":"{\"user\":1,\"num\":2}"} => {"key":"value","log":"{\"user\":1,\"num\":2}","user":1,"num":2}
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reserveData")]
    pub reserve_data: Option<bool>,
    /// Keeps the original event time in the parsed result. Default is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reserveTime")]
    pub reserve_time: Option<bool>,
}

/// Parse defines various parameters for the parse plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersParserParse {
    /// Path to the file that includes custom grok patterns.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customPatternPath")]
    pub custom_pattern_path: Option<String>,
    /// If true, use Fluent::Eventnow(current time) as a timestamp when time_key is specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "estimateCurrentEvent")]
    pub estimate_current_event: Option<bool>,
    /// Specifies the regular expression for matching logs. Regular expression also supports i and m suffix.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Grok Sections
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grok: Option<Vec<ClusterFilterFiltersParserParseGrok>>,
    /// The key has grok failure reason.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grokFailureKey")]
    pub grok_failure_key: Option<String>,
    /// The pattern of grok.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grokPattern")]
    pub grok_pattern: Option<String>,
    /// Specify grok pattern series set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grokPatternSeries")]
    pub grok_pattern_series: Option<String>,
    /// The @id parameter specifies a unique name for the configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If true, keep time field in th record.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepTimeKey")]
    pub keep_time_key: Option<bool>,
    /// If true, uses local time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localtime: Option<bool>,
    /// The @log_level parameter specifies the plugin-specific logging level
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    /// The regexp to match beginning of multiline. This is only for "multiline_grok".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiLineStartRegexp")]
    pub multi_line_start_regexp: Option<String>,
    /// Process value according to the specified format. This is available only when time_type is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormat")]
    pub time_format: Option<String>,
    /// Uses the specified time format as a fallback in the specified order. You can parse undetermined time format by using time_format_fallbacks. This options is enabled when time_type is mixed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormatFallbacks")]
    pub time_format_fallbacks: Option<String>,
    /// Specify time field for event time. If the event doesn't have this field, current time is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeKey")]
    pub time_key: Option<String>,
    /// parses/formats value according to this type, default is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeType")]
    pub time_type: Option<ClusterFilterFiltersParserParseTimeType>,
    /// Specify timeout for parse processing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Uses the specified timezone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The @type parameter specifies the type of the plugin.
    #[serde(rename = "type")]
    pub r#type: ClusterFilterFiltersParserParseType,
    /// Specify types for converting field into another, i.e: types user_id:integer,paid:bool,paid_usd_amount:float
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// If true, uses UTC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utc: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersParserParseGrok {
    /// If true, keep time field in the record.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepTimeKey")]
    pub keep_time_key: Option<bool>,
    /// The name of this grok section.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The pattern of grok. Required parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Process value using specified format. This is available only when time_type is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormat")]
    pub time_format: Option<String>,
    /// Specify time field for event time. If the event doesn't have this field, current time is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeKey")]
    pub time_key: Option<String>,
    /// Use specified timezone. one can parse/format the time value in the specified timezone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
}

/// Parse defines various parameters for the parse plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterFilterFiltersParserParseTimeType {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "unixtime")]
    Unixtime,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "mixed")]
    Mixed,
}

/// Parse defines various parameters for the parse plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterFilterFiltersParserParseType {
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "apache2")]
    Apache2,
    #[serde(rename = "apache_error")]
    ApacheError,
    #[serde(rename = "nginx")]
    Nginx,
    #[serde(rename = "syslog")]
    Syslog,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "tsv")]
    Tsv,
    #[serde(rename = "ltsv")]
    Ltsv,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "multiline")]
    Multiline,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "grok")]
    Grok,
    #[serde(rename = "multiline_grok")]
    MultilineGrok,
}

/// The filter_record_transformer filter plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersRecordTransformer {
    /// Automatically casts the field types. Default is false. This option is effective only for field values comprised of a single placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoTypecast")]
    pub auto_typecast: Option<bool>,
    /// When set to true, the full Ruby syntax is enabled in the ${...} expression. The default value is false. i.e: jsonized_record ${record.to_json}
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableRuby")]
    pub enable_ruby: Option<bool>,
    /// A list of keys to keep. Only relevant if renew_record is set to true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepKeys")]
    pub keep_keys: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<ClusterFilterFiltersRecordTransformerRecords>>,
    /// A list of keys to delete. Supports nested field via record_accessor syntax since v1.1.0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "removeKeys")]
    pub remove_keys: Option<String>,
    /// By default, the record transformer filter mutates the incoming data. However, if this parameter is set to true, it modifies a new empty hash instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renewRecord")]
    pub renew_record: Option<bool>,
    /// renew_time_key foo overwrites the time of events with a value of the record field foo if exists. The value of foo must be a Unix timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renewTimeKey")]
    pub renew_time_key: Option<String>,
}

/// The parameters inside <record> directives are considered to be new key-value pairs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersRecordTransformerRecords {
    /// New field can be defined as key
    pub key: String,
    /// The value must from Record properties. See https://docs.fluentd.org/filter/record_transformer#less-than-record-greater-than-directive
    pub value: String,
}

/// The filter_stdout filter plugin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersStdout {
    /// The format section
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ClusterFilterFiltersStdoutFormat>,
    /// The inject section
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inject: Option<ClusterFilterFiltersStdoutInject>,
}

/// The format section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersStdoutFormat {
    /// Delimiter for each field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// The @id parameter specifies a unique name for the configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If true, uses local time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localtime: Option<bool>,
    /// The @log_level parameter specifies the plugin-specific logging level
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    /// Specify newline characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub newline: Option<ClusterFilterFiltersStdoutFormatNewline>,
    /// Output tag field if true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputTag")]
    pub output_tag: Option<bool>,
    /// Output time field if true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputTime")]
    pub output_time: Option<bool>,
    /// Process value according to the specified format. This is available only when time_type is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormat")]
    pub time_format: Option<String>,
    /// Uses the specified time format as a fallback in the specified order. You can parse undetermined time format by using time_format_fallbacks. This options is enabled when time_type is mixed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormatFallbacks")]
    pub time_format_fallbacks: Option<String>,
    /// parses/formats value according to this type, default is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeType")]
    pub time_type: Option<ClusterFilterFiltersStdoutFormatTimeType>,
    /// Uses the specified timezone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The @type parameter specifies the type of the plugin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ClusterFilterFiltersStdoutFormatType>,
    /// If true, uses UTC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utc: Option<bool>,
}

/// The format section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterFilterFiltersStdoutFormatNewline {
    #[serde(rename = "lf")]
    Lf,
    #[serde(rename = "crlf")]
    Crlf,
}

/// The format section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterFilterFiltersStdoutFormatTimeType {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "unixtime")]
    Unixtime,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "mixed")]
    Mixed,
}

/// The format section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterFilterFiltersStdoutFormatType {
    #[serde(rename = "out_file")]
    OutFile,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "ltsv")]
    Ltsv,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "msgpack")]
    Msgpack,
    #[serde(rename = "hash")]
    Hash,
    #[serde(rename = "single_value")]
    SingleValue,
}

/// The inject section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersStdoutInject {
    /// Hostname value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The field name to inject hostname
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostnameKey")]
    pub hostname_key: Option<String>,
    /// Time section
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<ClusterFilterFiltersStdoutInjectInline>,
    /// The field name to inject tag
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagKey")]
    pub tag_key: Option<String>,
    /// The field name to inject time
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeKey")]
    pub time_key: Option<String>,
    /// The field name to inject worker_id
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workerIdKey")]
    pub worker_id_key: Option<String>,
}

/// Time section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterFiltersStdoutInjectInline {
    /// If true, uses local time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localtime: Option<bool>,
    /// Process value according to the specified format. This is available only when time_type is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormat")]
    pub time_format: Option<String>,
    /// Uses the specified time format as a fallback in the specified order. You can parse undetermined time format by using time_format_fallbacks. This options is enabled when time_type is mixed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeFormatFallbacks")]
    pub time_format_fallbacks: Option<String>,
    /// parses/formats value according to this type, default is string
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeType")]
    pub time_type: Option<ClusterFilterFiltersStdoutInjectInlineTimeType>,
    /// Uses the specified timezone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// If true, uses UTC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utc: Option<bool>,
}

/// Time section
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterFilterFiltersStdoutInjectInlineTimeType {
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "unixtime")]
    Unixtime,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "mixed")]
    Mixed,
}

/// ClusterFilterStatus defines the observed state of ClusterFilter
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterFilterStatus {
}

