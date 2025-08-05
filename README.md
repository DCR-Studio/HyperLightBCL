# **OpenBCLCore**
#####
**```A Open Source BCL-Launcher Core for Rust```**
#####
[简体中文](README_ZH_CN.md)
[繁體中文](README_ZH_TW.md)
#####
**A branch of BCL-Launcher Core written in Rust for command-line use**
#####

> [!WARNING]
> * **This project is a separate development branch from the Python-written [BCL-Launcher](https://github.com/DCR-Studio/BCL-Launcher) Core!**
> * **OpenBCLCore is still in early development. Bugs are inevitable in this early stage, so daily use is not recommended. If you find a bug, please report it to our Issues. Thank you!**
> * **The project is licensed under the GPL-3.0 license, and features are being continuously added**

#####

## **📆  Development Progress**
**The following is information and progress on planned feature additions**

#####

### **✓ Completed Features:**

* [x] **Offline Login**

### **🛠️ Planned / Completed Features:**

* [ ] **Launch the game**
* [ ] **Game version download and installation**
* [ ] **Download and install CurseForge and Modrinth mods, resources, save files, shaders, and modpacks**
* [ ] **Account Management System (Microsoft OAuth login and third-party authentication server login)**
* [ ] **More features to be added**

#####

## **📚 How to use?**

- **Build and Download via Github Actions**
- **Configure the Rust Development Environment for Compilation (for Developers)**
#####
### > **Building Steps:**

**If you don't want to download OpenBCLCore via Github Actions and want to experience new features immediately, please configure your Rust development environment and run the following command to compile:**

```bash
git clone git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo build --release
```

**After compilation, the files will be stored in the target/debug folder**

**If you want to test OpenBCLCore before compiling, and you're a developer proficient in Rust, you can use the following command:**

```bash
git clone git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo run
```

#####
## **📖 License**

**OpenBCLCore is licensed under **[GPL-3.0  license](LICENSE)** Open source license, this agreement is contagious**

#####
**©2024-2025 DCR Studio. All rights reserved**
