#![cfg(feature = "inspector")]

extern crate rusoto_core;
extern crate rusoto_inspector;

use rusoto_inspector::{Inspector, InspectorClient, ListAssessmentRunsRequest};
use rusoto_core::Region;

#[test]
fn should_list_assessment_runs() {
    let client = InspectorClient::simple(Region::UsEast1);
    let request = ListAssessmentRunsRequest::default();

    client.list_assessment_runs(request).sync().unwrap();
}
