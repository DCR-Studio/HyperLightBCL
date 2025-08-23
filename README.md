<div align="center">

<a href="https://github.com/DCR-Studio/HyperLightBCL">

<img src="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/images/logo.svg" alt="Logo" width="150" height="150">
</a>

# HyperLightBCL Branch

<b>English | <a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/README_zh_hans.md">简体中文</a> | <a href="https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/README_zh_hant.md">繁體中文</a>
</p>

[Contribution Guide[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md)
[Code Of Conduct[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CODE_OF_CONDUCT.md)

[![Stars](https://img.shields.io/github/stars/DCR-Studio/HyperLightBCL?style=for-the-badge&logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZlcnNpb249IjEiIHdpZHRoPSIxNiIgaGVpZ2h0PSIxNiI+PHBhdGggZD0iTTggLjI1YS43NS43NSAwIDAgMSAuNjczLjQxOGwxLjg4MiAzLjgxNSA0LjIxLjYxMmEuNzUuNzUgMCAwIDEgLjQxNiAxLjI3OWwtMy4wNDYgMi45Ny43MTkgNC4xOTJhLjc1MS43NTEgMCAwIDEtMS4wODguNzkxTDggMTIuMzQ3bC0zLjc2NiAxLjk4YS43NS43NSAwIDAgMS0xLjA4OC0uNzlsLjcyLTQuMTk0TC44MTggNi4zNzRhLjc1Ljc1IDAgMCAxIC40MTYtMS4yOGw0LjIxLS42MTFMNy4zMjcuNjY4QS43NS43NSAwIDAgMSA4IC4yNVoiIGZpbGw9IiNlYWM1NGYiLz48L3N2Zz4=&logoSize=auto&label=Stars&labelColor=444444&color=eac54f)](https://github.com/DCR-Studio/HyperLightBCL)
[![LICENSE](https://img.shields.io/github/license/DCR-Studio/HyperLightBCL?style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE)
![GitHub Release](https://img.shields.io/github/v/release/DCR-Studio/HyperLightBCL?label=Release&logo=github&style=for-the-badge)
[![Build](https://img.shields.io/badge/GitHub%20Actions-Build-181717?logo=github&logoColor=white&style=for-the-badge)](https://github.com/DCR-Studio/HyperLightBCL/actions)

 ~~**```Oh wow is this a Creeper meow meow cookie launcher awa```**~~

### A lightweight Minecraft Java Launcher written with a Rust backend + Tauri application framework
#### ~~(Although this is a project in an **early development stage and the code is not yet complete**~~)

</div>

---

> [!WARNING]
> * This project and the Core of [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher) are both developed using **Rust+Python**, but they are still **two different development branches!**
> * HyperLightBCL is currently in **early development**, so some bugs are unavoidable. It is not recommended for daily use. If you find any bugs while using it, please **submit them to our Issues**, thank you.
> * The project follows the **GNU General Public License v3.0** and is still under development...

---

## ❓ What is the HyperLightBCL Branch?
**HyperLightBCL** (formerly **OpenBCLCore**) is a standalone core branch forked by [**TNTyep520[↗]**](https://github.com/TNTyep520) from [**BCL-Launcher[↗]**](https://github.com/DCR-Studio/BCL-Launcher).

HyperLightBCL continues to uphold the design principles of **lightweight · customizable · open · player-centered**, and follows the **GNU General Public License v3.0**. We welcome any community development contributions.

**Development Group: 307955001**

---

## Tech stack used by the project:
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white&style=for-the-badge)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?style=for-the-badge&logo=tauri&logoColor=white&labelColor=24C8DB)](https://tauri.app/)
[![Node.js](https://img.shields.io/badge/Node.js-339933?style=for-the-badge&logo=nodedotjs&logoColor=white)](https://nodejs.org/)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

---

## 🚀 Features & Advantages:

### 🔒 Security

- Rust memory safety + asymmetric encryption and system-level key stores handle the launcher's sensitive data

### 💻 Cross-platform

- Cross-platform support ( Windows / Linux / macOS )

### 🧩 Extensible

- Rich third-party library ecosystem, allowing developers to quickly extend functionality
- Developers can independently modify or add components as needed without major changes to the entire project

---

## 📆 Development Progress:
Below are the planned feature modules and their progress:

### ✅ Completed features:

* [x] Offline login

### 🛠️ Planned / Upcoming features:

* [ ] Launch game versions
* [ ] Game version download and installation
* [ ] Download and install CurseForge and Modrinth mods, resource packs, worlds, shaders and modpacks
* [ ] Microsoft OAuth 2.0 and traditional Yggdrasil API and OAuth 2.0 Yggdrasil API logins
* [ ] Content manager: worlds, resource packs, shader packs, modpacks, mods
* [ ] Java management: custom JVM parameters, automatic Java detection and download
* [ ] Download sources: BMCLAPI, official download sources
* [ ] More features to be added......

---

## 📦 Installation & Usage:

### Download:

> [!IMPORTANT]  
> ❌ The project is currently in an **early development stage** and releases are not yet provided. However, you can still download preview builds automatically generated from the latest code via **GitHub Actions** for testing (this may require trusting the build or adjusting system settings).
>
> - [**Github Release Latest[↗]**](https://github.com/DCR-Studio/HyperLightBCL/releases/latest)
>
> - [**Github Actions[↗]**](https://github.com/DCR-Studio/HyperLightBCL/actions)

You can usually just visit the [**Launcher Official Website[↗]**](https://launcher.dcrstudio.top/download) to download

---

### Build & Debug (for developers):

> This section is intended only for users who are **participating in development**:

### Environment requirements:

> * A computer with 4GB or more of RAM **(x86_64 architecture only, any operating system)**
>
> * A computer with **Rust, Node.js development environment, and Tauri application framework support libraries**

### Build steps:

First clone this project and install the **frontend dependencies**:

```bash
git clone git@github.com:DCR-Studio/HyperLightBCL.git
cd HyperLightBCL
npm install
```

Run in **debug mode**:

```bash
npm run tauri dev
```

We warmly welcome **every community developer to contribute to this project!**

### Submitting source code to the repository (see also [Contribution Guide[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/docs/CONTRIBUTING.md)):

**Log in with your personal GitHub account, fork this repository, clone your fork using git, make changes, commit the source code with git commit, then create a Pull Request to submit your changes.**

---

## 🌟 Acknowledgements:
**Thanks to the following people who have contributed to the project:**

[![Contributors](https://contrib.rocks/image?repo=DCR-Studio/HyperLightBCL)](https://github.com/DCR-Studio/HyperLightBCL/graphs/contributors)

---

## Contribution & Feedback:

If you encounter any problems or have suggestions for improvement while using the project, please submit feedback via the repository's Issues page. We greatly appreciate your contributions and will continue to improve the code and features.

---

## 📜 License

We sincerely hope community developers will contribute code to us, therefore we have chosen to make the source code fully open source.

**HyperLightBCL** is licensed under the **[GNU General Public License v3.0[↗]](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE)**, which is a reciprocal (copyleft) license.

---

### Additional Terms (according to Section 7 of the GNU General Public License v3.0)

1. When you distribute a modified version of this program, you must reasonably modify the program's name or version number to indicate that it is different from the original. (According to [**GNU General Public License v3.0, 7(c)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE))
   - Modified versions **must not include the original program names “Bad Craft Launcher”, “HyperLightBCL” or its abbreviation “BCL” in their names, nor use names similar to the official names that may cause confusion.**
   - All modified versions **must clearly mark themselves as “Unofficial Modified Version” on the program startup page or main interface.**
   - All modified versions **must clearly state in their repository, README, program, etc. that they are modified based on “Bad Craft Launcher” or “HyperLightBCL”.**

2. You **must not remove the copyright notices displayed by the program.** (According to [**GNU General Public License v3.0, 7(b)[↗]**](https://github.com/DCR-Studio/HyperLightBCL/blob/main/LICENSE#L368-L370))

**Copyright ©2024-2025 DCR Studio and contributors. All rights reserved**
