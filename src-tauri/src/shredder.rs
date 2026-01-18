use rand::Rng;
use std::fs::{self, File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

/// 安全删除进度回调
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send>;

/// DoD 5220.22-M 安全删除算法
/// 3次覆写：全0 -> 全1 -> 随机数据
pub fn secure_delete(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    if !path.is_file() {
        return Err("路径不是文件".to_string());
    }

    // 获取文件大小
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_size = metadata.len();

    if file_size == 0 {
        fs::remove_file(path).map_err(|e| e.to_string())?;
        return Ok(());
    }

    // 打开文件进行覆写
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;

    const BUFFER_SIZE: usize = 64 * 1024; // 64KB 缓冲区

    // Pass 1: 覆写全0
    overwrite_with_pattern(&mut file, file_size, 0x00, BUFFER_SIZE)?;

    // Pass 2: 覆写全1
    overwrite_with_pattern(&mut file, file_size, 0xFF, BUFFER_SIZE)?;

    // Pass 3: 覆写随机数据
    overwrite_with_random(&mut file, file_size, BUFFER_SIZE)?;

    // 同步到磁盘
    file.sync_all().map_err(|e| e.to_string())?;

    // 关闭文件句柄
    drop(file);

    // 删除文件
    fs::remove_file(path).map_err(|e| e.to_string())?;

    Ok(())
}

/// 使用固定模式覆写文件
fn overwrite_with_pattern(file: &mut File, size: u64, pattern: u8, buffer_size: usize) -> Result<(), String> {
    file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    
    let buffer = vec![pattern; buffer_size];
    let mut remaining = size;

    while remaining > 0 {
        let write_size = std::cmp::min(remaining as usize, buffer_size);
        file.write_all(&buffer[..write_size]).map_err(|e| e.to_string())?;
        remaining -= write_size as u64;
    }

    file.sync_all().map_err(|e| e.to_string())?;
    Ok(())
}

/// 使用随机数据覆写文件
fn overwrite_with_random(file: &mut File, size: u64, buffer_size: usize) -> Result<(), String> {
    file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    
    let mut rng = rand::thread_rng();
    let mut buffer = vec![0u8; buffer_size];
    let mut remaining = size;

    while remaining > 0 {
        let write_size = std::cmp::min(remaining as usize, buffer_size);
        rng.fill(&mut buffer[..write_size]);
        file.write_all(&buffer[..write_size]).map_err(|e| e.to_string())?;
        remaining -= write_size as u64;
    }

    file.sync_all().map_err(|e| e.to_string())?;
    Ok(())
}

/// 安全删除目录中的所有文件
pub fn secure_delete_directory(dir_path: &str) -> Result<u64, String> {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        return Ok(0);
    }

    let mut deleted_count: u64 = 0;

    for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if secure_delete(entry.path().to_str().unwrap_or("")).is_ok() {
                deleted_count += 1;
            }
        }
    }

    // 删除空目录
    for entry in walkdir::WalkDir::new(path)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() && entry.path() != path {
            let _ = fs::remove_dir(entry.path());
        }
    }

    Ok(deleted_count)
}
