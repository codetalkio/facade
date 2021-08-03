use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Client,
};
use std::sync::Arc;

pub const PROXY_URL: &str = "https://httpbin.org";
pub const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

pub struct Env {
    pub client: Arc<Client<hyper_rustls::HttpsConnector<HttpConnector<GaiResolver>>, hyper::Body>>,
    pub state: State,
}
pub struct State(pub u64);
