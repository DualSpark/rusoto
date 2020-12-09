use inflector::Inflector;

use crate::botocore::{Member, Operation, Shape, ShapeType};
use crate::Service;
use super::GenerateProtocol;

// Rest Response Parser
//
// Used by rest-json and rest-xml protocol codegen to generate
// code to parse headers from the http response.
pub fn generate_response_headers_parser(
    service: &Service<'_>,
    operation: &Operation,
    protocol: &dyn GenerateProtocol,
) -> Option<String> {
    // nothing to do if there's no output type
    operation.output.as_ref()?;

    let shape = service
        .get_shape(&operation.output.as_ref().unwrap().shape)
        .unwrap();
    let members = shape.members.as_ref().unwrap();

    let parser_pieces = members
        .iter()
        .filter_map(
            |(member_name, member)| match member.location.as_ref().map(String::as_ref) {
                Some("header") => Some(parse_single_header(service, protocol, shape, member_name, member)),
                Some("headers") => {
                    Some(parse_multiple_headers(service, shape, member_name, member))
                }
                _ => None,
            },
        )
        .collect::<Vec<String>>();

    if !parser_pieces.is_empty() {
        Some(parser_pieces.join("\n"))
    } else {
        None
    }
}

fn parse_multiple_headers(
    service: &Service<'_>,
    shape: &Shape,
    member_name: &str,
    member: &Member,
) -> String {
    let member_shape = service.get_shape(&member.shape).unwrap();
    let required = shape.required(member_name);
    match member_shape.shape_type {
        ShapeType::Map => parse_headers_map(member_name, member, required),
        ShapeType::List => parse_headers_list(member_name, member, required),
        shape_type => panic!(
            "Unknown header shape type {:?} for {}",
            shape_type, member_name
        ),
    }
}

fn parse_headers_list(member_name: &str, member: &Member, required: bool) -> String {
    let set_statement = if required {
        format!("result.{} = values;", member_name.to_snake_case())
    } else {
        format!("result.{} = Some(values);", member_name.to_snake_case())
    };

    format!(
        "let mut values = Vec::new();
            for (key, value) in response.headers.iter() {{
              if key == \"{location_name}\" {{
                values.push(value);
              }}
            }}
            {set_statement}",
        location_name = member.location_name.as_ref().unwrap(),
        set_statement = set_statement
    )
}

fn parse_headers_map(member_name: &str, member: &Member, required: bool) -> String {
    let set_statement = if required {
        format!("result.{} = values;", member_name.to_snake_case())
    } else {
        format!("result.{} = Some(values);", member_name.to_snake_case())
    };

    format!(
        "let mut values = ::std::collections::HashMap::new();
    for (key, value) in response.headers.iter() {{
        if key.as_str().starts_with(\"{location_name}\") {{
            values.insert(key.as_str()[\"{location_name}\".len()..].to_owned(), value.to_owned());
        }}
    }}
    {set_statement}",
        location_name = member.location_name.as_ref().unwrap(),
        set_statement = set_statement
    )
}

fn parse_single_header(
    service: &Service<'_>,
    protocol: &dyn GenerateProtocol,
    shape: &Shape,
    member_name: &str,
    member: &Member,
) -> String {
    let member_shape = service.get_shape(&member.shape).unwrap();
    if shape.required(member_name) {
        format!(
            "let value = response.headers.remove(\"{location_name}\").unwrap();
                 result.{field_name} = {primitive_parser};",
            location_name = member.location_name.as_ref().unwrap(),
            field_name = member_name.to_snake_case(),
            primitive_parser = generate_header_primitive_parser(protocol, member_shape).unwrap_or("value")
        )
    } else {
        format!(
            "result.{field_name} = response.headers.remove(\"{location_name}\"){primitive_parser};",
            location_name = member.location_name.as_ref().unwrap(),
            field_name = member_name.to_snake_case(),
            primitive_parser = generate_header_primitive_parser(protocol, member_shape)
                .map(|s| format!(".map(|value| {})", s))
                .unwrap_or_default()
        )
    }
}

fn generate_header_primitive_parser(protocol: &dyn GenerateProtocol, shape: &Shape) -> Option<&'static str> {
    let statement = match shape.shape_type {
        ShapeType::Timestamp if protocol.timestamp_type() == "f64" => "value.parse::<f64>().unwrap()",
        ShapeType::String | ShapeType::Timestamp => return None,
        ShapeType::Double => "value.parse::<f64>().unwrap()",
        ShapeType::Integer | ShapeType::Long => "value.parse::<i64>().unwrap()",
        ShapeType::Float => "value.parse::<f32>().unwrap()",
        ShapeType::Boolean => "value.parse::<bool>().unwrap()",
        _ => panic!("Unknown primitive shape type"),
    };

    Some(statement)
}
