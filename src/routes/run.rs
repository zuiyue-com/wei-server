use axum::Json;

pub async fn index(Json(data): Json<Vec<String>>) -> String {
    let command: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    
    if command.len() < 1 {
        let data = format!("{}", serde_json::json!({
            "code": 400,
            "message": "missing param error, at least one param required"
        }));
        return data.to_string();
    }

    use async_process::{Command, Child, Stdio};
    #[cfg(target_os = "windows")]
    use async_process::windows::CommandExt;
    use tokio::time::{timeout, Duration};

    let command_path = "./".to_owned() + command[0];
    
    #[cfg(target_os = "windows")]
    let mut child: Child = Command::new(command_path)
        .args(command[1..].to_vec())
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn().unwrap();

    #[cfg(not(target_os = "windows"))]
    let mut child: Child = Command::new(command_path)
        .args(command[1..].to_vec())
        .stdout(Stdio::piped())
        .spawn().unwrap();

    let duration = Duration::from_secs(60);

    match timeout(duration, child.status()).await {
        Ok(result) => match result {
            Ok(_) => {
                let data = child.output().await.unwrap();
                let data = String::from_utf8_lossy(&data.stdout).into_owned();
                data
            },
            Err(e) => {
                format!("{}", 
                serde_json::json!({
                    "code": 400,
                    "message": format!("任务执行失败: {}", e)
                }))
            }
        },
        Err(_) => {
            // timeout elapsed, kill the process
            child.kill().unwrap();
            child.status().await.unwrap();

            format!("{}", 
                serde_json::json!({
                    "code": 400,
                    "message": format!("任务超时")
            }))
        }
    }
}

pub async fn index_async(Json(data): Json<Vec<String>>) -> String {
    let command: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    
    if command.len() < 1 {
        let data = format!("{}", serde_json::json!({
            "code": 400,
            "message": "missing param error, at least one param required"
        }));
        return data.to_string();
    }

    wei_run::run_async(command[0], (&command[1..]).to_vec()).unwrap();

    format!("{}", 
        serde_json::json!({
            "code": 200,
            "message": "success"
    }))
}

