#![cfg(feature = "cloudtrail")]

extern crate rusoto_core;
extern crate rusoto_cloudtrail;

use rusoto_cloudtrail::{CloudTrail, CloudTrailClient, DescribeTrailsRequest};
use rusoto_core::Region;

#[test]
fn should_describe_trails() {
    let client = CloudTrailClient::simple(Region::UsEast1);
    let request = DescribeTrailsRequest::default();

    client.describe_trails(request).sync().unwrap();
}
