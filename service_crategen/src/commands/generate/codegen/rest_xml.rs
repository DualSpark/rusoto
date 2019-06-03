use inflector::Inflector;
use std::io::Write;

use super::{error_type_name, generate_field_name, GenerateProtocol};
use super::{
    get_rust_type, mutate_type_name, rest_request_generator, rest_response_parser,
    xml_payload_parser,
};
use super::{FileWriter, IoResult};
use crate::botocore::{Member, Operation, Shape, ShapeType};
use crate::Service;

pub struct RestXmlGenerator;

impl GenerateProtocol for RestXmlGenerator {
    fn generate_method_signatures(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            writeln!(
                writer,
                "
                {documentation}
                {method_signature};
                ",
                documentation = generate_documentation(operation, service),
                method_signature = generate_method_signature(operation_name, operation, service),
            )?
        }
        Ok(())
    }

    fn generate_method_impls(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let input = match service.has_non_empty_input_shape(operation) {
                true => "input".to_owned(),
                false => format!("{} {{}}", operation.input_shape())
            };

            writeln!(writer,
                     "{documentation}
                    {method_signature} {{
                        Request::new({input}, self.region.clone(), self.client.clone())
                    }}
                    ",
                     documentation = generate_documentation(operation, service),
                     method_signature = generate_method_signature(operation_name, operation, service),
                     input = input)?;
        }
        Ok(())
    }

    fn generate_operations(&self, writer: &mut FileWriter, service: &Service<'_>) -> IoResult {
        for (operation_name, operation) in service.operations().iter() {
            let (request_uri, _) =
                rest_request_generator::parse_query_string(&operation.http.request_uri);
            let parse_non_payload =
                rest_response_parser::generate_response_headers_parser(service, operation)
                    .unwrap_or_else(|| "".to_owned());
            writeln!(writer,
                     "
                    impl ServiceRequest for {input_type} {{
                        type Output = {output_type};
                        type Error = {error_type};

                        #[allow(unused_variables, warnings)]
                        fn dispatch(self, region: &region::Region, dispatcher: &impl Dispatcher) -> RusotoFuture<Self::Output, Self::Error> {{
                            {modify_uri}

                            let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", region, &request_uri);

                            {set_headers}
                            {set_parameters}
                            {build_payload}

                            dispatcher.dispatch(request, |response| {{
                                if !response.status.is_success() {{
                                    return Box::new(response.buffer().from_err().and_then(|response| {{
                                        Err({error_type}::from_response(response))
                                    }}));
                                }}

                                {parse_response_body}
                            }})
                        }}
                    }}
                    ",
                     input_type = operation.input_shape_or("()"),
                     output_type = operation.output_shape_or("()"),
                     http_method = &operation.http.method,
                     endpoint_prefix = service.endpoint_prefix(),
                     error_type = error_type_name(service, operation_name),
                     build_payload = generate_payload_serialization(service, operation)
                         .unwrap_or_else(|| "".to_string()),
                     modify_uri = rest_request_generator::generate_uri_formatter(&request_uri,
                                                                                 service,
                                                                                 operation)
                         .unwrap_or_else(|| "".to_string()),
                     set_headers = rest_request_generator::generate_headers(service, operation)
                         .unwrap_or_else(|| "".to_string()),
                     set_parameters =
                         rest_request_generator::generate_params_loading_string(service,
                                                                                operation)
                             .unwrap_or_else(|| "".to_string()),
                     parse_response_body =
                         xml_payload_parser::generate_response_parser(service, operation, true, &parse_non_payload))?;
        }
        Ok(())
    }

    fn generate_prelude(&self, writer: &mut FileWriter, _service: &Service<'_>) -> IoResult {
        let imports = "
            use std::str::{FromStr};
            use std::io::Write;
            use xml::reader::ParserConfig;
            use rusoto_core::param::{Params, ServiceParams};
            use rusoto_core::signature::SignedRequest;
            use xml;
            use xml::EventReader;
            use xml::EventWriter;
            use rusoto_core::proto::xml::error::*;
            use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
            use rusoto_core::proto::xml::util::{peek_at_name, characters, end_element, find_start_element, start_element, skip_tree, deserialize_elements};
            "
            .to_owned();

        writeln!(writer, "{}", imports)
    }

    fn generate_serializer(&self, name: &str, shape: &Shape, service: &Service<'_>) -> Option<String> {
        if name != "RestoreRequest" && name.ends_with("Request") {
            if used_as_request_payload(shape) {
                return Some(generate_request_payload_serializer(name, shape));
            } else {
                return None;
            }
        }

        let ty = get_rust_type(service, name, shape, false, self.timestamp_type());
        Some(format!(
            "
                pub struct {name}Serializer;
                impl {name}Serializer {{
                    {serializer_signature} {{
                        {serializer_body}
                    }}
                }}
                ",
            name = name,
            serializer_body = generate_serializer_body(shape, service),
            serializer_signature = generate_serializer_signature(&ty),
        ))
    }

    fn generate_deserializer(
        &self,
        name: &str,
        shape: &Shape,
        service: &Service<'_>,
    ) -> Option<String> {
        let ty = get_rust_type(service, name, shape, false, self.timestamp_type());
        Some(xml_payload_parser::generate_deserializer(
            name, &ty, shape, service,
        ))
    }

    fn timestamp_type(&self) -> &'static str {
        "String"
    }
}

