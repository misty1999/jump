use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// ディレクトリをエイリアスとして登録
    Add {
        /// エイリアス名
        alias: String,
        /// 登録するパス（省略時は現在のディレクトリ）
        path: Option<String>,
    },
    /// 登録したエイリアスにジャンプ
    Jump {
        /// エイリアス名
        alias: String,
    },
    /// エイリアスを削除
    Delete {
        /// エイリアス名
        alias: String,
    },
    /// 登録されているエイリアス一覧を表示
    List,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    aliases: HashMap<String, String>,
}

impl Config {
    fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(config_path)
            .context("設定ファイルの読み込みに失敗しました")?;
        serde_json::from_str(&content).context("設定ファイルの解析に失敗しました")
    }

    fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("設定ディレクトリの作成に失敗しました")?;
        }
        let content = serde_json::to_string_pretty(self)
            .context("設定のシリアライズに失敗しました")?;
        fs::write(config_path, content).context("設定ファイルの書き込みに失敗しました")
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("設定ディレクトリが見つかりません")?;
        path.push("jump");
        path.push("config.json");
        Ok(path)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    match cli.command {
        Commands::Add { alias, path } => {
            if alias == "add" || alias == "delete" || alias == "list" {
                anyhow::bail!("'add'、'delete'、'list'はエイリアスとして使用できません");
            }
            let target_path = match path {
                Some(p) => PathBuf::from(p),
                None => std::env::current_dir()?,
            };
            if !target_path.exists() {
                anyhow::bail!("指定されたパスが存在しません: {}", target_path.display());
            }
            config.aliases.insert(alias.clone(), target_path.to_string_lossy().into_owned());
            config.save()?;
            println!("エイリアス '{}' を '{}' に登録しました！", alias, target_path.display());
        }
        Commands::Jump { alias } => {
            if let Some(path) = config.aliases.get(&alias) {
                println!("{}", path);
            } else {
                anyhow::bail!("エイリアス '{}' は登録されていません", alias);
            }
        }
        Commands::Delete { alias } => {
            if config.aliases.remove(&alias).is_some() {
                config.save()?;
                println!("エイリアス '{}' を削除しました！", alias);
            } else {
                anyhow::bail!("エイリアス '{}' は登録されていません", alias);
            }
        }
        Commands::List => {
            if config.aliases.is_empty() {
                println!("登録されているエイリアスはありません！");
            } else {
                println!("登録されているエイリアス一覧：");
                for (alias, path) in config.aliases.iter() {
                    println!("  {} -> {}", alias, path);
                }
            }
        }
    }

    Ok(())
}
