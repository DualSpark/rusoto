//! Amazon DynamoDB
//!
//! If you're using the service, you're probably looking for [DynamoDbClient](struct.DynamoDbClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/dynamodb.rs"));
