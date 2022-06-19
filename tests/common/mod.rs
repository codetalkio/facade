// use async_graphql::{
//     DeserializerError, EmptyMutation, EmptySubscription, Object, Request, Schema, SimpleObject,
//     Variables,
// };
use async_graphql::*;
use graphql_client::GraphQLQuery;
use graphql_client::QueryBody;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid as UuidV4;

/// Initialize the GraphQL handler.
#[allow(dead_code)]
pub static GRAPHQL: Lazy<Schema<Query, EmptyMutation, EmptySubscription>> =
    Lazy::new(|| Schema::new(Query, EmptyMutation, EmptySubscription));

/// Execute a GraphQL statement from a [graphql_client::QueryBody] and deserialize
/// the response into its expected data type.
#[allow(dead_code)]
pub async fn execute<Q, D>(query: QueryBody<Q>) -> Result<D, DeserializerError>
where
    Q: Serialize,
    D: DeserializeOwned,
{
    let req: Request = Request::new(query.query)
        .operation_name(query.operation_name)
        .variables(Variables::from_json(
            // TODO: Align error types so we can use `?` for this part as well.
            serde_json::to_value(query.variables).unwrap(),
        ));
    let res = GRAPHQL.execute(req).await;
    async_graphql::from_value::<D>(res.data)
}

/// The GraphQL schema for our mock service.
///
/// ```graphql
#[doc = include_str!("schema.graphql")]
/// ```
pub struct Query;

#[Object(rename_fields = "camelCase")]
impl Query {
    /// A simple resolver that returns a UUID V4.
    async fn generate_uuid(&self) -> Uuid {
        Uuid {
            uuid: UuidV4::new_v4().to_string(),
        }
    }

    /// The current users' details.
    async fn me(&self) -> Me {
        Me {
            username: "Test".to_string(),
            email: "test@example.com".to_string(),
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
struct Uuid {
    pub uuid: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
struct Me {
    pub username: String,
    pub email: String,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/common/schema.graphql",
    query_path = "tests/common/queries.graphql",
    response_derives = "Debug",
    variables_derives = "Debug"
)]
pub struct MeQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/common/schema.graphql",
    query_path = "tests/common/queries.graphql",
    response_derives = "Debug",
    variables_derives = "Debug"
)]
pub struct GenerateUuidQuery;
