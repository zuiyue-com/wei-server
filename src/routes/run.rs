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

    // use tokio::time::{timeout, Duration};

    // let result = timeout(Duration::from_secs(5), run_task(command)).await.unwrap();

    // match result {
    //     Ok(value) => value,
    //     Err(_) => {
    //         let data = format!("{}", serde_json::json!({
    //             "code": "400",
    //             "msg": format!("任务超时")
    //         }));
    //         data
    //     }
    // }

    use async_process::{Command, Child};
    use tokio::time::{timeout, Duration};
    
    let mut child: Child = Command::new(command[0])
        .args(command[1..].to_vec())
        .spawn().unwrap();

    let duration = Duration::from_secs(5);

    match timeout(duration, child.status()).await {
        Ok(result) => match result {
            Ok(_) => {
                let data = child.output().await.unwrap();
                println!("here:{:?}", data);
                let data = String::from_utf8_lossy(&data.stdout).into_owned();
                format!("{}", 
                serde_json::json!({
                    "code": "200",
                    "msg": "success",
                    "data": data
                }))
            },
            Err(e) => {
                format!("{}", 
                serde_json::json!({
                    "code": "400",
                    "msg": format!("任务执行失败: {}", e)
                }))
            }
        },
        Err(_) => {
            // timeout elapsed, kill the process
            child.kill().unwrap();
            child.status().await.unwrap();

            format!("{}", 
                serde_json::json!({
                    "code": "400",
                    "msg": format!("任务超时")
            }))
        }
    }
}
