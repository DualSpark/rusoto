#![cfg(feature = "codedeploy")]

extern crate rusoto;

use rusoto::codedeploy::{CodeDeployClient, ListApplicationsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_applications() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeDeployClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListApplicationsInput::default();

    client.list_applications(&request).unwrap();
}