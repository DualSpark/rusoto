//! AWS Application Load Balancing
//!
//! If you're using the service, you're probably looking for [ElbClient](struct.ElbClient.html).

use region;

include!(concat!(env!("OUT_DIR"), "/elbv2.rs"));
