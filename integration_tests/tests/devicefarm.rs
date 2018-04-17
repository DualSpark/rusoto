#![cfg(feature = "devicefarm")]

extern crate rusoto_core;
extern crate rusoto_devicefarm;

use rusoto_devicefarm::{DeviceFarm, DeviceFarmClient, ListDevicesRequest};
use rusoto_core::Region;

#[test]
pub fn should_list_devices() {
    
    let client = DeviceFarmClient::simple(Region::UsWest2);
    let request = ListDevicesRequest::default();

    client.list_devices(request).sync().unwrap();
}
