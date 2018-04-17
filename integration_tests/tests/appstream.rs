#![cfg(feature = "appstream")]

extern crate rusoto_core;
extern crate rusoto_appstream;
extern crate env_logger;

use rusoto_appstream::{AppStream, AppStreamClient, DescribeFleetsRequest};
use rusoto_core::Region;

#[test]
fn should_describe_fleets() {
    let _ = env_logger::try_init();
    let client = AppStreamClient::simple(Region::UsEast1);
    let request = DescribeFleetsRequest::default();

	let result = client.describe_fleets(request).sync().unwrap();
	println!("{:#?}", result);
}