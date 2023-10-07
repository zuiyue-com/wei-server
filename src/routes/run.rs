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
}