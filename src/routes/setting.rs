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