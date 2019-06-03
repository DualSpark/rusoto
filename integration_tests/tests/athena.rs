#![cfg(feature = "athena")]

extern crate rusoto_athena;
extern crate rusoto_core;

use rusoto_athena::{Athena, AthenaClient, ListNamedQueriesRequest};
use rusoto_core::Region;

#[test]
fn should_list_named_queries() {
    let client = AthenaClient::new(Region::UsEast1);
    let request = ListNamedQueriesRequest::default();

    client.list_named_queries(request).sync().unwrap();
}
