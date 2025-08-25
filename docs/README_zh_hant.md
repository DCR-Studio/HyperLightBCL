<div align="center">

<a href="https://github.com/DCR-Studio/HyperLightBCL">

<img src="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/images/logo.svg" alt="Logo" width="150" height="150">
</a>

# HyperLightBCL Branch

<b><a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/README.md">English</a> | <a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/README_zh_hans.md">简体中文</a> | 繁體中文
</p>

[貢獻指南[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING_zh_hant.md)
[行為準則[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CODE_OF_CONDUCT_zh_hant.md)

[![Stars](https://img.shields.io/github/stars/DCR-Studio/HyperLightBCL?style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZlcnNpb249IjEiIHdp[...]
[![LICENSE](https://img.shields.io/github/license/DCR-Studio/HyperLightBCL?style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE)
![GitHub Release](https://img.shields.io/github/v/release/DCR-Studio/HyperLightBCL?label=Release&logo=github&style=for-the-badge)
[![Build](https://img.shields.io/badge/GitHub%20Actions-Build-181717?logo=github&logoColor=white&style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/actions)

 ~~**```啊卧槽这是苦力怕喵喵呜饼干启动器吗awa```**~~ 

### 採用 Rust 後端 + Tauri 應用框架編寫的一個輕量化 Minecraft Java 啟動器
#### ~~(雖然但是這是一個處於**早期開發階段的專案，程式碼尚未完善**~~)

</div>

---

> [!WARNING]
> * 此專案與 [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 雖採用 **Rust+Python 開發**，但仍為**兩個不同的開發分支！**
> * HyperLightBCL 目前處於**早期開發階段**，可能會出現一些 Bug，故不建議作為日常使用的穩定啟動器；若您使用時發現 Bug，請**將其提交到我們的 Issues**，謝謝。
> * 專案遵循 **GNU General Public License v3.0 開源授權**，仍在開發中......

---

## ❓ 什麼是 HyperLightBCL 分支?
**HyperLightBCL**（原名 **OpenBCLCore**）是由 [**TNTyep520[↗]**](https://github.com/TNTyep520) 從 [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) 分離出來的單一分支，旨在提供更輕量、可擴充的啟動器核心。

HyperLightBCL 仍秉持 **輕量化 · 可定制 · 開放 · 以玩家為中心的設計理念**，遵循 **GNU General Public License v3.0 開源授權**，歡迎任何社群貢獻與協作。

**開發組：307955001**

---

## 專案所使用的技術棧：
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?style=for-the-badge&logo=tauri&logoColor=white&labelColor=24C8DB)](https://tauri.app/)
[![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)](https://nodejs.org/)
[![Vue](https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

---

## 🚀 特點與優勢：

### 🔒 安全

- Rust 的記憶體安全性 + 非對稱加密與系統級鑰匙存放器處理啟動器的敏感資料

### 💻 跨平台

- 跨平台支援 (Windows / Linux / macOS)

### 🧩 靈活擴展

- 豐富的第三方函式庫生態，開發者可快速擴展功能
- 開發者可根據需求獨立修改或新增元件，而無需大幅改動整個專案

---

## 📆 開發進度：
以下為**功能模組加入的開發計畫及進度：**

### ✅ 已完成功能：

* [x] 離線登入

### 🛠️ 計畫開發 / 尚未完成的功能：

* [ ] 啟動遊戲版本
* [ ] 遊戲版本下載與安裝功能
* [ ] 下載、安裝 CurseForge 與 Modrinth 的模組、資源、存檔、光影與整合包
* [ ] Microsoft OAuth 2.0 與傳統 Yggdrasil API 及 OAuth 2.0 Yggdrasil API 登入
* [ ] 內容管理器：存檔、資源包、光影包、整合包、模組
* [ ] Java 管理：JVM 自訂參數、Java 自動偵測與下載
* [ ] 下載來源提供：BMCLAPI、官方下載源
* [ ] 更多功能待新增......

---

## 📦 安裝與使用：

### 下載：

> [!IMPORTANT]  
> ❌ 目前專案處於**早期開發階段**，暫未釋出穩定 Release，但您仍可透過 **GitHub Actions** 的自動建置下載預覽版（可能**需留意平台相容性或構建錯誤**）。
>
> - [**Github Release Latest[↗]**](https://github.com/DCR-Studio/HyperLightBCL/releases/latest)
>
> - [**Github Actions[↗]**](https://github.com/DCR-Studio/HyperLightBCL/actions)

通常只需造訪 [**啟動器官網[↗]**](https://launcher.dcrstudio.top/hyperlightbcl/download) 下載即可

---

### 編譯與除錯（面向開發者）：

> 此段僅適用於**參與開發**的使用者：

### 環境需求：

> * 一台 4GB 記憶體以上的電腦 **(僅限 x86_64 架構，不限作業系統)**
>
> * 需具備 **Rust、Node.js 開發環境 與 Tauri 應用框架相關套件**

### 建構步驟：

首先 fork 並 clone 本專案，然後安裝前端相依套件：

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
npm install
```

以**除錯模式**執行：

```bash
npm run tauri dev
```

我們熱烈歡迎**每一位社群開發者對本專案的貢獻！**

### 提交倉庫原始碼（另見[貢獻指南[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING_zh_hant.md)）：

請使用個人 GitHub 帳號 fork 本倉庫，clone 您的 fork，修改後以 git commit 提交原始碼，最後發起 Pull Request 申請合併。

---

## 🌟 感謝名單：
**感謝以下為專案做出貢獻的人員：**

[![Contributors](https://contrib.rocks/image?repo=DCR-Studio/HyperLightBCL&v=12345)](https://github.com/DCR-Studio/HyperLightBCL/graphs/contributors)

---

## 貢獻與回饋：

如果您在使用過程中發現任何問題或有改進建議，歡迎透過倉庫的 Issues 頁面提交回饋。我們非常感謝您的貢獻，並會持續完善程式碼與功能。

---

## 📜 授權條款

我們誠摯希望社群開發者能為本專案貢獻程式碼，因此採用**完全開源**策略。

**HyperLightBCL** 遵循 **[GNU General Public License v3.0[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE)** 開源授權，該授權具有互相傳染性（copyleft）特性。

---

### 附加條款（依據 GNU General Public License v3.0 第七條）
1. 當您散佈本程式的修改版本時，必須以**合理方式修改該程式的名稱或版本號**，以示其與原始版本不同。（依據 [**GNU GPL v3.0, 7(c)[↗]**）
   - 修改版本 **不得在名稱中包含原程式名稱 “Bad Craft Launcher” “HyperLightBCL” 或其縮寫 “BCL”，也不得使用與官方名稱相近、可能導致混淆的名稱。**
   - 所有修改版本 **必須在程式啟動頁面或主介面中以明顯方式標示其為「非官方修改版」。**
   - 所有修改版本 **必須在其倉庫、README、程式等說明中明確標示基於 “Bad Craft Launcher” 或 “HyperLightBCL” 進行修改。**

2. 您**不得移除本程式所顯示的版權聲明**。（依據 [**GNU GPL v3.0, 7(b)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L368-L370)）

**Copyright ©2024-2025 DCR Studio and contributors. All rights reserved**
