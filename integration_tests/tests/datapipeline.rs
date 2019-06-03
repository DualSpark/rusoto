#![cfg(feature = "datapipeline")]

extern crate rusoto_core;
extern crate rusoto_datapipeline;

use rusoto_core::Region;
use rusoto_datapipeline::{DataPipeline, DataPipelineClient, ListPipelinesRequest};

#[test]
fn should_list_pipelines() {
    let client = DataPipelineClient::new(Region::UsEast1);
    let request = ListPipelinesRequest::default();

    client.list_pipelines(request).sync().unwrap();
}
