# Facade
> When you need to keep pretending you are a REST API, but you're really not

Facade is a Rust-based HTTP transformation layer to seamlessly convert REST calls into GraphQL calls for piecemeal API migrations.

**Goals:**

- [ ] Seamlessly proxy any HTTP requests through, keeping headers etc intact
- [ ] Support overriding specific URLs for specific HTTP verbs
- [ ] Support easy transformation into a GraphQL request
- [ ] Support easy transformation of GraphQL response data into expected REST output
- [ ] Be easily testable


## Resources

- [warp](https://github.com/seanmonstar/warp) is used as the HTTP server ([docs](https://docs.rs/warp/0.3.0/warp/))
- [warp-reverse-proxy](https://github.com/danielSanchezQ/warp-reverse-proxy) for proxying requests
- [reqwest](https://github.com/seanmonstar/reqwest) is used as the HTTP client


Some blog posts:
- https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
- https://blog.joco.dev/posts/warp_auth_server_tutorial
