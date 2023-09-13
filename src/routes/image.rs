pub async fn index() -> String {
    wei_run::run(
        "wei-qbittorrent", 
        vec![
            "list".to_owned()
        ]
    ).unwrap()
}