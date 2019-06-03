use std::collections::BTreeMap;

use crate::botocore::{Member, Operation, ServiceDefinition, Shape, ShapeType, Value, Input, Output};
use crate::cargo;
use crate::config::ServiceConfig;

#[derive(Debug)]
pub struct Service<'a> {
    config: &'a crate::ServiceConfig,
    definition: ServiceDefinition,
}

impl<'b> Service<'b> {
    pub fn new(config: &'b ServiceConfig, definition: ServiceDefinition) -> Self {
        Service { config, definition }
    }

    pub fn name(&self) -> &str {
        match self.definition.metadata.service_abbreviation {
            Some(ref service_abbreviation) => service_abbreviation.as_str(),
            None => self.definition.metadata.service_full_name.as_ref(),
        }
    }

    pub fn full_name(&self) -> &str {
        &self.definition.metadata.service_full_name
    }

    pub fn service_id(&self) -> Option<&str> {
        self.definition
            .metadata
            .service_id
            .as_ref()
            .map(std::string::String::as_str)
    }

    pub fn documentation(&self) -> Option<&String> {
        self.definition.documentation.as_ref()
    }

    pub fn protocol(&self) -> &str {
        &self.definition.metadata.protocol
    }

    pub fn client_type_name(&self) -> String {
        format!("{}Client", self.service_type_name())
    }

    pub fn service_type_name(&self) -> &str {
        &self.config.base_type_name
    }

    pub fn endpoint_prefix(&self) -> &str {
        &self.definition.metadata.endpoint_prefix
    }

    pub fn api_version(&self) -> &str {
        &self.definition.metadata.api_version
    }

    pub fn target_prefix(&self) -> Option<&String> {
        self.definition.metadata.target_prefix.as_ref()
    }

    pub fn json_version(&self) -> Option<&String> {
        self.definition.metadata.json_version.as_ref()
    }

    pub fn shapes(&self) -> &BTreeMap<String, Shape> {
        &self.definition.shapes
    }

    pub fn get_shape(&self, name: &str) -> Option<&Shape> {
        self.definition.shapes.get(name)
    }

    pub fn operations(&self) -> &BTreeMap<String, Operation> {
        &self.definition.operations
    }

