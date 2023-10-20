use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any};

pub mod run;
pub mod index;
pub mod image;
pub mod model;
pub mod user;

pub fn routes() -> Router {

    let proxy_target = std::sync::Arc::new(ProxyTarget {
        target_uri: "https://www.zuiyue.com/discuz".parse().unwrap(),
    });


    Router::new()
        .route("/run", post(run::index))
        .route("/index", get(index::index))
        .route("/index/download", get(index::download))
        .route("/user", get(user::manage))
        .route("/user/manage", get(user::manage))
        .route("/image", get(image::index))
        .route("/image/delete/:hash", get(image::delete))
        .route("/model", get(model::index))
        .route("/version", get(|| async { "wei-server" }))
        .route("/api/:rest", get(api_proxy))
        .nest_service("/", tower_http::services::ServeDir::new("dist"))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("*".parse::<axum::http::HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .layer(
            tower_http::add_extension::AddExtensionLayer::new(proxy_target)
        )
}

use flate2::read::GzDecoder;
use std::io::Read;

async fn api_proxy(
    axum::extract::Path((_,)): axum::extract::Path<(String,)>,
    axum::Extension(proxy_target): axum::Extension<std::sync::Arc<ProxyTarget>>,
    mut req: axum::http::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {

    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let path_and_query = match req.uri().path_and_query() {
        Some(v) => v.as_str(),
        None => "",
    };

    let path_and_query = path_and_query.replacen("/api/", "/", 1);
 
    let uri = format!("{}{}", proxy_target.target_uri, path_and_query).parse::<hyper::Uri>().unwrap();
    *req.uri_mut() = uri;

    // Forward the request to the target URI
    match client.request(req).await {
        Ok(mut res) => {
            let mut body_bytes = hyper::body::to_bytes(res.body_mut()).await.unwrap();

            // Check if the response is gzipped
            if res.headers().get("content-encoding") == Some(&hyper::header::HeaderValue::from_static("gzip")) {
                // If the response is gzipped, decode it
                let mut d = GzDecoder::new(&*body_bytes);
                let mut decoded_body = Vec::new();
                d.read_to_end(&mut decoded_body).unwrap();
                body_bytes = hyper::body::Bytes::from(decoded_body);
            }
            
            let body = hyper::Body::from(body_bytes);

            let res = hyper::Response::builder()
                .status(res.status())
                .body(body)
                .unwrap();
            
            return Ok(res);
        },
        Err(err) => {
            Ok(hyper::Response::builder()
                .status(500)
                .body(hyper::Body::from("Internal server error"))
                .unwrap())
        }
    }
}

struct ProxyTarget {
    target_uri: hyper::Uri,
}