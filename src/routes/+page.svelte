<script lang="ts">
  import { onMount } from "svelte";
  import {
    scanCaches,
    cleanCache,
    getPrivacyItems,
    cleanPrivacy,
    formatSizeLocal,
    type CacheItem,
    type ScanResult,
    type PrivacyItem,
  } from "$lib/api";

  // 状态
  let activeTab = $state<"cache" | "privacy">("cache");
  let isScanning = $state(false);
  let isCleaning = $state(false);
  let scanResult = $state<ScanResult | null>(null);
  let privacyItems = $state<PrivacyItem[]>([]);
  let selectedCaches = $state<Set<string>>(new Set());
  let selectedPrivacy = $state<Set<string>>(new Set());
  let cleanedSize = $state(0);
  let statusMessage = $state("");

  // 扫描缓存
  async function handleScan() {
    isScanning = true;
    statusMessage = "正在扫描系统缓存...";
    try {
      scanResult = await scanCaches();
      // 默认全选
      selectedCaches = new Set(scanResult.items.map((item) => item.path));
      statusMessage = `扫描完成，发现 ${formatSizeLocal(scanResult.total_size)} 可清理空间`;
    } catch (error) {
      statusMessage = `扫描失败: ${error}`;
    } finally {
      isScanning = false;
    }
  }

  // 扫描隐私项
  async function handleScanPrivacy() {
    isScanning = true;
    statusMessage = "正在扫描隐私痕迹...";
    try {
      privacyItems = await getPrivacyItems();
      selectedPrivacy = new Set(
        privacyItems.filter((item) => item.exists).map((item) => item.path)
      );
      statusMessage = `发现 ${privacyItems.filter((i) => i.exists).length} 项隐私痕迹`;
    } catch (error) {
      statusMessage = `扫描失败: ${error}`;
    } finally {
      isScanning = false;
    }
  }

  // 清理缓存
  async function handleClean() {
    if (selectedCaches.size === 0) {
      statusMessage = "请先选择要清理的项目";
      return;
    }

    isCleaning = true;
    cleanedSize = 0;
    statusMessage = "正在清理...";

    try {
      for (const path of selectedCaches) {
        const size = await cleanCache(path);
        cleanedSize += size;
      }
      statusMessage = `清理完成，已释放 ${formatSizeLocal(cleanedSize)} 空间`;
      // 重新扫描
      await handleScan();
    } catch (error) {
      statusMessage = `清理失败: ${error}`;
    } finally {
      isCleaning = false;
    }
  }

  // 清理隐私（安全删除）
  async function handleCleanPrivacy() {
    if (selectedPrivacy.size === 0) {
      statusMessage = "请先选择要粉碎的项目";
      return;
    }

    isCleaning = true;
    let cleanedCount = 0;
    statusMessage = "正在安全粉碎隐私痕迹...";

    try {
      for (const path of selectedPrivacy) {
        const count = await cleanPrivacy(path, true);
        cleanedCount += count;
      }
      statusMessage = `粉碎完成，已安全删除 ${cleanedCount} 个文件`;
      // 重新扫描
      await handleScanPrivacy();
    } catch (error) {
      statusMessage = `粉碎失败: ${error}`;
    } finally {
      isCleaning = false;
    }
  }

  // 切换选择
  function toggleCache(path: string) {
    const newSet = new Set(selectedCaches);
    if (newSet.has(path)) {
      newSet.delete(path);
    } else {
      newSet.add(path);
    }
    selectedCaches = newSet;
  }

  function togglePrivacy(path: string) {
    const newSet = new Set(selectedPrivacy);
    if (newSet.has(path)) {
      newSet.delete(path);
    } else {
      newSet.add(path);
    }
    selectedPrivacy = newSet;
  }

  // 初始化
  onMount(() => {
    handleScan();
    handleScanPrivacy();
  });
</script>

