use axum::Json;

pub async fn index(Json(data): Json<Vec<String>>) -> String {
    let command: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    
    if command.len() < 1 {
        let data = format!("{}", serde_json::json!({
            "code": "400",
            "msg": "missing param error, at least one param required"
        }));
        return data.to_string();
    }

    use tokio::time::{timeout, Duration};

    let handle = async {
        match wei_run::run(command[0], command[1..].to_vec()) {
            Ok(data) => data.to_string(),
            Err(e) => {
                let data = format!("{}", serde_json::json!({
                    "code": "400",
                    "msg": format!("{}", e)
                }));
                data
            }
        }
    };

    let result = timeout(Duration::from_secs(5), handle).await;
    
    match result {
        Ok(value) => value,
        // Ok(Err(e)) => {
        //     let data = format!("{}", serde_json::json!({
        //         "code": "400",
        //         "msg": format!("任务出错，错误：{}", e)
        //     }));
        //     data
        // }
        Err(_) => {
            let data = format!("{}", serde_json::json!({
                "code": "400",
                "msg": format!("任务超时")
            }));
            data
        }
    }
}