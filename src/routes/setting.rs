use winreg::enums::*;
use winreg::RegKey;
use std::env;

pub async fn autorun() -> String {
    let path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            return format!("{}", serde_json::json!({
                "code": 400,
                "message": format!("获取当前程序路径失败: {}", e)
            }))
        }
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path_str = path.to_str().ok_or("Invalid path").unwrap();

    let (key, _) = match hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
        Ok(key) => key,
        Err(e) => {
            return format!("{}", serde_json::json!({
                "code": 400,
                "message": format!("创建注册表失败: {}", e)
            }))
        }
    };

    match key.set_value("Wei", &path_str) {
        Ok(_) => (),
        Err(e) => {
            return format!("{}", serde_json::json!({
                "code": 400,
                "message": format!("设置注册表失败: {}", e)
            }))
        }
    };

    format!("{}", serde_json::json!({
        "code": 200,
        "message": "success"
    }))
}


pub async fn unautorun() -> String {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let key = match hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE) {
        Ok(key) => key,
        Err(e) => {
            return format!("{}", serde_json::json!({
                "code": 400,
                "message": format!("读取注册表失败: {}", e)
            }))
        }
    };
    
    match key.delete_value("Wei") {
        Ok(_) => (),
        Err(e) => {
            return format!("{}", serde_json::json!({
                "code": 400,
                "message": format!("删除注册表失败: {}", e)
            }))
        }
    };

    format!("{}", serde_json::json!({
        "code": 200,
        "message": "success"
    }))
}

use axum::Json;

pub async fn token_write_once(Json(data): Json<Vec<String>>) -> String {
    let command: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    let data = command[0];

    let token_path = format!("{}token.dat", wei_env::home_dir().unwrap());

    // 如果文件存在，内容为空，则写入，
    // 如果文件不存在，则创建并写入
    // 如果文件存在，不为空，则不写入
    if std::path::Path::new(&token_path).exists() {
        let content = match std::fs::read_to_string(&token_path) {
            Ok(content) => content,
            Err(_) => {
                return format!("{}", serde_json::json!({
                    "code": 400,
                    "message": "读取文件失败"
                }))
            }
        };

        if content.is_empty() {
            match std::fs::write(&token_path, data) {
                Ok(_) => (),
                Err(_) => {
                    return format!("{}", serde_json::json!({
                        "code": 400,
                        "message": "写入文件失败"
                    }))
                }
            }
        }
    } else {
        match std::fs::write(&token_path, data) {
            Ok(_) => (),
            Err(_) => {
                return format!("{}", serde_json::json!({
                    "code": 400,
                    "message": "写入文件失败"
                }))
            }
        }
    }

    format!("{}", serde_json::json!({
        "code": 200,
        "message": "success"
    }))
}


pub async fn token_write(Json(data): Json<Vec<String>>) -> String {
    let command: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    let data = command[0];

    let token_path = format!("{}token.dat", wei_env::home_dir().unwrap());

    // 如果文件存在，则写入，
    // 如果文件不存在，则创建并写入
    if std::path::Path::new(&token_path).exists() {
        match std::fs::write(&token_path, data) {
            Ok(_) => (),
            Err(_) => {
                return format!("{}", serde_json::json!({
                    "code": 400,
                    "message": "写入文件失败"
                }))
            }
        }
    } else {
        match std::fs::write(&token_path, data) {
            Ok(_) => (),
            Err(_) => {
                return format!("{}", serde_json::json!({
                    "code": 400,
                    "message": "写入文件失败"
                }))
            }
        }
    }

    format!("{}", serde_json::json!({
        "code": 200,
        "message": "success"
    }))
}
