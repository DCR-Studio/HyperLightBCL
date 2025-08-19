<img src="./assets/logo.svg" alt="HyperLightBCL Logo" width="150>" align="right">

# **HyperLightBCL**
#####
**```A Faster and More Convenient HyperLightBCL For Rust```**
#####
[English](README.md)
[繁體中文](README_ZH_TW.md)
#####
**一个 BCL-Launcher Core 分支，使用 Rust 打造的一个更快、更稳的一个图形化 GUI 启动器**
#####

> [!WARNING]
> * **该项目与采用 Python 编写的 [BCL-Launcher](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 是两个不同的开发分支!**
> * **HyperLightBCL 目前仍处于早期开发阶段，因早期开发阶段中会难以避免地会出现一些 Bug，故此不推荐日用，若发现 Bug，请提交到我们的 Issues，谢谢!**
> * **项目遵循 GNU General Public License 3.0 许可证，功能持续添加中**

#####

## **📆 开发进度**
**下面为计划加入功能模块的信息及进度**

#####

### **✓ 已完成功能：**

* [x] **离线登录 及 Microsoft OAuth 登录**

### **🛠️ 计划开发 / 完成功能：**

* [ ] **启动游戏版本**
* [ ] **游戏版本下载和安装功能**
* [ ] **下载并安装 CurseForge 和 Modrinth 模组、资源、存档、光影和整合包**
* [ ] **传统 Yggdrasil API 及 OAuth 2.0 Yggdrasil API 登录**
* [ ] **更多功能待添加......**

#####

## **? 如何使用？**

- **通过 Github Actions 构建下载**
- **配置 Rust 开发环境进行编译(面向开发者)**
#####
### > **📦 构建步骤：**

**若您不想通过 Github Actions 下载 HyperLightBCL，且想及时体验新功能，请配置好 Rust 开发环境后输入命令来进行编译：**

```bash
git clone  git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
cargo build --release
```

**在编译完成后，文件存放在 target/release 文件夹中**

**若您在编译之前想要测试 HyperLightBCL，且您是开发人员，精通 Rust，可通过我提供的命令来进行测试：**

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
cargo run
```

#####
## **📖 License**

**我们真诚的希望社区开发者能为我们贡献部分代码，故此，我们选择开源源代码**

**HyperLightBCL 遵循 **[GPL-3.0 license](LICENSE)** 开源许可证，这一协议具有传染性**

#####

### 附加条款 (依据 GPLv3 开源协议第七条)  

1. 当你分发该程序的修改版本时，你必须以合理方式修改该程序的名称或版本号，以示其与原始版本不同。(依据 [GPLv3, 7(c)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L372-L374))
   - 修改版本 **不得在名称中包含原程序名称 “Bad Craft Launcher” “HyperLightBCL” 或其缩写 “BCL”，也不得使用与官方名称相近、可能导致混淆的名称**。
   - 所有修改版本 **必须在程序启动页面或主界面中以明显方式标注其为“非官方修改版”**。

2. 你不得移除该程序所显示的版权声明。(依据 [GPLv3, 7(b)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L368-L370))

#####
**Copyright ©2024-2025 DCR Studio and contributors. All rights reserved**
