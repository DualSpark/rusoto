//! AWS CloudWatch
//!
//! If you're using the service, you're probably looking for [CloudWatchClient](struct.CloudWatchClient.html).

include!(concat!(env!("OUT_DIR"), "/cloudwatch.rs"));

#[cfg(test)]
mod test {
    use cloudwatch::{CloudWatchClient, PutMetricDataInput, Dimension, MetricDatum};

    use super::super::{Region, SignedRequest};
    use super::super::mock::*;

    extern crate env_logger;

    #[test]
    fn should_serialize_complex_metric_data_params() {
        let mock = MockRequestDispatcher::with_status(200)
            .with_body("")
            .with_request_checker(|request: &SignedRequest| {
                assert_eq!("POST", request.method);
                assert_eq!("/", request.path);
                assert_eq!(None, request.payload);
                assert_eq!(request.params.get("Namespace"),
                           Some(&Some("TestNamespace".to_owned())));
                assert_eq!(request.params.get("MetricData.member.1.MetricName"),
                           Some(&Some("buffers".to_owned())));
                assert_eq!(request.params.get("MetricData.member.1.Unit"),
                           Some(&Some("Bytes".to_owned())));
                assert_eq!(request.params.get("MetricData.member.1.Value"),
                           Some(&Some("1".to_owned())));
                assert_eq!(request.params.get("MetricData.member.1.Dimensions.member.1.Name"),
                           Some(&Some("foo".to_owned())));
                assert_eq!(request.params.get("MetricData.member.1.Dimensions.member.1.Value"),
                           Some(&Some("bar".to_owned())));

            });
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
                               }];
        let request = PutMetricDataInput {
            namespace: "TestNamespace".to_string(),
            metric_data: metric_data,
        };

        let client = CloudWatchClient::new(mock, MockCredentialsProvider, Region::UsEast1);
        let response = client.put_metric_data(&request).unwrap();
        println!("{:#?}", response);
    }
}
