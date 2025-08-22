<div align="center">

<a href="https://github.com/DCR-Studio/HyperLightBCL">

<img src="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/images/logo.svg" alt="Logo" width="150" height="150">
</a>

# HyperLightBCL Branch

<b><a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/README.md">English</a> | 简体中文 | <a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/README_zh_hant.md">繁體中文</a>
</p>

[贡献指南[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md)
[技术规范[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md#Angular-commit-convention)

[![Stars](https://img.shields.io/github/stars/DCR-Studio/HyperLightBCL?style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZlcnNpb249IjEiIHdpZHRoPSIxNiIgaGVpZ2h0PSIxNiI+PHBhdGggZD0iTTggLjI1YS43NS43NSAwIDAgMSAuNjczLjQxOGwxLjg4MiAzLjgxNSA0LjIxLjYxMmEuNzUuNzUgMCAwIDEgLjQxNiAxLjI3OWwtMy4wNDYgMi45Ny43MTkgNC4xOTJhLjc1MS43NTEgMCAwIDEtMS4wODguNzkxTDggMTIuMzQ3bC0zLjc2NiAxLjk4YS43NS43NSAwIDAgMS0xLjA4OC0uNzlsLjcyLTQuMTk0TC44MTggNi4zNzRhLjc1Ljc1IDAgMCAxIC40MTYtMS4yOGw0LjIxLS42MTFMNy4zMjcuNjY4QS43NS43NSAwIDAgMSA4IC4yNVoiIGZpbGw9IiNlYWM1NGYiLz48L3N2Zz4=&logoSize=auto&label=Stars&labelColor=444444&color=eac54f)](https://github.com/DCR-Studio/HyperLightBCL)
[![LICENSE](https://img.shields.io/github/license/DCR-Studio/HyperLightBCL?style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE)
![GitHub Release](https://img.shields.io/github/v/release/DCR-Studio/HyperLightBCL?label=Release&logo=github&style=for-the-badge)
[![Build](https://img.shields.io/badge/GitHub%20Actions-Build-181717?logo=github&logoColor=white&style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/actions)

 ~~**```啊卧槽这是苦力怕喵喵呜饼干启动器吗awa```**~~

### 采用 Rust 后端+Tauri 应用框架编写的一个轻量化 Minecraft Java Launcher
#### ~~(虽然但是这是一个处于**早期开发阶段的项目，代码暂未完善**~~)

</div>

---

> [!WARNING]
> * 此项目与 [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) 的 Core 虽采用 **Rust+Python 进行开发**，但仍是**两个不同的开发分支！**
> * HyperLightBCL 目前处于**早期开发阶段**，会难以避免地出现一些 Bug，故此不推荐日用，若你在使用过程中发现 Bug，请**提交到我们的 Issues**，谢谢！
> * 项目遵循 **GNU General Public License v3.0 开源许可证**，仍在开发中......

---

## ❓ 什么是 HyperLightBCL 分支?
**HyperLightBCL** (原名 **OpenBCLCore**) 是由 [**TNTyep520[↗]**](https://github.com/TNTyep520) 从 [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) 分离出来的 **单独 Rust 开发分支，完全采用 Rust 编写**，但仍属于 [**DCR-Studio[↗]**](https://github.com/DCR-Studio) **项目组**

HyperLightBCL 仍然秉持着**轻量化 · 可定制 · 开放 · 以玩家为中心的设计理念**，遵循 **GNU General Public License v3.0 开源许可证**，我们欢迎任何社区开发者为我们贡献源代码！

**开发组：307955001**

---

## 项目所使用的技术栈：
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?style=for-the-badge&logo=tauri&logoColor=white&labelColor=24C8DB)](https://tauri.app/)
[![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)](https://nodejs.org/)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

---

## 🚀 特点与优势：

### 🔒 安全  

- Rust 内存安全性+非对称加密和系统级密钥存储器处理启动器的敏感数据

### 💻 跨平台 

- 跨平台支持 ( Windows / Linux / macOS )

### 🧩 灵活扩展  

- 丰富的第三方库生态，开发者可快速扩展功能  
- 开发者可根据需求独立修改或新增组件，而无需大幅度改动整个项目

---

## 📆 开发进度：
以下是**加入功能模块的开发计划及进度：**

###  ✅ 已完成功能：

* [x] 离线登录

### 🛠️ 计划开发 / 完成功能：

* [ ] 启动游戏版本
* [ ] 游戏版本下载和安装功能
* [ ] 下载、安装 CurseForge 和 Modrinth 模组、资源、存档、光影和整合包
* [ ] Microsoft OAuth 2.0 与传统 Yggdrasil API 及 OAuth 2.0 Yggdrasil API 登录
* [ ] 内容管理器：存档、资源包、光影包、整合包、模组
* [ ] Java 管理：JVM 自定义参数、Java 自动检测与下载
* [ ] 下载源提供：BMCLAPI、官方下载源
* [ ] 更多功能待添加......

---

## 📦 安装与使用：

### 下载：

> [!IMPORTANT]  
> ❌ 目前项目处于**早期开发阶段**，暂不放出Release，但你仍可以通过 **GitHub Actions 链接**，下载由最新代码自动构建的预览版进行体验 (可能**需要登录 Github 账户**)：
>
> - [**Github Release Latest[↗]**](https://github.com/DCR-Studio/HyperLightBCL/releases/latest)
>
> - [**Github Actions[↗]**](https://github.com/DCR-Studio/HyperLightBCL/actions)

通常只需访问 [**启动器官网[↗]**](https://launcher.dcrstudio.top/download) 下载即可

---

### 编译与调试 (面向开发者)：

> 此内容仅适用于**参与开发**的用户：

### 环境要求：

> * 一台 4GB 内存以上的计算机 **(仅 x86_64 架构，不限操作系统)**
>
> * 含 **Rust 、Node.js 开发环境、Tauri 应用框架支持库**的计算机

### 构建步骤：

首先克隆本项目并安装**前端依赖**：

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
npm install
```

以**调试模式**运行：

```bash
npm run tauri dev
```

我们热烈欢迎**每一位社区开发者对该项目的贡献！**

### 提交仓库源代码 (另见[贡献指南[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md)) ：

**登录你的个人 Github 账户，fork 此仓库，使用 git 克隆你 fork 的仓库地址，自修改后通过 git commit 提交源代码，然后申请合并 Pull requests**

---

## 🌟 鸣谢人员：
**感谢以下为项目做出贡献的人员：**

[![Contributors](https://contrib.rocks/image?repo=DCR-Studio/HyperLightBCL)](https://github.com/DCR-Studio/HyperLightBCL/graphs/contributors)

---

## 贡献与反馈：

如果您在使用过程中发现任何问题或有改进建议，欢迎通过仓库的 Issues 页面提交反馈。我们非常感谢您的贡献，并将不断完善代码及功能

---

## 📜 License

我们真诚的希望社区开发者能为我们贡献部分代码，故此，我们选择**完全开源源代码**

**HyperLightBCL** 遵循 **[GNU General Public License v3.0[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE) 开源许可证**，这一协议具有互相**传染性**

---

### 附加条款 (依据 GNU General Public License v3.0 开源许可证第七条)  

1. 当你分发该程序的修改版本时，你必须以**合理方式修改该程序的名称或版本号**，以示其与原始版本不同。(依据 [**GNU General Public License v3.0, 7(c)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L372-L374))
   - 修改版本 **不得在名称中包含原程序名称 “Bad Craft Launcher” “HyperLightBCL” 或其缩写 “BCL”，也不得使用与官方名称相近、可能导致混淆的名称**。
   - 所有修改版本 **必须在程序启动页面或主界面中以明显方式标注其为“非官方修改版”**。
   - 所有修改版本 **必须明确在其仓库、README、程序等说明基于“Bad Craft Launcher” 或 “HyperLightBCL”修改**。

2. 你**不得移除该程序所显示的版权声明**。(依据 [**GNU General Public License v3.0, 7(b)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L368-L370))

**Copyright ©2024-2025 DCR Studio and contributors. All rights reserved**
