#![cfg(feature = "fsx")]

extern crate rusoto_core;
extern crate rusoto_fsx;

use rusoto_core::Region;
use rusoto_fsx::{DescribeFileSystemsRequest, Fsx, FsxClient};

#[test]
fn should_describe_filesystems() {
    let client = FsxClient::new(Region::UsEast1);
    let request = DescribeFileSystemsRequest::default();

    let res = client.describe_file_systems(request).sync().unwrap();
    println!("res is {:?}", res);
}