<main class="app">
  <header class="header">
    <div class="logo">
      <span class="icon">⚡</span>
      <h1>极客清空</h1>
    </div>
    <p class="subtitle">智能缓存清理 · 隐私痕迹粉碎</p>
  </header>

  <nav class="tabs">
    <button
      class="tab"
      class:active={activeTab === "cache"}
      onclick={() => (activeTab = "cache")}
    >
      <span class="tab-icon">💾</span>
      智能缓存清理
    </button>
    <button
      class="tab"
      class:active={activeTab === "privacy"}
      onclick={() => (activeTab = "privacy")}
    >
      <span class="tab-icon">🔒</span>
      隐私痕迹粉碎
    </button>
  </nav>

  <section class="content">
    {#if activeTab === "cache"}
      <div class="panel">
        <div class="panel-header">
          <h2>缓存扫描结果</h2>
          <button
            class="btn btn-secondary"
            onclick={handleScan}
            disabled={isScanning}
          >
            {isScanning ? "扫描中..." : "重新扫描"}
          </button>
        </div>

        {#if scanResult && scanResult.items.length > 0}
          <div class="stats">
            <div class="stat">
              <span class="stat-value"
                >{formatSizeLocal(scanResult.total_size)}</span
              >
              <span class="stat-label">可清理空间</span>
            </div>
            <div class="stat">
              <span class="stat-value">{scanResult.total_files}</span>
              <span class="stat-label">文件数量</span>
            </div>
          </div>

          <ul class="item-list">
            {#each scanResult.items as item}
              <li class="item" class:selected={selectedCaches.has(item.path)}>
                <label class="item-checkbox">
                  <input
                    type="checkbox"
                    checked={selectedCaches.has(item.path)}
                    onchange={() => toggleCache(item.path)}
                  />
                  <span class="checkmark"></span>
                </label>
                <div class="item-info">
                  <span class="item-name">{item.category}</span>
                  <span class="item-path">{item.path}</span>
                </div>
                <div class="item-stats">
                  <span class="item-size">{formatSizeLocal(item.size)}</span>
                  <span class="item-count">{item.file_count} 个文件</span>
                </div>
              </li>
            {/each}
          </ul>
        {:else if !isScanning}
          <div class="empty">
            <p>暂无可清理的缓存</p>
          </div>
        {/if}

        <div class="actions">
          <button
            class="btn btn-primary btn-large"
            onclick={handleClean}
            disabled={isCleaning || selectedCaches.size === 0}
          >
            {#if isCleaning}
              <span class="spinner"></span>
              清理中...
            {:else}
              🚀 一键清理
            {/if}
          </button>
        </div>
      </div>
    {:else}
      <div class="panel">
        <div class="panel-header">
          <h2>隐私痕迹</h2>
          <button
            class="btn btn-secondary"
            onclick={handleScanPrivacy}
            disabled={isScanning}
          >
            {isScanning ? "扫描中..." : "重新扫描"}
          </button>
        </div>

        <div class="warning-box">
          <span class="warning-icon">⚠️</span>
          <p>
            隐私粉碎采用 DoD 5220.22-M 军事级安全删除算法，数据将被彻底覆写
            <strong>无法恢复</strong>
          </p>
        </div>

        {#if privacyItems.length > 0}
          <ul class="item-list">
            {#each privacyItems as item}
              <li
                class="item"
                class:selected={selectedPrivacy.has(item.path)}
                class:disabled={!item.exists}
              >
                <label class="item-checkbox">
                  <input
                    type="checkbox"
                    checked={selectedPrivacy.has(item.path)}
                    disabled={!item.exists}
                    onchange={() => togglePrivacy(item.path)}
                  />
                  <span class="checkmark"></span>
                </label>
                <div class="item-info">
                  <span class="item-name">{item.name}</span>
                  <span class="item-path">{item.path}</span>
                </div>
                <div class="item-status">
                  {#if item.exists}
                    <span class="status-exists">存在</span>
                  {:else}
                    <span class="status-clean">已清理</span>
                  {/if}
                </div>
              </li>
            {/each}
          </ul>
        {:else if !isScanning}
          <div class="empty">
            <p>暂无隐私痕迹</p>
          </div>
        {/if}

        <div class="actions">
          <button
            class="btn btn-danger btn-large"
            onclick={handleCleanPrivacy}
            disabled={isCleaning || selectedPrivacy.size === 0}
          >
            {#if isCleaning}
              <span class="spinner"></span>
              粉碎中...
            {:else}
              🔐 安全粉碎
            {/if}
          </button>
        </div>
      </div>
    {/if}
  </section>

  <footer class="status-bar">
    {#if isScanning}
      <span class="spinner"></span>
    {/if}
    <span class="status-text">{statusMessage}</span>
  </footer>
</main>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: "Segoe UI", "Microsoft YaHei", sans-serif;
    background: linear-gradient(135deg, #0d1117 0%, #161b22 50%, #0d1117 100%);
    color: #e6edf3;
    min-height: 100vh;
    overflow-x: hidden;
  }

  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
    max-width: 900px;
    margin: 0 auto;
    padding: 24px;
  }

  .header {
    text-align: center;
    margin-bottom: 32px;
  }

  .logo {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin-bottom: 8px;
  }

  .logo .icon {
    font-size: 2.5rem;
    filter: drop-shadow(0 0 10px #58a6ff);
  }

  .logo h1 {
    font-size: 2rem;
    font-weight: 700;
    background: linear-gradient(90deg, #58a6ff, #a371f7);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    color: #8b949e;
    font-size: 0.95rem;
  }

  .tabs {
    display: flex;
    gap: 8px;
    margin-bottom: 24px;
    background: rgba(22, 27, 34, 0.8);
    padding: 6px;
    border-radius: 12px;
    border: 1px solid #30363d;
  }

  .tab {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 14px 20px;
    border: none;
    background: transparent;
    color: #8b949e;
    font-size: 1rem;
    font-weight: 500;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tab:hover {
    color: #e6edf3;
    background: rgba(88, 166, 255, 0.1);
  }

  .tab.active {
    background: linear-gradient(135deg, #238636 0%, #2ea043 100%);
    color: #fff;
    box-shadow: 0 4px 12px rgba(46, 160, 67, 0.3);
  }

  .tab-icon {
    font-size: 1.2rem;
  }

  .content {
    flex: 1;
  }

  .panel {
    background: rgba(22, 27, 34, 0.9);
    border: 1px solid #30363d;
    border-radius: 16px;
    padding: 24px;
    backdrop-filter: blur(10px);
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .panel-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #e6edf3;
  }

  .stats {
    display: flex;
    gap: 24px;
    margin-bottom: 24px;
    padding: 20px;
    background: rgba(88, 166, 255, 0.1);
    border-radius: 12px;
    border: 1px solid rgba(88, 166, 255, 0.2);
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: #58a6ff;
  }

  .stat-label {
    font-size: 0.85rem;
    color: #8b949e;
    margin-top: 4px;
  }

  .item-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 320px;
    overflow-y: auto;
    margin-bottom: 24px;
  }

  .item {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px;
    background: rgba(48, 54, 61, 0.5);
    border: 1px solid #30363d;
    border-radius: 10px;
    transition: all 0.2s ease;
  }

  .item:hover {
    background: rgba(48, 54, 61, 0.8);
    border-color: #58a6ff;
  }

  .item.selected {
    border-color: #58a6ff;
    background: rgba(88, 166, 255, 0.1);
  }

  .item.disabled {
    opacity: 0.5;
  }

  .item-checkbox {
    position: relative;
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .item-checkbox input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkmark {
    width: 22px;
    height: 22px;
    border: 2px solid #30363d;
    border-radius: 6px;
    background: #0d1117;
    transition: all 0.2s ease;
  }

  .item-checkbox input:checked + .checkmark {
    background: linear-gradient(135deg, #238636, #2ea043);
    border-color: #2ea043;
  }

  .item-checkbox input:checked + .checkmark::after {
    content: "✓";
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #fff;
    font-size: 14px;
    font-weight: bold;
  }

  .item-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .item-name {
    font-weight: 500;
    color: #e6edf3;
  }

  .item-path {
    font-size: 0.8rem;
    color: #8b949e;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-stats {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .item-size {
    font-weight: 600;
    color: #f0883e;
  }

  .item-count {
    font-size: 0.8rem;
    color: #8b949e;
  }

  .item-status {
    min-width: 60px;
    text-align: center;
  }

  .status-exists {
    color: #f0883e;
    font-size: 0.85rem;
  }

  .status-clean {
    color: #3fb950;
    font-size: 0.85rem;
  }

  .warning-box {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    background: rgba(240, 136, 62, 0.1);
    border: 1px solid rgba(240, 136, 62, 0.3);
    border-radius: 10px;
    margin-bottom: 20px;
  }

  .warning-icon {
    font-size: 1.5rem;
  }

  .warning-box p {
    font-size: 0.9rem;
    color: #f0883e;
    line-height: 1.5;
  }

  .warning-box strong {
    color: #f85149;
  }

  .actions {
    display: flex;
    justify-content: center;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px 20px;
    border: none;
    border-radius: 8px;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: rgba(48, 54, 61, 0.8);
    color: #e6edf3;
    border: 1px solid #30363d;
  }

  .btn-secondary:hover:not(:disabled) {
    background: rgba(48, 54, 61, 1);
    border-color: #58a6ff;
  }

  .btn-primary {
    background: linear-gradient(135deg, #238636 0%, #2ea043 100%);
    color: #fff;
  }

  .btn-primary:hover:not(:disabled) {
    box-shadow: 0 4px 16px rgba(46, 160, 67, 0.4);
    transform: translateY(-1px);
  }

  .btn-danger {
    background: linear-gradient(135deg, #da3633 0%, #f85149 100%);
    color: #fff;
  }

  .btn-danger:hover:not(:disabled) {
    box-shadow: 0 4px 16px rgba(248, 81, 73, 0.4);
    transform: translateY(-1px);
  }

  .btn-large {
    padding: 16px 48px;
    font-size: 1.1rem;
    border-radius: 12px;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .empty {
    text-align: center;
    padding: 48px 24px;
    color: #8b949e;
  }

  .status-bar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    margin-top: 24px;
    padding: 12px;
    background: rgba(22, 27, 34, 0.8);
    border-radius: 8px;
    border: 1px solid #30363d;
  }

  .status-text {
    font-size: 0.9rem;
    color: #8b949e;
  }

  /* 滚动条样式 */
  .item-list::-webkit-scrollbar {
    width: 8px;
  }

  .item-list::-webkit-scrollbar-track {
    background: rgba(48, 54, 61, 0.3);
    border-radius: 4px;
  }

  .item-list::-webkit-scrollbar-thumb {
    background: #30363d;
    border-radius: 4px;
  }

  .item-list::-webkit-scrollbar-thumb:hover {
    background: #484f58;
  }
</style>
