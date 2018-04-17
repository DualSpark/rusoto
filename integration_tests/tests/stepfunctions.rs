#![cfg(feature = "stepfunctions")]

extern crate rusoto_core;
extern crate rusoto_stepfunctions;

use rusoto_stepfunctions::{StepFunctions, StepFunctionsClient, ListStateMachinesInput};
use rusoto_core::Region;

#[test]
fn should_list_state_machines() {
    let client = StepFunctionsClient::simple(Region::UsEast1);
    let request = ListStateMachinesInput::default();

    let result = client.list_state_machines(request).sync().unwrap();
    println!("{:#?}", result);
}
