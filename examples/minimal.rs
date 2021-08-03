use facade::*;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Set up logging
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply();

    // Set up where Facade should proxy and which paths it should overwrite.
    // let server = Facade::builder()
    //     .bind(std::net::SocketAddr::from(([127, 0, 0, 1], 3000)))
    //     .get(
    //         "/api/v1/uuid",
    //         Facade::direct_graphql_wrapped(GraphQL::Query::uuid, "data"),
    //     )
    //     .get("/api/v2/uuid", Facade::direct_graphql(GraphQL::Query::uuid))
    //     .get("/api/v1/me", me_handler)
    //     .get("/api/v1/*", "https://httpbin.org")
    //     .build()
    //     .unwrap()
    //     .serve();

    // Start the server.
    // server.await

    Ok(())
}
