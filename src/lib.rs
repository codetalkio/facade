use crate::types::*;
use anyhow::*;
use hyper::{Body, Client, Request, Response, StatusCode};
use log::debug;
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router};
use std::sync::Arc;

pub mod proxy;
pub mod types;

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
    Ok(Response::new(Body::from(format!("Hello {}\n", user_id))))
}

/// Set up the logging service.
///
/// This is where you will configure the output level, log format, etc.
pub fn setup_logging_service() -> Result<()> {
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
        .context("Setting up logging service")
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
async fn error_handler(err: routerify::RouteError, _: RequestInfo) -> Response<Body> {
    eprintln!("{}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(format!("Something went wrong: {}", err)))
        .unwrap()
}

// Create a `Router<Body, Infallible>` for response body type `hyper::Body`
// and for handler error type `Infallible`.
pub fn router() -> Router<Body, anyhow::Error> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();
    let client: Client<_, hyper::Body> = Client::builder().build(https);
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
