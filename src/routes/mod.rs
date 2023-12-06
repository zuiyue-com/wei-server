use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any};

use once_cell::sync::Lazy;
use sled::Db;

pub mod run;
pub mod index;
pub mod image;
pub mod task;
pub mod setting;

static DB_TASK: Lazy<Db> = Lazy::new(|| {
    let db_path = format!("{}wei-task-db", wei_env::home_dir().unwrap());
    sled::open(&db_path).unwrap()
});

pub fn routes() -> Router {
    let proxy_target = std::sync::Arc::new(ProxyTarget {
        target_uri: match std::fs::read_to_string("server.dat") {
            Ok(content) => content.trim().parse().unwrap(),
            Err(_) => "https://www.zuiyue.com".parse().unwrap(),
        },
    });

    Router::new()
        .route("/run", post(run::index))
        .route("/run/async", post(run::index_async))

        .route("/setting/autorun", get(setting::autorun))
        .route("/setting/unautorun", get(setting::unautorun))

        .route("/index/download", get(index::download))

        .route("/image", get(image::index))
        .route("/image/delete/:hash", get(image::delete))

        .route("/task/list", post(task::list))
        .route("/task/insert", post(task::insert))
        .route("/task/delete", post(task::delete))

        .route("/version", get(|| async { "wei-server" }))
        .route("/api/*rest", get(api_proxy))
        .route("/api/*rest", post(api_proxy))
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

    let path_and_query = path_and_query.replacen("/api/", "", 1);
 
    let uri = format!("{}{}", proxy_target.target_uri, path_and_query).parse::<hyper::Uri>().unwrap();
    
    *req.uri_mut() = uri.clone();

    let host = proxy_target.target_uri.host().unwrap().to_string();
    let url = proxy_target.target_uri.to_string();
    let headers = req.headers_mut();

    // 设置或修改 Host 和 Referer 头部
    use hyper::header::HeaderValue;
    headers.insert("Host", HeaderValue::from_str(&host).unwrap());
    headers.insert("Referer", HeaderValue::from_str(&url).unwrap());

    let timeout_duration = tokio::time::Duration::from_secs(30);
    
    // Forward the request to the target URI
    match tokio::time::timeout(timeout_duration, client.request(req)).await {
        Ok(Ok(mut res)) => {
            let mut body_bytes = hyper::body::to_bytes(res.body_mut()).await.unwrap();

            // Check if the response is gzipped
            if res.headers().get("content-encoding") == Some(&hyper::header::HeaderValue::from_static("gzip")) {
                // If the response is gzipped, decode it
                let mut d = GzDecoder::new(&*body_bytes);
                let mut decoded_body = Vec::new();
                d.read_to_end(&mut decoded_body).unwrap();
                body_bytes = hyper::body::Bytes::from(decoded_body);
            }
            
            let body = hyper::Body::from(body_bytes.clone());

            let res = hyper::Response::builder()
                .status(res.status())
                .body(body)
                .unwrap();
            
            let body = std::str::from_utf8(&body_bytes).unwrap();
            // info!("wei-server uri: {}, body: {}", uri, &body[0..300.min(body.len())]);
            return Ok(res);
        },
        Ok(Err(err)) => {
            // info!("wei-server uri: {}, err: {}", uri, err);
            Ok(hyper::Response::builder()
                .status(500)
                .body(hyper::Body::from("Internal server error"))
                .unwrap())
        }
        Err(_) => {
            // info!("wei-server uri: {}, timeout", uri);
            Ok(hyper::Response::builder()
                .status(504)
                .body(hyper::Body::from("Gateway Timeout"))
                .unwrap())
        }
    }
}

struct ProxyTarget {
    target_uri: hyper::Uri,
}