    pub fn shape_for_value<'a>(&'a self, value: &Value) -> Option<&'a Shape> {
        self.definition.shapes.get(&value.shape)
    }

    pub fn shape_for_member<'a>(&'a self, member: &Member) -> Option<&'a Shape> {
        self.definition.shapes.get(&member.shape)
    }

    pub fn shape_type_for_member<'a>(&'a self, member: &Member) -> Option<ShapeType> {
        self.definition
            .shapes
            .get(&member.shape)
            .map(|shape| shape.shape_type)
    }

    pub fn signing_name(&self) -> String {
        match self.definition.metadata.signing_name {
            Some(ref signing_name) => signing_name.to_string(),
            None => self.definition.metadata.endpoint_prefix.to_string(),
        }
    }

    pub fn needs_serde_json_crate(&self) -> bool {
        match self.name() {
            "AmazonApiGatewayManagementApi"
            | "Amazon CloudSearch Domain"
            | "AWS Mobile"
            | "AWS IoT Data Plane"
            | "Amazon SageMaker Runtime" => false,
            _ => true,
        }
    }

    pub fn get_dependencies(&self) -> BTreeMap<String, cargo::Dependency> {
        let mut dependencies = BTreeMap::new();

        dependencies.insert(
            "bytes".to_owned(),
            cargo::Dependency::Simple("0.4.12".into()),
        );
        dependencies.insert(
            "futures".to_owned(),
            cargo::Dependency::Simple("0.1.16".into()),
        );
        dependencies.insert(
            "rusoto_core".to_owned(),
            cargo::Dependency::Extended {
                path: Some("../../core".into()),
                version: Some(self.config.core_version.clone()),
                optional: None,
                default_features: Some(false),
                features: None,
            },
        );

        match self.protocol() {
            "json" => {
                dependencies.insert(
                    "serde".to_owned(),
                    cargo::Dependency::Simple("1.0.2".into()),
                );
                dependencies.insert(
                    "serde_derive".to_owned(),
                    cargo::Dependency::Simple("1.0.2".into()),
                );
                dependencies.insert(
                    "serde_json".to_owned(),
                    cargo::Dependency::Simple("1.0.1".into()),
                );
            }
            "query" | "ec2" => {
                dependencies.insert(
                    "serde_urlencoded".to_owned(),
                    cargo::Dependency::Simple("0.5".into()),
                );
                dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.7".into()));
            }
            "rest-xml" => {
                dependencies.insert("xml-rs".to_owned(), cargo::Dependency::Simple("0.7".into()));
            }
            "rest-json" => {
                dependencies.insert(
                    "serde".to_owned(),
                    cargo::Dependency::Simple("1.0.2".into()),
                );
                dependencies.insert(
                    "serde_derive".to_owned(),
                    cargo::Dependency::Simple("1.0.2".into()),
                );
                // some rest-json services don't use the `serde_json` crate:
                if self.needs_serde_json_crate() {
                    dependencies.insert(
                        "serde_json".to_owned(),
                        cargo::Dependency::Simple("1.0.1".into()),
                    );
                }
            }
            protocol => panic!("Unknown protocol {}", protocol),
        }

        if let Some(ref custom_dependencies) = self.config.custom_dependencies {
            dependencies.extend(custom_dependencies.clone());
        }

        dependencies
    }

    pub fn get_dev_dependencies(&self) -> BTreeMap<String, cargo::Dependency> {
        let mut dev_dependencies = BTreeMap::new();

        dev_dependencies.insert(
            "rusoto_mock".to_owned(),
            cargo::Dependency::Extended {
                path: Some("../../../mock".into()),
                version: Some("0.39.0".into()),
                optional: None,
                default_features: None,
                features: None,
            },
        );

        if let Some(ref custom_dev_dependencies) = self.config.custom_dev_dependencies {
            dev_dependencies.extend(custom_dev_dependencies.clone());
        }

        dev_dependencies
    }

    pub fn visit_shapes<F>(&self, shape_name: &str, visitor: &mut F)
    where
        F: FnMut(&str, &Shape) -> bool
    {
        let shape = self
            .get_shape(shape_name)
            .expect("Shape missing from service definition");

        if !visitor(shape_name, shape) {
            return;
        }

        match shape.shape_type {
            ShapeType::Structure => {
                if let Some(ref members) = shape.members {
                    for member in members.values() {
                        self.visit_shapes(&member.shape, visitor);
                    }
                }
            }
            ShapeType::Map => {
                self.visit_shapes(shape.key_type(), visitor);
                self.visit_shapes(shape.value_type(), visitor);
            }
            ShapeType::List => {
                self.visit_shapes(shape.member_type(), visitor);
            }
            _ => {}
        }
    }

    pub fn has_non_empty_input_shape(&self, operation: &Operation) -> bool {
        operation.input.is_some()
            && self
                .get_shape(operation.input_shape())
                .as_ref()
                .map(|s| s.is_non_empty())
                .unwrap_or(false)
    }

    pub fn normalize_input_output_shapes(&mut self) {
        let empty_shape = Shape { shape_type: ShapeType::Structure, ..Default::default() };

        for (ref operation_name, ref mut operation) in self.definition.operations.iter_mut() {
            let input_type_name = format!("{}Request", operation_name);
            let output_type_name = format!("{}Response", operation_name);

            let input = operation.input.get_or_insert(Input::default());
            let input_shape = self.definition.shapes.get(&input.shape).cloned().unwrap_or_else(|| empty_shape.clone());
            self.definition.shapes.insert(input_type_name.clone(), input_shape);
            input.shape = input_type_name;

            let output = operation.output.get_or_insert(Output::default());
            let output_shape = self.definition.shapes.get(&output.shape).cloned().unwrap_or_else(|| empty_shape.clone());
            self.definition.shapes.insert(output_type_name.clone(), output_shape);
            output.shape = output_type_name;
        }
    } 
}
