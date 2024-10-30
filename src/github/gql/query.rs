use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/gql/schema.json",
    query_path = "src/github/gql/query.graphql",
    variables_derives = "Debug",
    response_derives = "Debug"
)]
pub struct Authenticate;
