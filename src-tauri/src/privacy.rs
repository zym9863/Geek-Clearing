use std::env;
use std::fs;
use std::path::PathBuf;

/// 清理浏览器数据类型
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrivacyItem {
    pub name: String,
    pub path: String,
    pub exists: bool,
}

/// 获取隐私相关路径
pub fn get_privacy_items() -> Vec<PrivacyItem> {
    let mut items = Vec::new();

    if let Ok(userprofile) = env::var("USERPROFILE") {
        let user_path = PathBuf::from(&userprofile);

        // Chrome历史
        let chrome_history = user_path
            .join("AppData")
            .join("Local")
            .join("Google")
            .join("Chrome")
            .join("User Data")
            .join("Default")
            .join("History");
        items.push(PrivacyItem {
            name: "Chrome浏览历史".to_string(),
            path: chrome_history.to_string_lossy().to_string(),
            exists: chrome_history.exists(),
        });

        // Edge历史
        let edge_history = user_path
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("Edge")
            .join("User Data")
            .join("Default")
            .join("History");
        items.push(PrivacyItem {
            name: "Edge浏览历史".to_string(),
            path: edge_history.to_string_lossy().to_string(),
            exists: edge_history.exists(),
        });

        // 最近访问的文件
        let recent = user_path
            .join("AppData")
            .join("Roaming")
            .join("Microsoft")
            .join("Windows")
            .join("Recent");
        items.push(PrivacyItem {
            name: "最近访问文件".to_string(),
            path: recent.to_string_lossy().to_string(),
            exists: recent.exists(),
        });

        // Windows搜索历史
        let search_history = user_path
            .join("AppData")
            .join("Local")
            .join("Packages")
            .join("Microsoft.Windows.Search_cw5n1h2txyewy")
            .join("LocalState")
            .join("ConstraintIndex")
            .join("Apps");
        items.push(PrivacyItem {
            name: "搜索历史".to_string(),
            path: search_history.to_string_lossy().to_string(),
            exists: search_history.exists(),
        });

        // 剪贴板历史 (Windows 10+)
        let clipboard = user_path
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("Windows")
            .join("Clipboard");
        items.push(PrivacyItem {
            name: "剪贴板历史".to_string(),
            path: clipboard.to_string_lossy().to_string(),
            exists: clipboard.exists(),
        });
    }

    items
}

/// 清理隐私项目
pub fn clean_privacy_item(path: &str, secure: bool) -> Result<u64, String> {
    let path = PathBuf::from(path);
    
    if !path.exists() {
        return Ok(0);
    }

    let mut cleaned: u64 = 0;

    if path.is_file() {
        if secure {
            crate::shredder::secure_delete(path.to_str().unwrap_or(""))?;
        } else {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
        cleaned = 1;
    } else if path.is_dir() {
        for entry in walkdir::WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if secure {
                    if crate::shredder::secure_delete(entry.path().to_str().unwrap_or("")).is_ok() {
                        cleaned += 1;
                    }
                } else {
                    if fs::remove_file(entry.path()).is_ok() {
                        cleaned += 1;
                    }
                }
            }
        }

        // 清理空目录
        for entry in walkdir::WalkDir::new(&path)
            .contents_first(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() && entry.path() != path {
                let _ = fs::remove_dir(entry.path());
            }
        }
    }

    Ok(cleaned)
}
