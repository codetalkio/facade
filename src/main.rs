use anyhow::*;
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Body, Client, Request, Response, Server, StatusCode,
};
use log::{debug, info};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router, RouterService};
use std::net::SocketAddr;
use std::sync::Arc;

const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
const PROXY_URL: &str = "https://httpbin.org";

struct Env {
    client: Arc<Client<hyper_rustls::HttpsConnector<HttpConnector<GaiResolver>>, hyper::Body>>,
    state: State,
}
struct State(u64);

async fn user_handler_2(req: Request<Body>) -> Result<Response<Body>> {
    // Access the app state.
    let env = req.data::<Env>().unwrap();
    debug!("State value: {}", env.state.0);

    Ok(Response::new(Body::from("User 2 page!")))
}

// A handler for "/" page.
async fn home_handler(_req: Request<Body>) -> Result<Response<Body>> {
    debug!("State value: ?");

    Ok(Response::new(Body::from("Home page")))
}

// A handler for "/users/:userId" page.
async fn user_handler(req: Request<Body>) -> Result<Response<Body>> {
    let user_id = req.param("userId").unwrap();
    Ok(Response::new(Body::from(format!("Hello {}", user_id))))
}

/// Set up the logging service.
///
/// This is where you will configure the output level, log format, etc.
fn setup_logging_service() -> Result<()> {
    // Add a `.chain(fern::log_file("output.log")?)` in the builder below
    // to log the output to a file as well.
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(LOG_LEVEL)
        .chain(std::io::stdout())
        .apply()
        .context("Setting up logging service")?;
    Ok(())
}

/// A middleware which logs incoming HTTP requests.
async fn logger(req: Request<Body>) -> Result<Request<Body>> {
    debug!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

// Define an error handler function which will accept the `routerify::Error`
// and the request information and generates an appropriate response.
async fn error_handler(err: routerify::Error, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
fn router() -> Router<Body, anyhow::Error> {
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<hyper_rustls::HttpsConnector<HttpConnector<GaiResolver>>, hyper::Body> =
        Client::builder().build(https);
    let client = Arc::new(client);

    // Create a router and specify the logger middleware and the handlers.
    // Here, "Middleware::pre" means we're adding a pre middleware which will be executed
    // before any route handlers.
    let mut r = Router::builder()
        // Specify the state data which will be available to every route handlers,
        // error handler and middlewares.
        .data(Env {
            client,
            state: State(100),
        });

    if LOG_LEVEL == log::LevelFilter::Debug {
        r = r.middleware(Middleware::pre(logger));
    }
    r.get("/", home_handler)
        .get("/users/:userId", user_handler)
        .get("/users/:userId/test", user_handler_2)
        .get("/*", proxy::proxy_handler)
        .err_handler_with_info(error_handler)
        .build()
        .unwrap()
}

// TODO: Rewrite this to sub-router and mutate headers etc in middleware.
// Check out https://github.com/routerify/routerify/blob/master/examples/scoped_router.rs
mod proxy {
    use super::*;

    pub async fn proxy_handler(mut req: Request<Body>) -> Result<Response<Body>> {
        // Access the app state.
        let env = req.data::<Env>().unwrap();
        let client = env.client.clone();
        debug!("State value: {}", env.state.0);

        rewrite_to_proxy(&mut req)?;
        client
            .request(req)
            .await
            .context("Making request to backend server")
    }

    fn rewrite_to_proxy(req: &mut Request<Body>) -> Result<()> {
        // Remove headers that may mess with reverse proxy behaviour.
        let blacklisted_headers = [
            "content-length",
            "transfer-encoding",
            "accept-encoding",
            "content-encoding",
        ];
        blacklisted_headers.iter().for_each(|key| {
            req.headers_mut().remove(*key);
        });

        let uri = req.uri();
        let uri_string = match uri.query() {
            None => format!("{}{}", PROXY_URL, uri.path()),
            Some(query) => format!("{}{}?{}", PROXY_URL, uri.path(), query),
        };
        *req.uri_mut() = uri_string
            .parse()
            .context("Parsing URI in rewrite_to_proxy")?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging_service()?;
    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Create a Service from the router above to handle incoming requests.
    let router = router();
    let service = RouterService::new(router).unwrap();

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    info!("App is running on: {}", addr);
    info!("Try calling http://localhost:3000/uuid to test the proxy.");
    server
        .await
        .context("Fatal server error resulting in the hyper server stopping")?;
    Ok::<(), anyhow::Error>(())
}
