#![cfg(feature = "cloudwatch")]

extern crate rusoto_cloudwatch;
extern crate rusoto_core;

use rusoto_cloudwatch::{CloudWatch, CloudWatchClient, Dimension, MetricDatum, PutMetricDataRequest};
use rusoto_core::Region;

#[test]
fn should_put_metric_data() {
    let client = CloudWatchClient::new(Region::UsEast1);

    let metric_data = vec![MetricDatum {
        dimensions: Some(vec![Dimension {
            name: "foo".to_string(),
            value: "bar".to_string(),
        }]),
        metric_name: "buffers".to_string(),
        statistic_values: None,
        timestamp: None,
        unit: Some("Bytes".to_string()),
        value: Some(1.0),
        ..Default::default()
    }];
    let request = PutMetricDataRequest {
        namespace: "TestNamespace".to_string(),
        metric_data: metric_data,
    };

    let response = client.put_metric_data(request).sync().unwrap();
    println!("{:#?}", response);
}
