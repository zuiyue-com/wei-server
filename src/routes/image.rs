pub async fn index() -> String {
    wei_run::run(
        "wei-qbittorrent", 
        vec![
            "list".to_owned()
        ]
    ).unwrap()
}

use axum::extract::Path;

pub async fn delete(Path(hash): Path<String>) -> String {
    wei_run::run(
        "wei-qbittorrent", 
        vec![
            "delete".to_owned(),
            hash
        ]
    ).unwrap()
}