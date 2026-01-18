use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// 缓存项信息
#[derive(Debug, Clone, Serialize)]
pub struct CacheItem {
    pub path: String,
    pub size: u64,
    pub file_count: u64,
    pub category: String,
}

/// 扫描结果
#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub items: Vec<CacheItem>,
    pub total_size: u64,
    pub total_files: u64,
}

/// 获取目录大小和文件数量
fn get_dir_stats(path: &PathBuf) -> (u64, u64) {
    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;

    if path.exists() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                    file_count += 1;
                }
            }
        }
    }

    (total_size, file_count)
}

/// 获取系统缓存路径
fn get_cache_paths() -> Vec<(PathBuf, String)> {
    let mut paths = Vec::new();

    // 用户临时目录 %TEMP%
    if let Ok(temp_dir) = env::var("TEMP") {
        paths.push((PathBuf::from(&temp_dir), "系统临时文件".to_string()));
    }

    // Windows临时目录
    let windows_temp = PathBuf::from("C:\\Windows\\Temp");
    if windows_temp.exists() {
        paths.push((windows_temp, "Windows临时文件".to_string()));
    }

    // Windows更新缓存
    let update_cache = PathBuf::from("C:\\Windows\\SoftwareDistribution\\Download");
    if update_cache.exists() {
        paths.push((update_cache, "Windows更新缓存".to_string()));
    }

    // 用户目录
    if let Ok(userprofile) = env::var("USERPROFILE") {
        let user_path = PathBuf::from(&userprofile);

        // Chrome缓存
        let chrome_cache = user_path
            .join("AppData")
            .join("Local")
            .join("Google")
            .join("Chrome")
            .join("User Data")
            .join("Default")
            .join("Cache");
        if chrome_cache.exists() {
            paths.push((chrome_cache, "Chrome浏览器缓存".to_string()));
        }

        // Edge缓存
        let edge_cache = user_path
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("Edge")
            .join("User Data")
            .join("Default")
            .join("Cache");
        if edge_cache.exists() {
            paths.push((edge_cache, "Edge浏览器缓存".to_string()));
        }

        // 缩略图缓存
        let thumbnail_cache = user_path
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("Windows")
            .join("Explorer");
        if thumbnail_cache.exists() {
            paths.push((thumbnail_cache, "缩略图缓存".to_string()));
        }

        // 最近文档
        let recent = user_path.join("AppData").join("Roaming").join("Microsoft").join("Windows").join("Recent");
        if recent.exists() {
            paths.push((recent, "最近文档记录".to_string()));
        }
    }

    paths
}

/// 扫描所有缓存
pub fn scan_all_caches() -> ScanResult {
    let cache_paths = get_cache_paths();
    let mut items = Vec::new();
    let mut total_size: u64 = 0;
    let mut total_files: u64 = 0;

    for (path, category) in cache_paths {
        let (size, count) = get_dir_stats(&path);
        if size > 0 {
            items.push(CacheItem {
                path: path.to_string_lossy().to_string(),
                size,
                file_count: count,
                category,
            });
            total_size += size;
            total_files += count;
        }
    }

    ScanResult {
        items,
        total_size,
        total_files,
    }
}

/// 清理指定路径的缓存
pub fn clean_cache(path: &str) -> Result<u64, String> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Ok(0);
    }

    let mut cleaned_size: u64 = 0;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                cleaned_size += metadata.len();
            }
            let _ = fs::remove_file(entry.path());
        }
    }

    // 尝试删除空目录
    for entry in WalkDir::new(&path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() && entry.path() != path {
            let _ = fs::remove_dir(entry.path());
        }
    }

    Ok(cleaned_size)
}
