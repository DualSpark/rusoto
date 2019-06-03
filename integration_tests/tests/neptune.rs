#![cfg(feature = "neptune")]

extern crate rusoto_core;
extern crate rusoto_neptune;

use rusoto_core::Region;
use rusoto_neptune::{DescribeDBClustersRequest, Neptune, NeptuneClient};

#[test]
fn should_describe_db_clusters() {
    let client = NeptuneClient::new(Region::UsEast1);
    let request = DescribeDBClustersRequest::default();

    match client.describe_db_clusters(request).sync() {
        Err(e) => panic!("Error listing Neptune clusters: {}", e),
        Ok(clusters) => println!("Found clusters: {:?}", clusters),
    }
}
