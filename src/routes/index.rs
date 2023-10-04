pub async fn index() -> &'static str {
    r#"{
        "code": 200,
        "msg": "success",
        "data": {
            "list": [
                {
                    "id": "1",
                    "uuid": "9e011f8e-975D-2A7b-BA0b-3390B78dcbED",
                    "image": "https://images.tusiassets.com/community/images/604726241320370765/1e934c23b64f9e98fa28cdd4f72be9d4.png",
                    "title": "绘世-启动器,SD-WebUI启动器",
                    "price": "免费",
                    "label": [
                        "AI绘画生成",
                        "Stable Diffusion启动器",
                        "WebUI启动器",
                        "文生图",
                        "绘世-启动器"
                    ],
                    "company": "秋葉aaaki",
                    "url": "https://anzu.link/",
                    "description": "绘世-启动器，原名SD-WebUI启动器，是一款由bilibili B站大佬@秋葉aaaki制作并免费发布的Stable Diffusion WebUI启动器电脑桌面版！无须安装部署，开箱即用，傻瓜式操作！"
                }
            ],
            "total": 1
        }
    }"#
}

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
    if v["code"].as_str() != Some("200") {
        return r#"{
            "code": 400,
            "msg": "failed",
            "data": "下载器没有启动，或者其它问题导致失败"
        }"#;
    }

    r#"{
        "code": 200,
        "msg": "success",
        "data": "绘世-启动器"
    }"#
}