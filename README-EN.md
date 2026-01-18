[简体中文](README.md)

# ⚡ Geek Clearing

A modern Windows desktop cleaning tool providing intelligent cache cleaning and privacy trace shredding.

![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)
![SvelteKit](https://img.shields.io/badge/SvelteKit-2.0-orange?logo=svelte)
![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)

## ✨ Features

### 💾 Smart Cache Cleaning
- **System Temp Files** - `%TEMP%`, `C:\Windows\Temp`
- **Windows Update Cache** - Free up space used by system updates
- **Browser Cache** - Chrome, Edge browser cache
- **Thumbnail Cache** - Windows Explorer thumbnails
- **One-Click Scan & Clean** - Quickly free up storage space

### 🔒 Privacy Trace Shredding
- **DoD 5220.22-M** - Military-grade secure deletion algorithm (3 passes)
- **Browser History** - Chrome, Edge browsing history
- **Recent Documents** - Recent file access records
- **Search History** - Windows search history
- **Clipboard History** - Clipboard cache data

## 🚀 Quick Start

### Prerequisites
- Node.js 18+
- pnpm
- Rust 1.70+

### Install Dependencies

```bash
pnpm install
```

### Development Mode

```bash
pnpm tauri dev
```

### Build for Production

```bash
pnpm tauri build
```

## 🏗️ Project Structure

```
Geek Clearing/
├── src/                      # SvelteKit Frontend
│   ├── lib/
│   │   └── api.ts           # Tauri API Wrapper
│   └── routes/
│       └── +page.svelte     # Main Interface
├── src-tauri/               # Rust Backend
│   └── src/
│       ├── lib.rs          # Main Entry & Command Registration
│       ├── scanner.rs      # Cache Scanning Module
│       ├── shredder.rs     # Secure Deletion Module (DoD 5220.22-M)
│       └── privacy.rs      # Privacy Cleaning Module
└── package.json
```

## 🔐 Secure Deletion Algorithm

Implemented using the **DoD 5220.22-M** standard for secure deletion:

| Pass | Operation | Description |
|------|-----------|-------------|
| 1 | Overwrite `0x00` | All zeros |
| 2 | Overwrite `0xFF` | All ones |
| 3 | Random Data | Random byte overwrite |

Files are deleted after three overwrites to ensure data cannot be recovered.

## 📄 License

MIT License
