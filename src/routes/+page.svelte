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
  <header class="hero">
    <div class="brand">
      <div class="brand-icon">⚡</div>
      <div class="brand-text">
        <h1>极客清空</h1>
        <p class="subtitle">智能缓存清理 · 隐私痕迹粉碎</p>
      </div>
    </div>
    <div class="hero-actions">
      <div class="status-pill" aria-live="polite">
        {#if isScanning}
          <span class="pulse"></span>
          扫描中
        {:else if isCleaning}
          <span class="pulse danger"></span>
          执行中
        {:else}
          <span class="pulse ok"></span>
          就绪
        {/if}
      </div>
      <div class="meta-chip">Tauri · Windows</div>
    </div>
  </header>

  <nav class="tabs" aria-label="功能切换">
    <button
      class="tab"
      class:active={activeTab === "cache"}
      onclick={() => (activeTab = "cache")}
    >
      <span class="tab-icon">💾</span>
      <span class="tab-text">
        <span class="tab-title">智能缓存清理</span>
        <span class="tab-desc">释放空间</span>
      </span>
    </button>
    <button
      class="tab"
      class:active={activeTab === "privacy"}
      onclick={() => (activeTab = "privacy")}
    >
      <span class="tab-icon">🔒</span>
      <span class="tab-text">
        <span class="tab-title">隐私痕迹粉碎</span>
        <span class="tab-desc">安全删除</span>
      </span>
    </button>
  </nav>

  <section class="content">
    {#if activeTab === "cache"}
      <div class="panel">
        <div class="panel-head">
          <div class="panel-title">
            <h2>缓存扫描</h2>
            <p class="panel-subtitle">一键识别可清理空间</p>
          </div>
          <button class="btn btn-ghost" onclick={handleScan} disabled={isScanning}>
            {isScanning ? "扫描中..." : "重新扫描"}
          </button>
        </div>

        <div class="panel-grid">
          <aside class="summary-card">
            <div class="summary-top">
              <span class="summary-title">系统缓存概览</span>
              <span class="chip">缓存</span>
            </div>
            {#if scanResult}
              <div class="metric">
                <span class="metric-value">
                  {formatSizeLocal(scanResult.total_size)}
                </span>
                <span class="metric-label">可清理空间</span>
              </div>
              <div class="metric">
                <span class="metric-value">{scanResult.total_files}</span>
                <span class="metric-label">文件数量</span>
              </div>
            {:else}
              <div class="empty-compact">尚未扫描</div>
            {/if}
            <div class="summary-actions">
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
            <p class="summary-note">
              默认已全选，可在列表中取消不需要的项目。
            </p>
          </aside>

          <div class="list-card">
            <div class="list-head">
              <div>
                <h3>清理项清单</h3>
                <p>逐项控制你的清理范围</p>
              </div>
              <div class="list-meta">
                {scanResult ? `${scanResult.items.length} 项` : "0 项"}
              </div>
            </div>

            {#if scanResult && scanResult.items.length > 0}
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
          </div>
        </div>
      </div>
    {:else}
      <div class="panel">
        <div class="panel-head">
          <div class="panel-title">
            <h2>隐私痕迹</h2>
            <p class="panel-subtitle">军规级安全粉碎</p>
          </div>
          <button
            class="btn btn-ghost"
            onclick={handleScanPrivacy}
            disabled={isScanning}
          >
            {isScanning ? "扫描中..." : "重新扫描"}
          </button>
        </div>

        <div class="panel-grid">
          <aside class="summary-card summary-danger">
            <div class="summary-top">
              <span class="summary-title">安全粉碎</span>
              <span class="chip danger">不可恢复</span>
            </div>
            <div class="warning-box">
              <span class="warning-icon">⚠️</span>
              <div>
                <p>采用 DoD 5220.22-M 军事级安全删除算法</p>
                <p class="warning-strong">数据将被彻底覆写，无法恢复</p>
              </div>
            </div>
            <div class="summary-actions">
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
            <p class="summary-note">建议清理前关闭相关应用，避免误删。</p>
          </aside>

          <div class="list-card">
            <div class="list-head">
              <div>
                <h3>隐私项目</h3>
                <p>仅粉碎“存在”的项目</p>
              </div>
              <div class="list-meta">{privacyItems.length} 项</div>
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
          </div>
        </div>
      </div>
    {/if}
  </section>

  <footer class="status-bar" aria-live="polite">
    {#if isScanning}
      <span class="spinner"></span>
    {/if}
    <span class="status-text">{statusMessage}</span>
  </footer>
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600&family=Syne:wght@500;700&display=swap");

  :global(:root) {
    --bg: #0a0b10;
    --panel: rgba(15, 18, 28, 0.88);
    --panel-border: rgba(255, 255, 255, 0.08);
    --text: #e7eaf3;
    --muted: #a1a8ba;
    --accent: #22f0c7;
    --accent-strong: #00d6b4;
    --accent-warm: #f7a43b;
    --danger: #ff5d6c;
    --danger-strong: #ff3e54;
    --success: #48e0a6;
    --shadow: 0 24px 60px rgba(0, 0, 0, 0.4);
  }

  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(body) {
    font-family: "IBM Plex Sans", "Noto Sans SC", sans-serif;
    background: radial-gradient(
        1200px 800px at 10% -10%,
        rgba(34, 240, 199, 0.15),
        transparent 60%
      ),
      radial-gradient(
        1000px 600px at 90% 0%,
        rgba(247, 164, 59, 0.12),
        transparent 55%
      ),
      var(--bg);
    color: var(--text);
    min-height: 100vh;
    overflow-x: hidden;
  }

  :global(body)::before {
    content: "";
    position: fixed;
    inset: 0;
    background-image: linear-gradient(
        transparent 95%,
        rgba(255, 255, 255, 0.03) 96%
      ),
      linear-gradient(90deg, transparent 95%, rgba(255, 255, 255, 0.03) 96%);
    background-size: 28px 28px;
    opacity: 0.35;
    pointer-events: none;
    mix-blend-mode: soft-light;
  }

  :global(body)::after {
    content: "";
    position: fixed;
    inset: 0;
    background: radial-gradient(
        circle at 20% 20%,
        rgba(255, 255, 255, 0.06),
        transparent 35%
      ),
      radial-gradient(
        circle at 80% 30%,
        rgba(255, 255, 255, 0.04),
        transparent 40%
      );
    pointer-events: none;
    opacity: 0.6;
  }

  .app {
    max-width: 1120px;
    margin: 0 auto;
    padding: 32px 24px 28px;
    display: flex;
    flex-direction: column;
    gap: 24px;
    min-height: 100vh;
  }

  .hero {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 18px 22px;
    border-radius: 18px;
    background: rgba(15, 18, 28, 0.65);
    border: 1px solid var(--panel-border);
    box-shadow: var(--shadow);
    backdrop-filter: blur(12px);
    position: relative;
    overflow: hidden;
  }

  .hero::after {
    content: "";
    position: absolute;
    right: -80px;
    top: -80px;
    width: 220px;
    height: 220px;
    background: radial-gradient(
      circle,
      rgba(34, 240, 199, 0.22),
      transparent 60%
    );
    opacity: 0.9;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 16px;
    z-index: 1;
  }

  .brand-icon {
    width: 52px;
    height: 52px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.8rem;
    border-radius: 16px;
    background: linear-gradient(
      135deg,
      rgba(34, 240, 199, 0.2),
      rgba(34, 240, 199, 0.02)
    );
    border: 1px solid rgba(34, 240, 199, 0.4);
    box-shadow: inset 0 0 18px rgba(34, 240, 199, 0.2);
  }

  .brand h1 {
    font-family: "Syne", "Noto Sans SC", sans-serif;
    font-size: 2.2rem;
    letter-spacing: 1px;
    margin-bottom: 4px;
  }

  .subtitle {
    color: var(--muted);
    font-size: 0.95rem;
  }

  .hero-actions {
    display: flex;
    align-items: center;
    gap: 12px;
    z-index: 1;
  }

  .status-pill {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    border-radius: 999px;
    background: rgba(10, 12, 20, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.08);
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: #dce0ea;
  }

  .pulse {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent);
    box-shadow: 0 0 12px rgba(34, 240, 199, 0.8);
    animation: pulse 1.6s infinite;
  }

  .pulse.ok {
    background: var(--success);
    box-shadow: 0 0 12px rgba(72, 224, 166, 0.8);
  }

  .pulse.danger {
    background: var(--danger);
    box-shadow: 0 0 12px rgba(255, 93, 108, 0.8);
  }

  .meta-chip {
    padding: 8px 12px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.08);
    font-size: 0.85rem;
    color: var(--muted);
  }

  .tabs {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 16px 18px;
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(12, 14, 22, 0.65);
    color: var(--muted);
    cursor: pointer;
    transition: all 0.25s ease;
  }

  .tab:hover {
    transform: translateY(-1px);
    border-color: rgba(34, 240, 199, 0.35);
    color: var(--text);
  }

  .tab.active {
    background: linear-gradient(
      135deg,
      rgba(34, 240, 199, 0.25),
      rgba(34, 240, 199, 0.02)
    );
    border-color: rgba(34, 240, 199, 0.6);
    color: #e9fff9;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.25);
  }

  .tab-icon {
    font-size: 1.5rem;
  }

  .tab-text {
    display: flex;
    flex-direction: column;
    gap: 4px;
    text-align: left;
  }

  .tab-title {
    font-weight: 600;
    font-size: 1rem;
  }

  .tab-desc {
    font-size: 0.8rem;
    color: var(--muted);
  }

  .tab.active .tab-desc {
    color: rgba(233, 255, 249, 0.8);
  }

  .content {
    flex: 1;
  }

  .panel {
    background: var(--panel);
    border: 1px solid var(--panel-border);
    border-radius: 20px;
    padding: 22px;
    backdrop-filter: blur(14px);
    box-shadow: var(--shadow);
  }

  .panel-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 20px;
  }

  .panel-title h2 {
    font-size: 1.4rem;
    margin-bottom: 4px;
    font-family: "Syne", "Noto Sans SC", sans-serif;
  }

  .panel-subtitle {
    color: var(--muted);
    font-size: 0.9rem;
  }

  .panel-grid {
    display: grid;
    grid-template-columns: minmax(240px, 300px) minmax(0, 1fr);
    gap: 20px;
  }

  .summary-card {
    background: rgba(12, 14, 22, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .summary-danger {
    border-color: rgba(255, 93, 108, 0.35);
    background: rgba(24, 12, 16, 0.7);
  }

  .summary-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .summary-title {
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: var(--muted);
  }

  .chip {
    padding: 4px 10px;
    border-radius: 999px;
    font-size: 0.75rem;
    background: rgba(34, 240, 199, 0.2);
    color: #d3fff5;
    border: 1px solid rgba(34, 240, 199, 0.4);
  }

  .chip.danger {
    background: rgba(255, 93, 108, 0.2);
    border-color: rgba(255, 93, 108, 0.4);
    color: #ffd6db;
  }

  .metric {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .metric-value {
    font-size: 1.8rem;
    font-weight: 600;
    color: var(--accent);
  }

  .metric-label {
    font-size: 0.85rem;
    color: var(--muted);
  }

  .summary-actions {
    margin-top: 4px;
  }

  .summary-note {
    font-size: 0.82rem;
    color: var(--muted);
    line-height: 1.4;
  }

  .list-card {
    background: rgba(10, 12, 20, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 16px;
    padding: 18px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .list-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .list-head h3 {
    font-size: 1.05rem;
    margin-bottom: 6px;
  }

  .list-head p {
    font-size: 0.85rem;
    color: var(--muted);
  }

  .list-meta {
    padding: 6px 10px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.06);
    color: var(--muted);
    font-size: 0.8rem;
  }

  .item-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-height: 340px;
    overflow-y: auto;
    padding-right: 2px;
  }

  .item {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px;
    background: rgba(20, 23, 34, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 12px;
    transition: all 0.2s ease;
  }

  .item:hover {
    border-color: rgba(34, 240, 199, 0.4);
    transform: translateY(-1px);
  }

  .item.selected {
    border-color: rgba(34, 240, 199, 0.6);
    background: rgba(34, 240, 199, 0.08);
  }

  .item.disabled {
    opacity: 0.5;
    filter: grayscale(0.4);
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
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    background: rgba(10, 12, 20, 0.8);
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .item-checkbox input:checked + .checkmark {
    background: linear-gradient(
      135deg,
      rgba(34, 240, 199, 0.9),
      rgba(0, 214, 180, 0.9)
    );
    border-color: rgba(34, 240, 199, 0.9);
  }

  .item-checkbox input:checked + .checkmark::after {
    content: "✓";
    color: #041413;
    font-size: 12px;
    font-weight: 700;
  }

  .item-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .item-name {
    font-weight: 600;
  }

  .item-path {
    font-size: 0.8rem;
    color: var(--muted);
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
    color: var(--accent-warm);
  }

  .item-count {
    font-size: 0.75rem;
    color: var(--muted);
  }

  .item-status {
    min-width: 70px;
    text-align: center;
    font-size: 0.8rem;
  }

  .status-exists {
    color: var(--accent-warm);
  }

  .status-clean {
    color: var(--success);
  }

  .warning-box {
    display: flex;
    gap: 12px;
    padding: 14px;
    border-radius: 12px;
    background: rgba(255, 93, 108, 0.08);
    border: 1px solid rgba(255, 93, 108, 0.3);
  }

  .warning-icon {
    font-size: 1.4rem;
  }

  .warning-box p {
    font-size: 0.85rem;
    color: #ffd5da;
    line-height: 1.4;
  }

  .warning-strong {
    color: #fff;
    font-weight: 600;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px 18px;
    border-radius: 12px;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.2s ease;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  .btn-ghost {
    background: rgba(255, 255, 255, 0.04);
    border-color: rgba(255, 255, 255, 0.08);
    color: var(--text);
  }

  .btn-ghost:hover:not(:disabled) {
    border-color: rgba(34, 240, 199, 0.5);
    color: #e9fff9;
  }

  .btn-primary {
    background: linear-gradient(
      135deg,
      rgba(34, 240, 199, 0.95),
      rgba(0, 214, 180, 0.95)
    );
    color: #041413;
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.35);
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 16px 30px rgba(0, 0, 0, 0.4);
  }

  .btn-danger {
    background: linear-gradient(
      135deg,
      rgba(255, 93, 108, 0.95),
      rgba(255, 62, 84, 0.95)
    );
    color: #120607;
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.35);
  }

  .btn-danger:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 16px 30px rgba(0, 0, 0, 0.4);
  }

  .btn-large {
    width: 100%;
    padding: 14px 18px;
    font-size: 1rem;
    border-radius: 14px;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-top-color: currentColor;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .empty {
    text-align: center;
    padding: 40px 20px;
    color: var(--muted);
    border: 1px dashed rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    background: rgba(10, 12, 20, 0.4);
  }

  .empty-compact {
    color: var(--muted);
    font-size: 0.9rem;
    background: rgba(255, 255, 255, 0.04);
    padding: 10px 12px;
    border-radius: 10px;
    border: 1px dashed rgba(255, 255, 255, 0.1);
  }

  .status-bar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    padding: 12px;
    border-radius: 12px;
    background: rgba(12, 14, 22, 0.7);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: var(--muted);
  }

  .status-text {
    font-size: 0.85rem;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes pulse {
    0% {
      transform: scale(1);
      opacity: 1;
    }
    70% {
      transform: scale(1.3);
      opacity: 0.4;
    }
    100% {
      transform: scale(1);
      opacity: 1;
    }
  }

  .item-list::-webkit-scrollbar {
    width: 8px;
  }

  .item-list::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.04);
    border-radius: 4px;
  }

  .item-list::-webkit-scrollbar-thumb {
    background: rgba(34, 240, 199, 0.4);
    border-radius: 4px;
  }

  .item-list::-webkit-scrollbar-thumb:hover {
    background: rgba(34, 240, 199, 0.7);
  }

  @media (max-width: 900px) {
    .hero {
      flex-direction: column;
      align-items: flex-start;
    }

    .hero-actions {
      width: 100%;
      justify-content: flex-start;
      flex-wrap: wrap;
    }

    .panel-grid {
      grid-template-columns: 1fr;
    }

    .tab {
      padding: 14px 16px;
    }
  }
</style>
