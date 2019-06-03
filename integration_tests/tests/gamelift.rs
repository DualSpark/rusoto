#![cfg(feature = "gamelift")]

extern crate rusoto_core;
extern crate rusoto_gamelift;

use rusoto_core::Region;
use rusoto_gamelift::{GameLift, GameLiftClient, ListFleetsRequest};

#[test]
fn should_list_fleets() {
    let client = GameLiftClient::new(Region::UsWest2);
    let request = ListFleetsRequest::default();

    let result = client.list_fleets(request).sync().unwrap();
    println!("{:#?}", result);
}
