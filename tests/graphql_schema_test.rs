use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use graphql_client::GraphQLQuery;
use std::fs;
use std::str::FromStr;
use uuid::Uuid;

mod common;

#[ignore]
#[tokio::test]
async fn generate_graphql_sdl() {
    let schema = Schema::build(common::Query, EmptyMutation, EmptySubscription).finish();
    fs::write("tests/common/schema.graphql", &schema.sdl()).unwrap();
}

#[tokio::test]
async fn compare_generated_and_saved_graphql_sdl() {
    let generated_schema = Schema::build(common::Query, EmptyMutation, EmptySubscription).finish();
    let saved_schema = fs::read_to_string("tests/common/schema.graphql").unwrap();
    assert_eq!(&generated_schema.sdl(), &saved_schema);
}

#[tokio::test]
async fn generates_a_valid_uuid() {
    let query = common::GenerateUuidQuery::build_query(common::generate_uuid_query::Variables);
    let json: common::generate_uuid_query::ResponseData = common::execute(query).await.unwrap();

    // Validate that the response is a valid UUID V4.
    assert!(Uuid::from_str(&json.generate_uuid.uuid).is_ok());
}

#[tokio::test]
async fn returns_an_object_for_me() {
    let query = common::MeQuery::build_query(common::me_query::Variables);
    let json: common::me_query::ResponseData = common::execute(query).await.unwrap();

    // Validate the data returned by the resolver.
    assert_eq!(&json.me.username, "Test");
    assert_eq!(&json.me.email, "test@example.com");
}
