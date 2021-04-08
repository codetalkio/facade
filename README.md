# Facade
> When you need to pretend you are a REST API, but you're really not

Facade is a Rust-based HTTP transformation layer to seamlessly convert REST calls into GraphQL calls for piecemeal API migrations.

**Goals:**

- [ ] Seamlessly proxy any HTTP requests through, keeping headers etc intact
- [ ] Support overriding specific URLs for specific HTTP verbs
- [ ] Support easy transformation into a GraphQL request
- [ ] Support easy transformation of GraphQL response data into expected REST output
- [ ] Be easily testable (e.g. make sure it's easy to check a route got directed to the right match)


## Implementation Goal

First off, we'll need to implement some test servers for REST and for GraphQL.

After this, we'll need a couple of cases:
- Passthrough paths that are not overridden
- A direct transformation from REST<->GraphQL
- An advanced transformation from REST<->GraphQL which processes the data


We can set up a couple of REST endpoints:
- `GET /api/v1/uuid` returns a UUID
- `GET /api/v1/me` returns a JSON object `{ data: { username: "Ariel", ... } }`

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

TODO: Make the above setup a bit more advanced.

## Resources

- [hyper](https://docs.rs/crate/hyper) for managing HTTP requests
- [routerify](https://github.com/routerify/routerify) to manage routes in hyper
- [anyhow](https://docs.rs/anyhow/) for nice error handling
- [log](https://docs.rs/log/) and [fern](https://docs.rs/fern) for logging
- [chrono](https://docs.rs/chrono/) for time handling
- [hyper-rustls](https://docs.rs/hyper-rustls) for handling HTTPS in hyper
- [httpbin: A simple HTTP Request & Response Service.](http://httpbin.org) is very useful for testing
- [cargo-watch](https://crates.io/crates/cargo-watch) for developing with `cargo dev` (install via `cargo install cargo-watch`)

Potentially:
- [warp](https://github.com/seanmonstar/warp) as a higher-level alternative to hyper
- [warp-reverse-proxy](https://github.com/danielSanchezQ/warp-reverse-proxy) is a warp filter for easy reverse proxying
- [reqwest](https://github.com/seanmonstar/reqwest) is a high-level HTTP client


Some blog posts:
- https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
- https://blog.joco.dev/posts/warp_auth_server_tutorial
- [Live coding an HTTP reverse proxy in Rust](https://www.youtube.com/watch?v=FcHYQMRfGWw) (and [gist](https://gist.github.com/snoyberg/35a661fff527692d09675ef540c7c1eb) of the code)
- [benchmark of different web frameworks](https://github.com/routerify/routerify-benchmark)