fn generate_documentation(operation: &Operation, service: &Service<'_>) -> String {
    let mut docs = match operation.documentation {
        Some(ref docs) => crate::doco::Item(docs).to_string(),
        None => "".to_owned(),
    };

    // Specialized docs for services:
    match service.name().to_ascii_lowercase().as_ref() {
        "route 53" => {
            if operation.name == "ChangeResourceRecordSets" {
                docs = format!("/// For TXT records, see <a href=\"./util/fn.quote_txt_record.html\">util::quote_txt_record</a>\n{}", docs);
            }
            ()
        }
        _ => (),
    }

    docs
}

fn generate_payload_serialization(service: &Service<'_>, operation: &Operation) -> Option<String> {
    // nothing to do if there's no input type
    if !service.has_non_empty_input_shape(operation) {
        return None;
    }

    let input = operation.input.as_ref().expect("no input");
    let input_shape = service.get_shape(&input.shape).expect("input shape not found");

    let mut parts: Vec<String> = Vec::new();

    // the payload field determines which member of the input shape is sent as the request body (if any)
    if input_shape.payload.is_some() {
        parts.push(generate_payload_member_serialization(input_shape));
        parts.push(
            generate_service_specific_code(service, operation).unwrap_or_else(|| "".to_owned()),
        );
    } else if used_as_request_payload(input_shape) {
        // In Route 53, no operation has "payload" parameter but some API actually requires
        // payload. In that case, the payload should include members whose "location" parameter is
        // missing.
        let xmlns = match input.xml_namespace.as_ref() {
            Some(ns) => ns,
            None => panic!("input {:?} does not have namespace", input)
        };
        parts.push("let mut writer = EventWriter::new(Vec::new());".to_owned());
        parts.push(format!(
            "{name}Serializer::serialize(&mut writer, \"{name}\", &self, \"{xmlns}\");",
            name = input.shape,
            xmlns = xmlns.uri
        ));
        parts.push("request.set_payload(Some(writer.into_inner()));".to_owned());
        parts.push(
            generate_service_specific_code(service, operation).unwrap_or_else(|| "".to_owned()),
        );
    }

    Some(parts.join("\n"))
}

fn generate_service_specific_code(service: &Service<'_>, operation: &Operation) -> Option<String> {
    // S3 needs some special handholding.  Others may later.
    // See `handlers.py` in botocore for more details
    match service.service_type_name() {
        "S3" => match &operation.name[..] {
            "PutBucketTagging"
            | "PutBucketLifecycle"
            | "PutBucketLifecycleConfiguration"
            | "PutBucketCors"
            | "DeleteObjects"
            | "PutBucketReplication" => Some("request.set_content_md5_header();".to_owned()),
            _ => None,
        },
        _ => None,
    }
}

