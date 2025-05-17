# Jump

ディレクトリ間を素早く移動するためのコマンドラインツール

## 概要

Jumpは、頻繁に使用するディレクトリにエイリアスを設定し、素早く移動できるようにするツールです。

## インストール

1. プロジェクトをビルド：
```bash
cargo build --release
```

2. シェル統合を設定：
```bash
source [jump.shのパス]
```

example:source /home/misty1999/cli/jump/jump.sh


bashrcやzshrcに上記を追加することで、永続的に使用可能になります。

## 使用方法

### ディレクトリの登録

現在のディレクトリを登録：
```bash
$ cd ~/Documents/projects/my-awesome-project
$ jump add proj
# 「proj」というエイリアスで現在のディレクトリを登録
```

特定のパスを直接登録：
```bash
$ jump add docs ~/Documents
# 「docs」というエイリアスで ~/Documents を登録
```

### 登録したディレクトリへの移動

```bash
$ jump proj
# ~/Documents/projects/my-awesome-project へジャンプ！
```

### エイリアスの削除

```bash
$ jump delete proj
# 「proj」エイリアスの登録を削除
```

### エイリアス一覧の表示

```bash
$ jump list
```

## コマンド一覧

- `jump add <エイリアス> [パス]` - ディレクトリをエイリアスとして登録
  - パスを省略すると現在のディレクトリを登録
- `jump <エイリアス>` - 登録したディレクトリへ移動
- `jump delete <エイリアス>` - 登録したエイリアスを削除
- `jump list` - 登録されているエイリアス一覧を表示

## 注意事項

- `add`と`delete`と`list`はエイリアスとして使用できません
- 設定は`~/.config/jump/config.json`に保存されます
