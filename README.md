# Facade
> When you need to pretend you are a REST API, but you're really not

Facade is a Rust-based HTTP transformation layer to seamlessly convert REST calls into GraphQL calls for piecemeal API migrations.

**Goals:**

- [ ] Seamlessly proxy any HTTP requests through, keeping headers etc intact
- [ ] Support overriding specific URLs for specific HTTP verbs
- [ ] Support easy transformation into a GraphQL request
- [ ] Support easy transformation of GraphQL response data into expected REST output
- [ ] Be easily testable (e.g. make sure it's easy to check a route got directed to the right match)
  - [ ] Support for testing e.g. `/api/v1/me` was mapped, and `/api/v1/device` was passed through
  - [ ] Add header `X-FACADE-MATCH` that exposes what happened to the path (e.g.  `MAPPED` or `PASSTHROUGH`) - you should be able to turn this off also

**Non Goals:**
- [ ] Extending existing GraphQL schema
- [ ] Focusing on anything else than GraphQL (it should be trivial, but GraphQL should be the easy-path)

## Motivation

 GraphQL is only a recent addition into the API space, with REST having been the predominant way to structure APIs so far. So now you want to get on the sweet journey towards a nicely structured GraphQL API—but wait!—you still need to keep your REST API around because you have old clients that either cannot or will not be updated (e.g. old Mobile App releases).

How do you initiate your API modernization without now needing to maintain two different APIs? This is where _facade_ comes into play.

_facade_ is a service that sits in front of you existing REST API to allow you a piecemeal migration to your new API, by rewriting requests on-the-fly. After mapping an incoming REST call to the new GraphQL query, you can now remove the old code in your REST API.

This project was inspired by a real-world situtation where the backend provided a legacy REST API along with a new GraphQL API. These two APIs were written in different languages and served by different services. Completely removing the REST API was far away from possilbe, because of old clients existing, specifically previous Mobile App releases, that simply couldn't be updated.

On the frontend-side of things, one could easily migrate each new release to the GraphQL API as it became fully fleshed out, but the backend was stuck supporting the REST API in perpetuity. We needed a way to still support the same legacy API calls, but without needing to maintain the legacy code as well—and thus, _facade_ was born.

## Implementation Stategy

First off, we'll need to implement:
- [ ] Test servers for REST
- [ ] Test servers for GraphQL

After this, we'll need a couple of cases:
- [ ] Passthrough: Paths that are not overridden
- [ ] Simple case: A direct transformation from REST<->GraphQL (fields map directly)
- [ ] Advanced case: A transformation from REST<->GraphQL which processes the data (fields need transformation)


We can set up a couple of REST endpoints:
- Passthrough: `GET /api/v1/device` returns a JSON object `{ data: { devices: [] } }`
- Simple case: `GET /api/v1/uuid` returns a JSON object `{ data : { uuid: "UUIDV4...." } }`
- Simple case: `GET /api/v2/uuid` returns a JSON object `{ uuid: "UUIDV4...." }`
- Advanced case: `GET /api/v1/me` returns a JSON object `{ data: { username: "Ariel", ... } }`

And a GraphQL schema:
```graphql
type User {
  username: String!
}

Query {
  me: User!
  uuid: String!
}
```

The REST endpoint should easily support if things are wrapped in something or not (e.g. `data`).

### Ideal Library Design
Let's start from how the user would interact with the library:

```rust
use facade::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply();

    // Set up where Facade should proxy and which paths it should overwrite.
    let server = Facade::builder()
      .bind(std::net::SocketAddr::from(([127, 0, 0, 1], 3000)))
      .get("/api/v1/uuid", Facade::direct_graphql_wrapped(GraphQL::Query::uuid, "data"))
      .get("/api/v2/uuid", Facade::direct_graphql(GraphQL::Query::uuid))
      .get("/api/v1/me", me_handler)
      .get("/api/v1/*", "https://httpbin.org")
      .build()
      .unwrap()
      .serve();

    // Start the server.
    server.await
}
```

## Resources

- [hyper](https://docs.rs/crate/hyper) for managing HTTP requests
- [routerify](https://github.com/routerify/routerify) to manage routes in hyper
- [anyhow](https://docs.rs/anyhow/) for nice error handling
- [log](https://docs.rs/log/) and [fern](https://docs.rs/fern) for logging
- [chrono](https://docs.rs/chrono/) for time handling
- [hyper-rustls](https://docs.rs/hyper-rustls) for handling HTTPS in hyper
- [async-graphql](https://github.com/async-graphql/async-graphql) for our GraphQL client (and server for testing)
- [httpbin: A simple HTTP Request & Response Service.](http://httpbin.org) is very useful for testing
- [cargo-watch](https://crates.io/crates/cargo-watch) for developing with `cargo dev` or `cargo watch -x 'test -- --nocapture'` (install via `cargo install cargo-watch`)

Potentially:
- [warp](https://github.com/seanmonstar/warp) as a higher-level alternative to hyper
- [warp-reverse-proxy](https://github.com/danielSanchezQ/warp-reverse-proxy) is a warp filter for easy reverse proxying
- [reqwest](https://github.com/seanmonstar/reqwest) is a high-level HTTP client


Some blog posts:
- https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
- https://blog.joco.dev/posts/warp_auth_server_tutorial
- [Live coding an HTTP reverse proxy in Rust](https://www.youtube.com/watch?v=FcHYQMRfGWw) (and [gist](https://gist.github.com/snoyberg/35a661fff527692d09675ef540c7c1eb) of the code)
- [benchmark of different web frameworks](https://github.com/routerify/routerify-benchmark)
- [Use an Action to embed the code example and ensure it always compiles?](https://github.com/marketplace/actions/markdown-embed-code-from-file)
