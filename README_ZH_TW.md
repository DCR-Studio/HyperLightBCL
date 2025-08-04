# **OpenBCLCore**
#####
**```A Open Source BCL-Launcher Core For Rust```**
#####
[English](README.md)
[简体中文](README_ZH_CN.md)
#####
**以命令列形式採用 Rust 編寫的一個 BCL-Launcher Core 分支**
#####

> [!WARNING]
> * **該專案與採用 Python 編寫的 [BCL-Launcher](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 是兩個不同的開發分支!**
> * **OpenBCLCore 目前仍處於早期開發階段，因早期開發階段中會難以避免地會出現一些 Bug，故此不推薦日用，若發現 Bug，請提交到我們的 Issus，謝謝!**
> * **專案遵循 GPL-3.0 許可證，功能持續新增**

#####

## **📆 開發進度**
**以下為計畫加入功能模組的資訊及進度**

#####

### **✓ 已完成功能：**

**暫無完成進度**

### **🛠️ 計畫開發 / 完成功能：**

* [ ] **啟動遊戲版本**
* [ ] **遊戲版本下載與安裝功能**
* [ ] **下載並安裝 CurseForge 和 Modrinth 模組、資源、檔案、光影和整合套件**
* [ ] **帳號登入系統（Microsoft OAuth 登入、離線登入及第三方驗證伺服器登入）**
* [ ] **更多功能待添加**

#####

## **📚 如何使用？**

- **透過 Github Actions 建置下載**
- **配置 Rust 開發環境進行編譯(面向開發者)**
#####
### > **建置步驟：**

**若您不想透過 Github Actions 下載 OpenBCLCore，且想及時體驗新功能，請設定 Rust 開發環境後輸入指令來進行編譯：**

```bash
git clone git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo build
```

**在編譯完成後，檔案存放在 target/debug 資料夾中**

**若您在編譯之前想要測試 OpenBCLCore，且您是開發人員，精通 Rust，可透過我提供的命令來進行測試：**

```bash
git clone git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo run
```

#####
## **📖 License**

**OpenBCLCore 遵循 **[GPL-3.0 license](LICENSE)** 開源許可證，這一協議具有傳染性**

#####
**©2024-2025 DCR Studio. 保留所有權利**