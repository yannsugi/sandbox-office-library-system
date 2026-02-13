# Scripts

バックエンド開発を効率化するスクリプト集。

---

## generate-schemas.sh

全ドメインのスキーマファイルを**自動検出**して一括生成します。

### 特徴

- `diesel.toml` から `[print_schema.*]` セクションを自動検出
- 新しいドメイン追加時にスクリプト修正不要！
- 成功/失敗を集計してレポート

### 使い方

```bash
cd backend
./scripts/generate-schemas.sh
```

### 実行例

```
🔧 Generating schema files for all domains...

📋 Detected domains from diesel.toml:
  - division
  - user
  - book

📝 Generating division schema...
   ✓ Schema generated successfully

📝 Generating user schema...
   ✓ Schema generated successfully

📝 Generating book schema...
   ✓ Schema generated successfully

==========================================
Summary:
  ✓ Success: 3
==========================================

✅ All schemas generated successfully!
```

### 個別生成

```bash
diesel print-schema --schema-key division
diesel print-schema --schema-key user
diesel print-schema --schema-key book
```

---

## 新しいドメインの追加

新しいドメイン（例: `author`）を追加する手順:

### 1. diesel.tomlに追加

```toml
[print_schema.author]
file = "db_domain/author/src/schema.rs"
filter = { only_tables = ["authors"] }
with_docs = true
```

### 2. スキーマ生成

```bash
./scripts/generate-schemas.sh
```

**それだけ！** スクリプトが自動的に新しいドメインを検出して生成します。

---

## トラブルシューティング

### diesel コマンドが見つからない

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### diesel.toml が見つからない

プロジェクトルートの `backend/diesel.toml` を確認してください。
設定内容を確認:

```bash
cat diesel.toml
```

### スキーマが生成されない

1. マイグレーション実行: `diesel migration run`
2. テーブル存在確認: `psql` で確認
3. diesel.toml の `filter` 設定確認
4. データベース接続確認: `.env` の `DATABASE_URL` を確認
