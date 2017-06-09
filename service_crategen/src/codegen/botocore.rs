use std::collections::{BTreeSet, BTreeMap};
use std::error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json;

use super::serialization::{ShapesMap, ShapeName};

const BOTOCORE_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/botocore/botocore/data/");

#[derive(Debug, Deserialize)]
pub struct ServiceDefinition {
    pub documentation: Option<String>,
    pub examples: Option<BTreeMap<String, String>>,
    pub metadata: Metadata,
    pub operations: BTreeMap<String, Operation>,
    #[serde(deserialize_with="ShapesMap::deserialize_shapes_map")]
    pub shapes: BTreeMap<String, Shape>,
    pub version: Option<String>
}

impl ServiceDefinition {
    pub fn load(name: &str, protocol_version: &str) -> Result<Self, Box<error::Error>> {
        let input_path = Path::new(BOTOCORE_DIR)
            .join(format!("{}/{}/service-2.json", name, protocol_version));

        let input_file = BufReader::new(File::open(&input_path)?);

        let service: ServiceDefinition = serde_json::from_reader(input_file)?;

        Ok(service)
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    #[serde(rename="requestUri")]
    pub request_uri: String,
    #[serde(rename="responseCode")]
    pub response_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub documentation: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub documentation: Option<String>,
    #[serde(rename="resultWrapper")]
    pub result_wrapper: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub shape: String,
}

impl Error {
    pub fn idiomatic_error_name(&self) -> String {
        self.shape.replace("Exception", "")
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HttpError {
    pub code: Option<String>,
    #[serde(rename="httpStatusCode")]
    pub http_status_code: i32,
    #[serde(rename="senderFault")]
    pub sender_fault: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    pub flattened: Option<bool>,
    pub location: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
    pub streaming: Option<bool>,
    #[serde(rename="xmlAttribute")]
    pub xml_attribute: Option<bool>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Member {
    pub fn deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    pub fn streaming(&self) -> bool {
        self.streaming.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct XmlNamespace {
    pub prefix: Option<String>,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Key {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub required: Option<bool>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

impl Key {
    pub fn tag_name(&self) -> String {
        self.location_name.as_ref().map(String::as_ref).unwrap_or_else(|| "key").to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub documentation: Option<String>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    #[serde(deserialize_with="ShapeName::deserialize_shape_name")]
    pub shape: String,
}

impl Value {
    pub fn tag_name(&self) -> String {
        self.location_name.as_ref().map(String::as_ref).unwrap_or_else(|| "value").to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    #[serde(rename="box")]
    pub aws_box: Option<bool>,
    pub documentation: Option<String>,
    pub error: Option<HttpError>,
    pub exception: Option<bool>,
    pub fault: Option<bool>,
    pub flattened: Option<bool>,
    pub key: Option<Key>,
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    pub max: Option<f64>,
    pub member: Option<Member>,
    pub members: Option<BTreeMap<String, Member>>,
    pub min: Option<f32>,
    pub pattern: Option<String>,
    pub payload: Option<String>,
    pub required: Option<Vec<String>>,
    #[serde(rename="enum")]
    pub shape_enum: Option<Vec<String>>,
    #[serde(rename="type")]
    pub shape_type: ShapeType,
    pub sensitive: Option<bool>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    pub value: Option<Value>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<XmlNamespace>,
}

impl Shape {
    pub fn is_primitive(&self) -> bool {
        match self.shape_type {
            ShapeType::Structure | ShapeType::Map | ShapeType::List => false,
            _ => true,
        }
    }
}

impl<'a> Shape {
    pub fn key_type(&'a self) -> &'a str {
        &self.key.as_ref().expect("Key shape undefined").shape
    }

    pub fn value_type(&'a self) -> &'a str {
        &self.value.as_ref().expect("Value shape undefined").shape
    }

    pub fn member_type(&'a self) -> &'a str {
        &self.member.as_ref().expect("Member shape undefined").shape
    }

    pub fn required(&self, field: &'a str) -> bool {
        self.required.is_some() && self.required.as_ref().unwrap().contains(&String::from(field))
    }

    pub fn exception(&self) -> bool {
        self.exception.unwrap_or(false)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ShapeType {
    #[serde(rename="blob")]
    Blob,
    #[serde(rename="boolean")]
    Boolean,
    #[serde(rename="double")]
    Double,
    #[serde(rename="float")]
    Float,
    #[serde(rename="integer")]
    Integer,
    #[serde(rename="list")]
    List,
    #[serde(rename="long")]
    Long,
    #[serde(rename="map")]
    Map,
    #[serde(rename="string")]
    String,
    #[serde(rename="structure")]
    Structure,
    #[serde(rename="timestamp")]
    Timestamp,
}

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub alias: Option<String>,
    pub deprecated: Option<bool>,
    pub documentation: Option<String>,
    #[serde(rename="documentationUrl")]
    pub documentation_url: Option<String>,
    pub errors: Option<BTreeSet<Error>>,
    pub http: HttpRequest,
    pub input: Option<Input>,
    pub name: String,
    pub output: Option<Output>,
}

impl<'a> Operation {
    pub fn input_shape(&'a self) -> &'a str {
        &self.input.as_ref().expect("Operation input undefined").shape
    }

    pub fn output_shape_or(&'a self, default: &'a str) -> &'a str {
        match self.output.as_ref() {
            Some(output) => &output.shape,
            None => default,
        }
    }

    // botocore duplicates errors in a few places
    // return a unique set
    pub fn errors(&'a self) -> &BTreeSet<Error> {
        self.errors.as_ref().unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename="apiVersion")]
    pub api_version: String,
    #[serde(rename="checksumFormat")]
    pub checksum_format: Option<String>,
    #[serde(rename="endpointPrefix")]
    pub endpoint_prefix: String,
    #[serde(rename="globalEndpoint")]
    pub global_endpoint: Option<String>,
    #[serde(rename="jsonVersion")]
    pub json_version: Option<String>,
    pub protocol: String,
    #[serde(rename="serviceAbbreviation")]
    pub service_abbreviation: Option<String>,
    #[serde(rename="serviceFullName")]
    pub service_full_name: String,
    #[serde(rename="signatureVersion")]
    pub signature_version: String,
    #[serde(rename="signingName")]
    pub signing_name: Option<String>,
    #[serde(rename="targetPrefix")]
    pub target_prefix: Option<String>,
    #[serde(rename="timestampFormat")]
    pub timestamp_format: Option<String>,
    #[serde(rename="xmlNamespace")]
    pub xml_namespace: Option<String>,
}
