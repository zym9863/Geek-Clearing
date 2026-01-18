import { invoke } from "@tauri-apps/api/core";

/** 缓存项信息 */
export interface CacheItem {
    path: string;
    size: number;
    file_count: number;
    category: string;
}

/** 扫描结果 */
export interface ScanResult {
    items: CacheItem[];
    total_size: number;
    total_files: number;
}

/** 隐私项目 */
export interface PrivacyItem {
    name: string;
    path: string;
    exists: boolean;
}

/** 扫描所有缓存 */
export async function scanCaches(): Promise<ScanResult> {
    return await invoke<ScanResult>("scan_caches");
}

/** 清理指定缓存 */
export async function cleanCache(path: string): Promise<number> {
    return await invoke<number>("clean_cache", { path });
}

/** 安全删除文件 (DoD 5220.22-M) */
export async function secureDeleteFile(path: string): Promise<void> {
    return await invoke("secure_delete_file", { path });
}

/** 安全删除目录 */
export async function secureDeleteDir(path: string): Promise<number> {
    return await invoke<number>("secure_delete_dir", { path });
}

/** 获取隐私项目列表 */
export async function getPrivacyItems(): Promise<PrivacyItem[]> {
    return await invoke<PrivacyItem[]>("get_privacy_items");
}

/** 清理隐私项目 */
export async function cleanPrivacy(path: string, secure: boolean): Promise<number> {
    return await invoke<number>("clean_privacy", { path, secure });
}

/** 格式化文件大小 */
export async function formatSize(bytes: number): Promise<string> {
    return await invoke<string>("format_size", { bytes });
}

/** 本地格式化文件大小 (无需调用Rust) */
export function formatSizeLocal(bytes: number): string {
    const KB = 1024;
    const MB = KB * 1024;
    const GB = MB * 1024;

    if (bytes >= GB) {
        return `${(bytes / GB).toFixed(2)} GB`;
    } else if (bytes >= MB) {
        return `${(bytes / MB).toFixed(2)} MB`;
    } else if (bytes >= KB) {
        return `${(bytes / KB).toFixed(2)} KB`;
    } else {
        return `${bytes} B`;
    }
}