fn generate_payload_member_serialization(shape: &Shape) -> String {
    let payload_field = shape.payload.as_ref().unwrap();
    let payload_member = shape.members.as_ref().unwrap().get(payload_field).unwrap();

    // if the member is 'streaming', it's a boxed stream that should just be delivered as the body
    if payload_member.streaming() {
        format!(
            "if let Some(__body) = self.{} {{
                     request.set_payload_stream(__body);
                 }}",
            payload_field.to_snake_case()
        )
    }
    // otherwise serialize the object to XML and use that as the payload
    else if shape.required(payload_field) {
        // some payload types are not required members of their shape
        format!("let mut writer = EventWriter::new(Vec::new());
                 {xml_type}Serializer::serialize(&mut writer, \"{xml_type}\", &self.{payload_field});
                 request.set_payload(Some(writer.into_inner()));",
                payload_field = payload_field.to_snake_case(),
                xml_type = payload_member.shape)
    } else {
        format!("if self.{payload_field}.is_some() {{
                    let mut writer = EventWriter::new(Vec::new());
                    {xml_type}Serializer::serialize(&mut writer, \"{location_name}\", self.{payload_field}.as_ref().unwrap());
                    request.set_payload(Some(writer.into_inner()));
                }} else {{
                    request.set_payload(Some(Vec::new()));
                }}",
                payload_field = payload_field.to_snake_case(),
                xml_type = payload_member.shape,
                location_name = payload_member.location_name.as_ref().unwrap())
    }
}

fn generate_method_signature(
    operation_name: &str,
    operation: &Operation,
    service: &Service<'_>,
) -> String {
    if service.has_non_empty_input_shape(operation) {
        format!(
            "fn {method_name}(&self, input: {input_type}) -> Request<{input_type}>",
            input_type = operation.input_shape(),
            method_name = operation_name.to_snake_case()
        )
    } else {
        format!(
            "fn {method_name}(&self) -> Request<{input_type}>",
            input_type = operation.input_shape(),
            method_name = operation_name.to_snake_case()
        )
    }
}

fn generate_serializer_body(shape: &Shape, service: &Service<'_>) -> String {
    match shape.shape_type {
        ShapeType::List => generate_list_serializer(shape, service),
        ShapeType::Map => generate_map_serializer(shape),
        ShapeType::Structure => generate_struct_serializer(shape, service),
        _ => generate_primitive_serializer(shape),
    }
}

fn generate_serializer_signature(name: &str) -> String {
    format!("
        #[allow(unused_variables, warnings)]
        pub fn serialize<W>(mut writer: &mut EventWriter<W>, name: &str, obj: &{}) -> Result<(), xml::writer::Error> where W: Write",
            name)
}

