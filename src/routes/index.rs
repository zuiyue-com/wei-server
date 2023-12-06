pub async fn download() ->  &'static str {
    
    let torrent = format!("http://download.zuiyue.com/windows/0.1.37.torrent");

    // 使用qbittorrent下载数据
    let path = std::env::current_dir().unwrap().join("test");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    
    let data = wei_run::run(
        "wei-qbittorrent", 
        vec![
            "add",
            torrent.as_str(),
            path.display().to_string().as_str()
        ]
    ).unwrap();

    let v: serde_json::Value = serde_json::from_str(&data).unwrap();
    if v["code"] != 200 {
        return r#"{
            "code": 400,
            "message": "下载器没有启动，或者其它问题导致失败",
            "data": "下载器没有启动，或者其它问题导致失败"
        }"#;
    }

    r#"{
        "code": 200,
        "message": "绘世-启动器",
        "data": "绘世-启动器"
    }"#
}