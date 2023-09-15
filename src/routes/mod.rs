// 暴露index.rs的index函数
use axum::Router;
use axum::routing::{get,post};
use tower_http::cors::{Any};

pub mod index;
pub mod image;
pub mod model;
pub mod user;

pub fn routes() -> Router {
    Router::new()
        .route("/index", get(index::index))
        .route("/index/download", get(index::download))
        .route("/user", get(user::manage))
        .route("/user/manage", get(user::manage))
        .route("/image", get(image::index))
        .route("/image/delete/:hash", get(image::delete))
        .route("/model", get(model::index))
        .route("/", get(|| async { "wei-server" }))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("*".parse::<axum::http::HeaderValue>().unwrap())
                .allow_headers(Any)
                .allow_methods(Any),
        )
}
