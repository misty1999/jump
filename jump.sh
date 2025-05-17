#!/bin/bash

# jumpコマンドの実装
function jump() {
    if [ "$1" = "add" ]; then
        if [ -z "$2" ]; then
            echo "エイリアスを指定してください！"
            return 1
        fi
        if [ -n "$3" ]; then
            # パスが指定されている場合
            "$JUMP_BIN" add "$2" "$3"
        else
            # 現在のディレクトリを登録
            "$JUMP_BIN" add "$2"
        fi
    elif [ "$1" = "delete" ]; then
        if [ -z "$2" ]; then
            echo "削除するエイリアスを指定してください！"
            return 1
        fi
        "$JUMP_BIN" delete "$2"
    elif [ "$1" = "list" ]; then
        # エイリアス一覧を表示
        "$JUMP_BIN" list
    else
        # エイリアスへのジャンプ
        if [ -z "$1" ]; then
            echo "エイリアスを指定してください！"
            return 1
        fi
        target_dir=$("$JUMP_BIN" jump "$1")
        if [ $? -eq 0 ]; then
            cd "$target_dir"
        else
            echo "$target_dir"
            return 1
        fi
    fi
}

# jump_binのパスを設定
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JUMP_BIN="$SCRIPT_DIR/target/release/jump"

# jumpコマンドをエクスポート
export -f jump 