# 贡献指南 - HyperLightBCL(简体中文)

<b><a href="CONTRIBUTING.md">Engish</a> | 简体中文 | <a href="CONTRIBUTING_zh_hant.md">繁體中文</a>
</p>


感谢你对本项目的关注！本文档解释了如何报告问题、提出修改建议以及提交代码，以便维护者能够快速审查和接受你的贡献。

---

## 在开始之前

- 搜索已存在和已关闭的 Issues 与 Pull Requests，避免重复工作。
- 对于大型、架构性或破坏性更改，请在实现之前先开启一个 Issue 进行讨论。描述动机、要解决的问题和可能的实现方法。
- 如果不确定优先级或要提交到哪个分支，请在 Issue 中询问或联系维护者。

---

## 行为准则

请遵守项目的行为准则。如果仓库中存在 `CODE_OF_CONDUCT.md` 文件，请遵循其中的规范。保持尊重和建设性的交流有助于项目发展。

---

## 报告问题

在提交 Issue 时，请尽量使用模板（如有），并包括：

- 清晰的标题和简洁的问题描述。
- 复现问题的步骤（最好有最小可复现示例）。
- 环境详情：操作系统及版本、CPU 架构、Java/Rust/Node 版本（如果相关）。
- 期望的行为与实际行为。
- 相关日志、错误信息和截图。
- 使用的版本或提交号（如 `v0.1.0` 或 commit hash）。

安全漏洞：不要在公开的 Issue 中披露安全相关问题。请遵循仓库的 `SECURITY.md` 文件说明，或私下联系维护者报告漏洞。

---

## 开发工作流

以下是通过 GitHub Pull Request 贡献代码的典型流程。

### 1. Fork 并克隆

将仓库 Fork 到自己的 GitHub 账户，然后克隆并添加 upstream 远程：

```bash
# 克隆自己的 fork
git clone https://github.com/<your-username>/HyperLightBCL.git
cd HyperLightBCL
# 添加 upstream 以保持与原始仓库同步
git remote add upstream https://github.com/DCR-Studio/HyperLightBCL.git
````

### 2. 创建分支

从最新的 upstream/main（或维护者指定的分支）创建描述性分支。

命名建议：

* 使用简短、小写、kebab-case 风格：`feat/add-downloader`、`fix/issue-123`、`docs/update-readme`

示例：

```bash
git fetch upstream
git checkout -b feat/my-feature upstream/main
```

保持分支专注（每个分支只做一类逻辑修改），便于审核。

### 3. 保持分支最新

定期 rebase 或 merge upstream 的更新，保持分支最新。推荐使用 rebase 以保持更干净的提交历史：

```bash
git fetch upstream
git rebase upstream/main
# 解决冲突后继续
git rebase --continue
```

如果你更喜欢 merge：

```bash
git fetch upstream
git merge upstream/main
```

完成后推送到自己的 fork：

```bash
git push -f origin your-branch-name  # rebase 后需要强制推送
# 或 merge 后
git push origin your-branch-name
```

### 4. 本地构建、检查和测试

在提交 PR 之前，请确保项目能正确构建并通过测试。

Rust（后端/核心）：

```bash
# 格式化与检查
cargo fmt --all
cargo clippy --all -- -D warnings
# 运行测试
cargo test
```

前端（Tauri / React / TypeScript）：

```bash
npm install
npm run lint        # 检查代码风格（如果配置）
npm run build       # 确认能正常构建
npm test            # 运行前端测试
```

根据仓库的配置文件和 CI 调整命令。如有不清楚的地方，请在 Issue 中询问。

### 5. 提交信息规范

本项目使用 **Conventional Commits**（Angular 风格），以便生成清晰的 changelog 和历史。

基本格式：

```
<type>(<scope>): <subject>

<body>

<footer>
```

* 标题简洁（建议 ≤ 50 字符），使用祈使句现在时，如 "add"、"fix"、"update"。
* 正文换行宽度约 72 字符。

常见类型：

* feat: 新功能
* fix: 修复 bug
* docs: 仅文档修改
* style: 格式化、空格、缺少分号等
* refactor: 既不是修复 bug 也不是新增功能的代码改动
* perf: 性能优化
* test: 添加或修复测试
* build: 构建系统或依赖相关修改
* ci: CI 配置修改
* chore: 其他不影响 src 或 tests 的改动

示例：

```
feat(auth): add microsoft oauth login
fix(startup): handle missing game directory on windows
docs(readme): improve installation instructions
```

如果你的改动回滚了之前的提交，请以 `revert:` 开头，并在正文中包含 `This reverts commit <hash>.`。

如有破坏性 API 变更，请在 footer 加入 `BREAKING CHANGE:` 并附上迁移说明。

### 6. 推送并提交 Pull Request

将分支推送到 fork 仓库，然后向 upstream 提交 Pull Request（目标分支通常为 `main`，除非维护者指定）。

PR 描述应包含：

* 简要说明改动和动机。
* 相关 Issues 和 PR（如 `Closes #123`）。
* 本地测试方法，方便审阅者复现。
* 如果涉及 UI 或用户可见行为，请附上截图或日志。

响应审查意见，按需更新分支。保持提交整洁，必要时使用 rebase。

---

## Pull Request 检查清单

* [ ] 我已阅读 CONTRIBUTING 指南。
* [ ] 我的分支基于最新的 upstream/main。
* [ ] 我已添加测试（如适用），且测试本地通过。
* [ ] 我已运行过代码检查和格式化。
* [ ] PR 描述中解释了动机和实现细节。
* [ ] 我已关联相关 Issues（如 `Closes #...`）。
* [ ] 我已考虑改动的安全性与隐私影响。
* [ ] CI 检查通过（或说明了暂未适用原因）。

---

## 开发规范与工具

* Rust 使用 `rustfmt` / `cargo fmt`，前端使用 `prettier` / `eslint`。仓库可能包含 `.rustfmt.toml`、`.prettierrc`、`.eslintrc` 等配置。
* 保持改动小而专注，便于审查。
* 为公共 API 和复杂逻辑编写注释和文档。

---

## 安全与敏感数据

* 不要提交密钥、API key 或私人凭证。如果误提交，请立即通知维护者，以便替换密钥并修复历史。
* 按 `SECURITY.md` 指引报告漏洞。

---

## 许可与贡献者协议

提交贡献即表示你同意你的贡献将遵循项目的开源协议（见仓库 `LICENSE` 文件）。

如项目要求 DCO 或 CLA，维护者会明确说明。如果需要 DCO，请在提交时使用 `git commit -s` 签名。

---

## 获取帮助

如果需要帮助，请开启一个带 `help wanted` 标签的 Issue，或加入开发群组（联系方式见 README）。维护者和贡献者会尽量回复，但响应时间可能有所不同。

---

## 致谢

感谢你为 HyperLightBCL 做出的贡献！无论大小，你的努力都将产生积极影响。

***简体中文文档由ChatGPT翻译***