<img src="./assets/logo.svg" alt="HyperLightBCL Logo" width="150>" align="right">

# **HyperLightBCL**
#####
**```A Faster and More Convenient HyperLightBCL For Rust```**
#####
[English](README.md)
[简体中文](README_ZH_CN.md)
#####
**一個 BCL-Launcher Core 分支，使用 Rust 打造的一個更快、更穩的一個圖形化 GUI 啟動器**
#####

> [!WARNING]
> * **該專案與採用 Python 編寫的 [BCL-Launcher](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 是兩個不同的開發分支!**
> * **HyperLightBCL 目前仍處於早期開發階段，因早期開發階段中會難以避免地會出現一些 Bug，故此不推薦日用，若發現 Bug，請提交到我們的 Issues，謝謝!**
> * **專案遵循 GNU General Public License 3.0 許可證，功能持續添加中**

#####

## **📆 開發進度**
**以下為計畫加入功能模組的資訊及進度**

#####

### **✓ 已完成功能：**

* [x] **離線登入 及 Microsoft OAuth 登入**

### **🛠️ 計畫開發 / 完成功能：**

* [ ] **啟動遊戲版本**
* [ ] **遊戲版本下載與安裝功能**
* [ ] **下載並安裝 CurseForge 和 Modrinth 模組、資源、檔案、光影和整合套件**
* [ ] **傳統 Yggdrasil API 及 OAuth 2.0 Yggdrasil API 登入**
* [ ] **更多功能待添加......**

#####

## **? 如何使用？  **

- **透過 Github Actions 建置下載**
- **配置 Rust 開發環境進行編譯(面向開發者)**
#####
### > **📦 建置步驟：**

**若您不想透過 Github Actions 下載 HyperLightBCL，且想及時體驗新功能，請設定好 Rust 開發環境後輸入指令來進行編譯：**

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
cargo build --release
```

**編譯完成後，檔案存放在 target/release 資料夾中**

**若您在編譯之前想要測試 HyperLightBCL，且您是開發人員，精通 Rust，可透過我提供的命令來進行測試：**

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
cargo run
```

#####
## **📖 License**

**我們真誠的希望社群開發者能為我們貢獻部分程式碼，故此，我們選擇開源程式碼**

**HyperLightBCL 遵循 **[GPL-3.0 license](LICENSE)** 開源許可證，此協議具有傳染性**

#####

### 附加條款 (依據 GPLv3 開源協定第七條)

1. 當你散佈程式的修改版本時，你必須以合理方式修改程式的名稱或版本號，以示其與原始版本不同。  (依據 [GPLv3, 7(c)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L372-L374))
    - 修改版本 **不得在名稱中包含原始程式名稱 “Bad Craft Launcher” “HyperLightBCL” 或其縮寫 “BCL”，也不得使用與官方名稱相近、可能導致混淆的名稱**。
    - 所有修改版本 **必須在程式啟動頁面或主介面中以明顯方式標註其為「非官方修改版」**。

2. 你不得移除該程式所顯示的版權聲明。  (依據 [GPLv3, 7(b)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L368-L370))

#####
**Copyright ©2024-2025 DCR Studio and contributors. All rights reserved**
