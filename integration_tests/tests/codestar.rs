#![cfg(feature = "codestar")]

extern crate rusoto_codestar;
extern crate rusoto_core;

use rusoto_codestar::{CodeStar, CodeStarClient, ListProjectsRequest};
use rusoto_core::Region;

#[test]
fn should_list_projects() {
    let client = CodeStarClient::new(Region::UsEast1);
    let request = ListProjectsRequest::default();

    let result = client.list_projects(request).sync().unwrap();
    println!("Result is {:?}", result);
}
