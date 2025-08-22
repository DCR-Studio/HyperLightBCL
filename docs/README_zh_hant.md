<div align="center">

<a href="https://github.com/DCR-Studio/HyperLightBCL">

<img src="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/images/logo.svg" alt="Logo" width="150" height="150">
</a>

# HyperLightBCL Branch

  <b><a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/README.md">English</a> | <a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/README_zh_hans.md">简体中文</a> | 繁體中文
</p>

[貢獻指南[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md)
[技術規範[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md#Angular-commit-convention)

[![Stars](https://img.shields.io/github/stars/DCR-Studio/HyperLightBCL?style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZlcnNpb249IjEiIHdpZHRoPSIxNiIgaGVpZ2h0PSIxNiI+PHBhdGggZD0iTTggLjI1YS43NS43NSAwIDAgMSAuNjczLjQxOGwxLjg4MiAzLjgxNSA0LjIxLjYxMmEuNzUuNzUgMCAwIDEgLjQxNiAxLjI3OWwtMy4wNDYgMi45Ny43MTkgNC4xOTJhLjc1MS43NTEgMCAwIDEtMS4wODguNzkxTDggMTIuMzQ3bC0zLjc2NiAxLjk4YS43NS43NSAwIDAgMS0xLjA4OC0uNzlsLjcyLTQuMTk0TC44MTggNi4zNzRhLjc1Ljc1IDAgMCAxIC40MTYtMS4yOGw0LjIxLS42MTFMNy4zMjcuNjY4QS43NS43NSAwIDAgMSA4IC4yNVoiIGZpbGw9IiNlYWM1NGYiLz48L3N2Zz4=&logoSize=auto&label=Stars&labelColor=444444&color=eac54f)](https://github.com/DCR-Studio/HyperLightBCL)
[![LICENSE](https://img.shields.io/github/license/DCR-Studio/HyperLightBCL?style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE)
![GitHub Release](https://img.shields.io/github/v/release/DCR-Studio/HyperLightBCL?label=Release&logo=github&style=for-the-badge)
[![Build](https://img.shields.io/badge/GitHub%20Actions-Build-181717?logo=github&logoColor=white&style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/actions)

 ~~**```啊臥槽這是苦力怕喵喵嗚餅乾啟動器嗎awa```**~~

### 採用 Rust 後端+Tauri 應用框架編寫的一個輕量化 Minecraft Java Launcher
#### ~~(雖然但是這是一個處於**早期開發階段的項目，代碼暫未完善**~~)

</div>

---

> [!WARNING]
> * 此項目與 [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 雖採用 **Rust+Python 進行開發**，但仍是**兩個不同的開發分支！**
> * HyperLightBCL 目前處於**早期開發階段**，會難以避免地出現一些 Bug，故此不推薦日用，若你在使用過程中發現 Bug，請**提交到我們的 Issues**，謝謝！
> * 項目遵循 **GNU General Public License v3.0 開源許可證**，仍在開發中......

---

## ❓ 什麼是 HyperLightBCL 分支?
**HyperLightBCL** (原名 **OpenBCLCore**) 是由 [**TNTyep520[↗]**](https://github.com/TNTyep520) 從 [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) 分離出來的 **單獨 Rust 開發分支，完全採用 Rust 編寫**，但仍屬於 [**DCR-Studio[↗]**](https://github.com/DCR-Studio) **項目組**

HyperLightBCL 仍然保持著**輕量化 · 可定製 · 開放 · 以玩家為中心的設計理念**，遵循 **GNU General Public License v3.0 開源許可證**，我們歡迎任何社區開發者為我們貢獻源代碼！

**開發組：307955001**

---

## 項目所使用的技術棧：
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?style=for-the-badge&logo=tauri&logoColor=white&labelColor=24C8DB)](https://tauri.app/)
[![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)](https://nodejs.org/)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

---

## 🚀 特點與優勢：

### 🔒 安全  

- Rust 內存安全性+非對稱和系統級密鑰存儲器處理啟動器的敏感數據

### 💻 跨平台

- 跨平台支持 ( Windows / Linux / macOS )

### 🧩 靈活擴展

- 豐富的第三方庫生態，開發者可快速擴展功能
- 開發者可根據需求獨立修改或或新增組件，而無需大幅度改動整個項目

---

## 📆 開發進度：
以下是**加入功能模塊的開發計劃及進度：**

###  ✅ 已完成功能：

* [x] 離線登錄

### 🛠️ 計劃開發 / 完成功能：

* [ ] 啟動遊戲版本
* [ ] 遊戲版本下載和安裝功能
* [ ] 下載、安裝 CurseForge 和 Modrinth 模組、資源、存檔、光影和整合包
* [ ] Microsoft OAuth 2.0 與傳統 Yggdrasil API 及 OAuth 2.0 Yggdrasil API 登錄
* [ ] 內容管理器：存檔、資源包、光影包、整合包、模組
* [ ] Java 管理：JVM 自定義參數、Java 自動檢測與下載
* [ ] 下載源提供：BMCLAPI、官方下載源
* [ ] 更多功能待添加......

---

## 📦 安裝與使用：

### 下載：

> [!IMPORTANT]  
> ❌ 目前項目處於**早期開發階段**，暫不放出Release，但你仍可以通過 **GitHub Actions 鏈接**，下載由最新代碼自動構建的預覽版進行體驗 (可能**需要登錄 Github 賬戶**)：
>
> - [**Github Release Latest[↗]**](https://github.com/DCR-Studio/HyperLightBCL/releases/latest)
>
> - [**Github Actions[↗]**](https://github.com/DCR-Studio/HyperLightBCL/actions)

通常只需要訪問 [**啟動器官網[↗]**](https://launcher.dcrstudio.top/download) 下載即可

---

### 编译与调试 (面向開發者)：

> 此內容僅適用於**參與開發**的用戶：

### 環境要求：

> * 一台 4GB 內存以上的計算機 **(僅 x86_64 架構，不限操作系統)**
>
> * 含 **Rust 、Node.js 開發環境、Tauri 應用框架支持庫**的計算機

### 構建步驟：

首先克隆此項目並安裝**前端依賴**：

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
npm install
```

以**調試模式**運行：

```bash
npm run tauri dev
```

我們熱烈歡迎**每一位社區開發者對該項目的貢獻！**

### 提交倉庫源代碼 (另見[貢獻指南[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md)) ：

**登錄你的個人 Github 賬戶，fork 此倉庫，使用 git 克隆你 fork 的倉庫地址，自修改後通過 git commit 提提交源代碼，然後申請合併 Pull requests**

---

## 🌟 鳴謝人員：
**感謝以下為項目做出貢獻的人員：**

[![Contributors](https://contrib.rocks/image?repo=DCR-Studio/HyperLightBCL)](https://github.com/DCR-Studio/HyperLightBCL/graphs/contributors)

---

## 貢獻與反饋：

如果你在使用過程中發現任何問題或有改進建議，歡迎你通過倉庫的 Issues 頁面提交反饋。我們非常感謝你的貢獻，並將不斷完善代碼與功能

---

## 📜 License

我們真誠的希望社區開發者能為我們貢獻部分代碼，故此，我們選擇**完全開源源代碼**

**HyperLightBCL** 遵循 **[GNU General Public License v3.0[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE) 開源許可證**，這一協議具有互相**傳染性**

---

### 附加條款 (依據 GNU General Public License v3.0 開源許可證第七條)  

1. 當你分發該程序的修改版本時，你必須以**合理方式修改該程序的名稱或版本號**，以示其與原始版本的不同。(依據 [**GNU General Public License v3.0, 7(c)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L372-L374))
   - 修改版本 **不得在名稱中包含源程序名稱 “Bad Craft Launcher” “HyperLightBCL” 或其簡寫 “BCL”，也不得使用與官方名稱相近、可能導致混淆的名稱**。
   - 所有修改版本 **必須在程序啟動頁面或主頁面中以明顯形式標註其為“非官方修改版”**。
   - 所有修改版本 **必须明確在其倉庫、README、程序等說明基於“Bad Craft Launcher” 或 “HyperLightBCL”修改**。

2. 你**不得移除該程序所顯示的版權聲明**。(依據 [**GNU General Public License v3.0, 7(b)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L368-L370))

**Copyright ©2024-2025 DCR Studio and contributors. All rights reserved**
