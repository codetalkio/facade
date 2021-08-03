use crate::types::*;
use anyhow::*;
use hyper::{Body, Request, Response};
use log::debug;
use routerify::prelude::*;

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
