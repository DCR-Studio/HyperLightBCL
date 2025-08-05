# **OpenBCLCore**
#####
**```A Open Source BCL-Launcher Core For Rust```**
#####
[English](README.md)
[繁體中文](README_ZH_TW.md)
#####
**以命令行形式采用 Rust 编写的一个 BCL-Launcher Core 分支**
#####

> [!WARNING]
> * **该项目与采用 Python 编写的 [BCL-Launcher](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 是两个不同的开发分支!**
> * **OpenBCLCore 目前仍处于早期开发阶段，因早期开发阶段中会难以避免地会出现一些 Bug，故此不推荐日用，若发现 Bug，请提交到我们的 Issus，谢谢!**
> * **项目遵循 GPL-3.0 许可证，功能持续添加中**

#####

## **📆 开发进度**
**下面为计划加入功能模块的信息及进度**

#####

### **✓ 已完成功能：**

* [x] **离线登录**

### **🛠️ 计划开发 / 完成功能：**

* [ ] **启动游戏版本**
* [ ] **游戏版本下载和安装功能**
* [ ] **下载并安装 CurseForge 和 Modrinth 模组、资源、存档、光影和整合包**
* [ ] **账户管理系统（Microsoft OAuth 登录及第三方身份验证服务器登录）**
* [ ] **更多功能待添加**

#####

## **📚 如何使用？**

- **通过 Github Actions 构建下载**
- **配置 Rust 开发环境进行编译(面向开发者)**
#####
### > **构建步骤：**

**若您不想通过 Github Actions 下载 OpenBCLCore，且想及时体验新功能，请配置好 Rust 开发环境后输入命令来进行编译：**

```bash
git clone  git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo build --release
```

**在编译完成后，文件存放在 target/debug 文件夹中**

**若您在编译之前想要测试 OpenBCLCore，且您是开发人员，精通 Rust，可通过我提供的命令来进行测试：**

```bash
git clone git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo run
```

#####
## **📖 License**

**OpenBCLCore 遵循 **[GPL-3.0 license](LICENSE)** 开源许可证，这一协议具有传染性**

#####
**©2024-2025 DCR Studio. 保留所有权利**
