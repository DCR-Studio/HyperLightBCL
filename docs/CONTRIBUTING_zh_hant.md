# 貢獻指南 - HyperLightBCL(繁體中文)

<b><a href="CONTRIBUTING.md">Engish</a> | <a href="CONTRIBUTING_zh_hans.md">简体中文</a> | 繁體中文
</p>


感謝你對本專案的關注！本文檔解釋了如何回報問題、提出修改建議以及提交程式碼，以便維護者能夠快速審查並接受你的貢獻。

---

## 在開始之前

- 搜尋已存在與已關閉的 Issues 和 Pull Requests，避免重複工作。
- 對於大型、架構性或破壞性更改，請在實作之前先開啟一個 Issue 進行討論。描述動機、要解決的問題與可能的實作方法。
- 如果不確定優先級或要提交到哪個分支，請在 Issue 中詢問或聯絡維護者。

---

## 行為準則

請遵守專案的行為準則。如果倉庫中存在 `CODE_OF_CONDUCT.md` 檔案，請遵循其中規範。保持尊重與建設性的交流有助於專案發展。

---

## 回報問題

在提交 Issue 時，請盡量使用範本（如有），並包括：

- 清楚的標題與簡潔的問題描述。
- 重現問題的步驟（最好有最小可重現示例）。
- 環境詳情：作業系統及版本、CPU 架構、Java/Rust/Node 版本（如相關）。
- 預期行為與實際行為。
- 相關日誌、錯誤訊息與截圖。
- 使用的版本或提交號（如 `v0.1.0` 或 commit hash）。

安全漏洞：請勿在公開的 Issue 中揭露安全相關問題。請遵循倉庫的 `SECURITY.md` 檔案說明，或私下聯絡維護者回報漏洞。

---

## 開發流程

以下是透過 GitHub Pull Request 貢獻程式碼的典型流程。

### 1. Fork 並克隆

將倉庫 Fork 到自己的 GitHub 帳號，然後克隆並新增 upstream 遠端：

```bash
# 克隆自己的 fork
git clone https://github.com/<your-username>/HyperLightBCL.git
cd HyperLightBCL
# 新增 upstream 以保持與原始倉庫同步
git remote add upstream https://github.com/DCR-Studio/HyperLightBCL.git
````

### 2. 建立分支

從最新的 upstream/main（或維護者指定的分支）建立描述性分支。

命名建議：

* 使用簡短、小寫、kebab-case 風格：`feat/add-downloader`、`fix/issue-123`、`docs/update-readme`

範例：

```bash
git fetch upstream
git checkout -b feat/my-feature upstream/main
```

保持分支專注（每個分支只處理一類邏輯修改），方便審查。

### 3. 保持分支最新

定期 rebase 或 merge upstream 的更新，保持分支最新。建議使用 rebase，以保持更乾淨的提交歷史：

```bash
git fetch upstream
git rebase upstream/main
# 解決衝突後繼續
git rebase --continue
```

若你偏好 merge：

```bash
git fetch upstream
git merge upstream/main
```

完成後推送至自己的 fork：

```bash
git push -f origin your-branch-name  # rebase 後需要強制推送
# 或 merge 後
git push origin your-branch-name
```

### 4. 本地建置、檢查與測試

在提交 PR 之前，請確認專案能正確建置並通過測試。

Rust（後端/核心）：

```bash
# 格式化與檢查
cargo fmt --all
cargo clippy --all -- -D warnings
# 執行測試
cargo test
```

前端（Tauri / React / TypeScript）：

```bash
npm install
npm run lint        # 檢查程式碼風格（如有設定）
npm run build       # 確認能正常建置
npm test            # 執行前端測試
```

依倉庫的設定檔與 CI 調整指令。如有疑問，請在 Issue 中詢問。

### 5. 提交訊息規範

本專案使用 **Conventional Commits**（Angular 風格），以利生成清楚的 changelog 與歷史。

基本格式：

```
<type>(<scope>): <subject>

<body>

<footer>
```

* 標題簡潔（建議 ≤ 50 字元），使用祈使句現在時，如 "add"、"fix"、"update"。
* 正文換行寬度約 72 字元。

常見類型：

* feat: 新功能
* fix: 修復 bug
* docs: 僅文件修改
* style: 格式、空白、缺分號等
* refactor: 非修復 bug 或新增功能的程式碼更動
* perf: 效能優化
* test: 新增或修復測試
* build: 建置系統或相依修改
* ci: CI 設定修改
* chore: 其他不影響 src 或 tests 的更動

範例：

```
feat(auth): add microsoft oauth login
fix(startup): handle missing game directory on windows
docs(readme): improve installation instructions
```

若你的更動回退了先前的提交，請以 `revert:` 開頭，並在正文中包含 `This reverts commit <hash>.`。

如有破壞性 API 更動，請在 footer 加入 `BREAKING CHANGE:` 並附上遷移說明。

### 6. 推送並提交 Pull Request

將分支推送至 fork 倉庫，然後向 upstream 提交 Pull Request（目標分支通常為 `main`，除非維護者指定）。

PR 描述應包含：

* 簡要說明更動與動機。
* 相關 Issues 與 PR（如 `Closes #123`）。
* 本地測試方式，方便審閱者重現。
* 若影響 UI 或使用者可見行為，請附上截圖或日誌。

回應審查意見，按需更新分支。保持提交乾淨，必要時使用 rebase。

---

## Pull Request 檢查清單

* [ ] 我已閱讀 CONTRIBUTING 指南。
* [ ] 我的分支基於最新的 upstream/main。
* [ ] 我已新增測試（如適用），且測試本地通過。
* [ ] 我已執行過程式碼檢查與格式化。
* [ ] PR 描述中解釋了動機與實作細節。
* [ ] 我已關聯相關 Issues（如 `Closes #...`）。
* [ ] 我已考慮更動的安全性與隱私影響。
* [ ] CI 檢查通過（或說明了暫未適用原因）。

---

## 開發規範與工具

* Rust 使用 `rustfmt` / `cargo fmt`，前端使用 `prettier` / `eslint`。倉庫可能包含 `.rustfmt.toml`、`.prettierrc`、`.eslintrc` 等設定。
* 保持更動小而專注，方便審查。
* 為公開 API 與複雜邏輯撰寫註解與文件。

---

## 安全與敏感資料

* 請勿提交金鑰、API key 或私人憑證。若誤提交，請立即通知維護者，以便替換金鑰並修復歷史。
* 請依 `SECURITY.md` 指引回報漏洞。

---

## 授權與貢獻者協議

提交貢獻即表示你同意你的貢獻將遵循專案的開源授權（見倉庫 `LICENSE` 檔案）。

如專案要求 DCO 或 CLA，維護者會明確說明。如需 DCO，請在提交時使用 `git commit -s` 簽名。

---

## 獲取幫助

若需要幫助，請開啟一個帶 `help wanted` 標籤的 Issue，或加入開發群組（聯絡方式見 README）。維護者與貢獻者會盡量回覆，但回應時間可能有所不同。

---

## 致謝

感謝你為 HyperLightBCL 所做的貢獻！無論大小，你的努力都將產生正面影響。

***繁體中文文件由ChatGPT翻譯***