fn generate_primitive_serializer(shape: &Shape) -> String {
    let value_str = match shape.shape_type {
        ShapeType::Blob => "String::from_utf8(obj.to_vec()).expect(\"Not a UTF-8 string\")",
        _ => "obj.to_string()",
    };
    format!("
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(\"{{value}}\", value = {value_str})))?;
        writer.write(xml::writer::XmlEvent::end_element())
        ", value_str = value_str)
}

fn generate_list_serializer(shape: &Shape, service: &Service<'_>) -> String {
    // flattened lists don't have enclosing <FooList> tags
    // around the list elements
    let flattened = match shape.flattened {
        Some(true) => true,
        _ => false,
    };

    let member = shape.member.as_ref().expect("Member shape undefined");
    let element_type = &mutate_type_name(service, &member.shape);
    let mut serializer = "".to_owned();

    if flattened {
        serializer += &format!(
            "
            for element in obj {{
                {element_type}Serializer::serialize(writer, name, element)?;
            }}",
            element_type = element_type
        );
    } else {
        serializer += "writer.write(xml::writer::XmlEvent::start_element(name))?;";
        serializer += &format!(
            "
            for element in obj {{
                {element_type}Serializer::serialize(writer, \"{location_name}\", element)?;
            }}",
            element_type = element_type,
            location_name = member.location_name.as_ref().unwrap()
        );
        serializer += "writer.write(xml::writer::XmlEvent::end_element())?;";
    }

    serializer += "Ok(())";
    serializer
}

fn generate_map_serializer(_shape: &Shape) -> String {
    // No rest-xml services use Map shape type currently.
    unreachable!()
}

fn generate_struct_serializer(shape: &Shape, service: &Service<'_>) -> String {
    let mut serializer = "writer.write(xml::writer::XmlEvent::start_element(name))?;".to_owned();

    for (member_name, member) in shape.members.as_ref().unwrap().iter() {
        // look up member.shape in all_shapes.  use that shape.member.location_name
        let location_name = member.location_name.as_ref().unwrap_or(member_name);

        if member.deprecated() {
            continue;
        }

        let member_shape = service.shape_for_member(member).unwrap();

        match member_shape.shape_type {
            ShapeType::List | ShapeType::Map | ShapeType::Structure => {
                serializer += &generate_complex_struct_field_serializer(
                    shape,
                    member,
                    location_name,
                    member_name,
                );
            }
            _ => {
                serializer +=
                    &generate_primitive_struct_field_serializer(shape, location_name, member_name);
            }
        }
    }

    serializer += "writer.write(xml::writer::XmlEvent::end_element())";
    serializer
}

fn generate_primitive_struct_field_serializer(
    shape: &Shape,
    location_name: &str,
    member_name: &str,
) -> String {
    if shape.required(member_name) {
        format!(
        "writer.write(xml::writer::XmlEvent::start_element(\"{location_name}\"))?;
        writer.write(xml::writer::XmlEvent::characters(&format!(\"{{value}}\", value=obj.{field_name})))?;
        writer.write(xml::writer::XmlEvent::end_element())?;",
        field_name = generate_field_name(member_name),
        location_name = location_name,
    )
    } else {
        format!(
            "if let Some(ref value) = obj.{field_name} {{
                writer.write(xml::writer::XmlEvent::start_element(\"{location_name}\"))?;
                writer.write(xml::writer::XmlEvent::characters(&format!(\"{{value}}\", value=value)));
                writer.write(xml::writer::XmlEvent::end_element())?;
            }}",
            field_name = generate_field_name(member_name),
            location_name = location_name,
        )
    }
}

fn generate_complex_struct_field_serializer(
    shape: &Shape,
    member: &Member,
    location_name: &str,
    member_name: &str,
) -> String {
    if shape.required(member_name) {
        format!("{xml_type}Serializer::serialize(&mut writer, \"{location_name}\", &obj.{field_name})?;",
                xml_type = member.shape,
                location_name = location_name,
                field_name = generate_field_name(member_name))
    } else {
        format!(
            "
            if let Some(ref value) = obj.{field_name} {{
                &{xml_type}Serializer::serialize(&mut writer, \"{location_name}\", value)?;
            }}",
            xml_type = member.shape,
            location_name = location_name,
            field_name = generate_field_name(member_name)
        )
    }
}

fn used_as_request_payload(shape: &Shape) -> bool {
    if shape.payload.is_some() {
        return false;
    }
    if let Some(ref members) = shape.members {
        for member in members.values() {
            if member.location.is_none() {
                return true;
            }
        }
    }
    false
}

fn generate_request_payload_serializer(name: &str, shape: &Shape) -> String {
    let mut parts = Vec::new();
    parts.push(format!("pub struct {name}Serializer;", name = name));
    parts.push(format!("impl {name}Serializer {{", name = name));
    parts.push("#[allow(unused_variables, warnings)]".to_owned());
    parts.push(format!("pub fn serialize<W>(mut writer: &mut EventWriter<W>, name: &str, obj: &{name}, xmlns: &str) -> Result<(), xml::writer::Error> where W: Write {{", name = name));
    parts.push(
        "writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;".to_owned(),
    );
    for (member_name, member) in shape.members.as_ref().unwrap() {
        if member.location.is_none() {
            let location_name = member.location_name.as_ref().unwrap_or(member_name);
            parts.push(generate_complex_struct_field_serializer(
                shape,
                member,
                location_name,
                member_name,
            ));
        }
    }
    parts.push("writer.write(xml::writer::XmlEvent::end_element())".to_owned());
    parts.push("}}".to_owned());
    parts.join("")
}
