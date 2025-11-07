# rust_thebook

Rustの公式ドキュメント「The Book」を使った学習用リポジトリです。

## プロジェクト構成

`project/` ディレクトリ内に各章の学習プロジェクトを配置しています。

## 新しいプロジェクトの作成方法

```bash
cd project
cargo new --vcs none <プロジェクト名>
```

`--vcs none` オプションを使用することで、個別のgitリポジトリが作成されず、このリポジトリ全体で一元管理できます。

## プロジェクトの実行

```bash
cd project/<プロジェクト名>
cargo run
```