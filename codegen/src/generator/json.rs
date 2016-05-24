use inflector::Inflector;

use std::collections::HashMap;

use botocore::{Operation, Service};
use super::GenerateProtocol;

pub struct JsonGenerator;

impl GenerateProtocol for JsonGenerator {
    fn generate_methods(&self, service: &Service) -> String {
        service.operations.values().map(|operation| {

            let output_type = operation.output_shape_or("()");

            format!("
                {documentation}
                pub fn {method_name}(&mut self, input: &{input_type}) -> {result_type} {{
                    let encoded = serde_json::to_string(input).unwrap();
                    let mut request = SignedRequest::new(\"{http_method}\", \"{endpoint_prefix}\", self.region, \"{request_uri}\");
                    request.set_content_type(\"application/x-amz-json-{json_version}\".to_owned());
                    request.add_header(\"x-amz-target\", \"{target_prefix}.{name}\");
                    request.set_payload(Some(encoded.as_bytes()));
                    let mut result = request.sign_and_execute(try!(self.credentials_provider.credentials()), &self.client);
                    let status = result.status.to_u16();
                    let mut body = String::new();
                    result.read_to_string(&mut body).unwrap();
                    match status {{
                        200 => {{
                            {ok_response}
                        }}
                        _ => {err_response},
                    }}
                }}
                ",
                documentation = generate_documentation(operation).unwrap_or("".to_owned()),
                endpoint_prefix = service.metadata.endpoint_prefix,
                http_method = operation.http.method,
                input_type = operation.input_shape(),
                method_name = operation.name.to_snake_case(),
                name = operation.name,
                ok_response = generate_ok_response(operation, output_type),
                err_response = generate_err_response(service, operation),
                result_type = generate_result_type(service, operation, output_type),
                request_uri = operation.http.request_uri,
                target_prefix = service.metadata.target_prefix.as_ref().unwrap(),
                json_version = service.metadata.json_version.as_ref().unwrap(),
            )
        }).collect::<Vec<String>>().join("\n")
    }

    fn generate_prelude(&self, service: &Service) -> String {
        format!(
            "use std::io::Read;

            use serde_json;

            use credential::ProvideAwsCredentials;
            use region::Region;
            use signature::SignedRequest;

            {error_imports}",
            error_imports = generate_error_imports(service))
    }

    fn generate_struct_attributes(&self) -> String {
        "#[derive(Debug, Default, Deserialize, Serialize)]".to_owned()
    }

    fn generate_error_types(&self, service: &Service) -> Option<String>{
        if service.typed_errors() {

            // grab error type documentation for use with error enums in generated code
            // botocore presents errors as structs.  we filter those out in generate_types.
            let mut error_documentation = HashMap::new();

            for (name, shape) in service.shapes.iter() {
                if shape.exception() && shape.documentation.is_some() {
                    error_documentation.insert(name, shape.documentation.as_ref().unwrap());
                }
            }

            Some(service.operations.iter()
                .filter_map(|(_, operation)| generate_error_type(operation, &error_documentation) )
                .collect::<Vec<String>>()
                .join("\n")
                )
        } else {
           None
        }
   }
}


pub fn generate_error_type(operation: &Operation, error_documentation: &HashMap<&String, &String>,) -> Option<String> {

    let error_type_name = operation.error_type_name();

    Some(format!("
        #[derive(Debug, PartialEq)]
        pub enum {type_name} {{
            {error_types}
        }}

        impl {type_name} {{
            pub fn from_body(body: &str) -> {type_name} {{
                match from_str::<Value>(body) {{
                    Ok(json) => {{
                        let error_type: &str = match json.find(\"__type\") {{
                            Some(error_type) => error_type.as_string().unwrap_or(\"Unknown\"),
                            None => \"Unknown\",
                        }};

                        match error_type {{
                            {type_matchers}
                        }}
                    }},
                    Err(_) => {type_name}::Unknown(String::from(body))
                }}
            }}
        }}
        impl From<AwsError> for {type_name} {{
            fn from(err: AwsError) -> {type_name} {{
                {type_name}::Unknown(err.message)
            }}
        }}
        impl fmt::Display for {type_name} {{
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                write!(f, \"{{}}\", self.description())
            }}
        }}
        impl Error for {type_name} {{
            fn description(&self) -> &str {{
               match *self {{
                   {description_matchers}
               }}
           }}
       }}
       ",
       type_name = error_type_name,
       error_types = generate_error_enum_types(operation, error_documentation).unwrap_or(String::from("")),
       type_matchers = generate_error_type_matchers(operation).unwrap_or(String::from("")),
       description_matchers = generate_error_description_matchers(operation).unwrap_or(String::from(""))))
    }

fn generate_error_enum_types(operation: &Operation, error_documentation: &HashMap<&String, &String>) -> Option<String> {
    let mut enum_types: Vec<String> = Vec::new();

    if operation.errors.is_some() {
        for error in operation.errors.as_ref().unwrap().iter() {
            enum_types.push(format!("\n///{}\n{}(String)",
                error_documentation.get(&error.shape).unwrap_or(&&String::from("")),
                error.idiomatic_error_name()));
        }
    }

    enum_types.push("/// A validation error occurred.  Details from AWS are provided.\nValidation(String)".to_string());
    enum_types.push("/// An unknown error occurred.  The raw HTTP response is provided.\nUnknown(String)".to_string());
    Some(enum_types.join(","))
}

fn generate_error_type_matchers(operation: &Operation) -> Option<String> {
    let mut type_matchers: Vec<String> = Vec::new();
    let error_type = operation.error_type_name();

    if operation.errors.is_some() {
        for error in operation.errors.as_ref().unwrap().iter() {
            type_matchers.push(format!("\"{error_shape}\" => {error_type}::{error_name}(String::from(body))",
                error_shape = error.shape,
                error_type = error_type,
                error_name = error.idiomatic_error_name()))
        }
    }

   type_matchers.push(format!("\"Validation\" => {error_type}::Validation(String::from(body))", error_type = error_type));
   type_matchers.push(format!("_ => {error_type}::Unknown(String::from(body))",  error_type = error_type));
   Some(type_matchers.join(","))
}

fn generate_error_description_matchers(operation: &Operation) -> Option<String> {
    let mut type_matchers: Vec<String> = Vec::new();
    let error_type = operation.error_type_name();

    if operation.errors.is_some() {
        for error in operation.errors.as_ref().unwrap().iter() {
            type_matchers.push(format!("{error_type}::{error_shape}(ref cause) => cause",
                error_type = operation.error_type_name(),
                error_shape = error.idiomatic_error_name()))
        }
    }

   type_matchers.push(format!("{error_type}::Validation(ref cause) => cause", error_type = error_type));
   type_matchers.push(format!("{error_type}::Unknown(ref cause) => cause", error_type = error_type));
   Some(type_matchers.join(","))
}
fn generate_result_type<'a>(service: &Service, operation: &Operation, output_type: &'a str) -> String {
    if service.typed_errors() {
        format!("Result<{}, {}>", output_type, operation.error_type_name())
    } else {
        format!("AwsResult<{}>", output_type)
    }
}

fn generate_error_imports(service: &Service) -> &'static str {
    if service.typed_errors() {
        "use error::AwsError;
        use std::error::Error;
        use std::fmt;
        use serde_json::{Value, from_str};"
    } else {
        "use error::{AwsResult, parse_json_protocol_error};"
    }
}

fn generate_documentation(operation: &Operation) -> Option<String> {
    operation.documentation.as_ref().map(|docs| {
        format!("#[doc=\"{}\"]", docs.replace("\"", "\\\""))
    })
}

fn generate_ok_response(operation: &Operation, output_type: &str) -> String {
    if operation.output.is_some() {
        format!("Ok(serde_json::from_str::<{}>(&body).unwrap())", output_type)
    } else {
        "Ok(())".to_owned()
    }
}

fn generate_err_response(service: &Service, operation: &Operation) -> String {
    if service.typed_errors() {
        format!("Err({}::from_body(&body))", operation.error_type_name())
    } else {
        String::from("Err(parse_json_protocol_error(&body))") 
    }
}
