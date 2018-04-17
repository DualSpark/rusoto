#![cfg(feature = "autoscaling")]

extern crate rusoto_core;
extern crate rusoto_autoscaling;

use rusoto_autoscaling::{Autoscaling, AutoscalingClient, AutoScalingGroupNamesType};
use rusoto_core::Region;

#[test]
fn should_describe_auto_scaling_groups() {
    let client = AutoscalingClient::simple(Region::UsEast1);
    let request = AutoScalingGroupNamesType::default();

    let response = client.describe_auto_scaling_groups(request).sync().unwrap();
    println!("{:#?}", response);
}
