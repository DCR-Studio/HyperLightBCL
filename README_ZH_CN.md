## **OpenBCLCore**
#####
**```A Open Source BCL-Launcher Core For Rust```**
#####
[English](README.md)
#####
**目前开发的 BCL-Launcher Core 分支使用 Rust 编写**
#####
## **警告：**
- **OpenBCLCore 使用 Rust 编写，与 BCL-Launcher 使用 Python 编写的 Core 无关！**
- **OpenBCLCore 采用 GPL-3 许可证。OpenBCLCore 早期开发时难以避免会出现 bug。若在我们的源代码或使用过程中发现任何 bug，请提交至我们的 Github 上的 Issues，谢谢！**
- **BCL-Launcher Core 分支处于早期开发阶段，不建议长期使用。**
#####
## **待完成功能：**

* [ ] 启动游戏版本
* [ ] 游戏版本下载和安装功能
* [ ] 下载并安装 CurseForge 和 Modrinth 模组、资源、存档、光影和整合包
* [ ] 账户登录系统（Microsoft OAuth 登录、离线登录及第三方身份验证服务器登录）
* [ ] 更多更多功能待添加
#####
## **已完成功能：**

**暂无**
#####
## **如何使用？**

- **通过 Github Actions 构建中下载**
- **配置 Rust 环境进行编译**
#####
## **Rust 构建步骤：**

**第一种方法：**
```bash
git clone  git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo build
# 编译后的二进制文件存放路径为 target/debug
```

**第二种方法：**
```bash
git clone git@github.com:DCR-Studio/OpenBCLCore.git
cd OpenBCLCore
cargo run
```
#####
## **许可证**
**本项目代码遵循 **[GPL-3.0 license](LICENSE)** 开源协议**
#####
**最终解释权归 DCR-Studio 所有**
