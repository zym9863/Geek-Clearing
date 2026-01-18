mod scanner;
mod shredder;
mod privacy;

use scanner::{CacheItem, ScanResult};
use privacy::PrivacyItem;

/// 扫描所有缓存
#[tauri::command]
fn scan_caches() -> ScanResult {
    scanner::scan_all_caches()
}

/// 清理指定缓存
#[tauri::command]
fn clean_cache(path: String) -> Result<u64, String> {
    scanner::clean_cache(&path)
}

/// 安全删除文件（DoD 5220.22-M）
#[tauri::command]
fn secure_delete_file(path: String) -> Result<(), String> {
    shredder::secure_delete(&path)
}

/// 安全删除目录
#[tauri::command]
fn secure_delete_dir(path: String) -> Result<u64, String> {
    shredder::secure_delete_directory(&path)
}

/// 获取隐私项目列表
#[tauri::command]
fn get_privacy_items() -> Vec<PrivacyItem> {
    privacy::get_privacy_items()
}

/// 清理隐私项目
#[tauri::command]
fn clean_privacy(path: String, secure: bool) -> Result<u64, String> {
    privacy::clean_privacy_item(&path, secure)
}

/// 格式化文件大小
#[tauri::command]
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scan_caches,
            clean_cache,
            secure_delete_file,
            secure_delete_dir,
            get_privacy_items,
            clean_privacy,
            format_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
