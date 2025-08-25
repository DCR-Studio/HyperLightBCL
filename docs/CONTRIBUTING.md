# Contributing to HyperLightBCL

<b>Engish | <a href="CONTRIBUTING_zh_hans.md">简体中文</a> | <a href="CONTRIBUTING_zh_hant.md">繁體中文</a>
</p>

Thank you for your interest in contributing! This document explains how to report issues, propose changes, and submit code so maintainers can review and accept your contributions quickly.

---

## Before you start

- Search existing open and closed Issues and Pull Requests to avoid duplicating work.
- For large, architectural, or breaking changes, open an Issue to discuss the proposal before implementing it. Describe the motivation, the problem you want to solve, and possible implementation approaches.
- If you are unsure about priorities or the right branch to target, ask in an Issue or contact the maintainers.

---

## Code of conduct

Please follow the project's Code of Conduct. If a `CODE_OF_CONDUCT.md` file exists in the repository, follow it. Respectful and constructive communication helps the project thrive.

---

## Reporting issues

When opening an Issue, use the Issue template (if available) and include:

- A clear title and a concise description of the problem.
- Steps to reproduce the problem (minimal reproducible example if possible).
- Environment details: OS and version, CPU architecture, Java/Rust/Node versions if relevant.
- Expected behavior vs actual behavior.
- Relevant logs, error messages, and screenshots.
- The version or commit you used (e.g. `v0.1.0` or commit hash).

Security vulnerabilities: Do not disclose security-sensitive issues in a public Issue. Instead, follow the repository's SECURITY.md instructions or contact the maintainers privately to report the vulnerability.

---

## Development workflow

This section outlines a typical workflow for contributing code changes via GitHub Pull Requests.

### 1. Fork and clone

Fork the repository to your GitHub account, then clone your fork and add the upstream remote:

```bash
# clone your fork
git clone https://github.com/<your-username>/HyperLightBCL.git
cd HyperLightBCL
# add upstream remote to stay in sync with the original repo
git remote add upstream https://github.com/DCR-Studio/HyperLightBCL.git
```

### 2. Create a branch

Create a descriptive branch from the latest upstream/main (or the branch maintainers indicate).

Naming recommendations:

- Use short, lowercase, kebab-case names: `feat/add-downloader`, `fix/issue-123`, `docs/update-readme`.

Example:

```bash
git fetch upstream
git checkout -b feat/my-feature upstream/main
```

Keep branches focused (one logical change per branch) to simplify review.

### 3. Keep your branch up to date

Rebase or merge the upstream changes regularly to keep your branch current. We recommend rebasing for a cleaner history:

```bash
git fetch upstream
git rebase upstream/main
# resolve conflicts, then continue the rebase
git rebase --continue
```

If you prefer merges:

```bash
git fetch upstream
git merge upstream/main
```

Push to your fork after the rebase/merge:

```bash
git push -f origin your-branch-name  # force-push only after rebase
# or if merged
git push origin your-branch-name
```

### 4. Build, lint, and test locally

Before opening a PR, ensure the project builds and tests pass locally.

Rust (backend/core):

```bash
# format and check
cargo fmt --all
cargo clippy --all -- -D warnings
# run tests
cargo test
```

Frontend (Tauri / React / TypeScript):

```bash
npm install
npm run lint        # lint the codebase (if configured)
npm run build       # ensure the frontend builds
npm test            # run frontend tests
```

Adjust commands to match the repository's scripts and CI configuration. If a command is missing or unclear, ask in an Issue.

### 5. Commit messages

We use Conventional Commits (Angular style) to make changelogs and history clear.

Basic format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

- Keep the subject concise (recommended ≤ 50 characters). Use imperative present tense: "add", "fix", "update".
- Wrap the body at ~72 characters per line.

Common types:

- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Formatting, white-space, missing semi-colons, etc
- refactor: Code change that neither fixes a bug nor adds a feature
- perf: Performance improvements
- test: Adding or fixing tests
- build: Changes that affect the build system or external dependencies
- ci: CI configuration changes
- chore: Other changes that do not modify src or tests

Examples:

```
feat(auth): add microsoft oauth login
fix(startup): handle missing game directory on windows
docs(readme): improve installation instructions
```

If your change reverts a previous commit, start the subject with `revert:` and include `This reverts commit <hash>.` in the body.

If your change introduces breaking API changes include a `BREAKING CHANGE:` section in the footer with migration notes.

### 6. Push and open a Pull Request

Push your branch to your fork and open a Pull Request to the upstream repository (target the branch the maintainers specify, usually `main`).

Your PR description should include:

- A short summary of the change and motivation.
- Related issues and PRs (e.g., `Closes #123`).
- Tests you ran locally and instructions for reviewers to reproduce the change.
- Screenshots or logs if the change affects the UI or user-visible behavior.

Respond to review feedback and update your branch as requested. Keep commits tidy and use rebase when appropriate.

---

## Pull Request checklist

- [ ] I have read the CONTRIBUTING guide.
- [ ] My branch is based on the latest upstream/main and up to date.
- [ ] I have added tests where applicable and they pass locally.
- [ ] I have run linters and formatters.
- [ ] The PR description explains the motivation and implementation details.
- [ ] I have linked related issues (e.g., `Closes #...`).
- [ ] I have considered security and privacy implications of this change.
- [ ] CI checks pass (or I have documented why they do not yet apply).

---

## Development standards and tools

- Use `rustfmt` / `cargo fmt` for Rust formatting and `prettier` / `eslint` for frontend code. The repository may include configuration files such as `.rustfmt.toml`, `.prettierrc`, or `.eslintrc`.
- Keep changes small and focused to simplify review.
- Document public APIs and complex logic with comments and README updates.

---

## Security and sensitive data

- Do not commit secrets, API keys, or private credentials. If you accidentally commit sensitive information, notify the maintainers immediately so the secret can be rotated and the history remediated.
- Follow the repository's SECURITY.md guidance for reporting vulnerabilities.

---

## Licensing and contributor agreement

By contributing you agree that your contributions will be licensed under the project's license (see the repository `LICENSE` file).

If this project requires a Developer Certificate of Origin (DCO) or a Contributor License Agreement (CLA), maintainers will document that requirement. If DCO is required, sign commits with `git commit -s`.

---

## Getting help

If you need help, open an Issue with the `help wanted` label or ask in the development group (refer to README for contact details). Maintainters and contributors will try to respond as time allows — response times may vary.

---

## Thank you

Thanks for helping improve HyperLightBCL! Your contributions, however small, make a difference.
