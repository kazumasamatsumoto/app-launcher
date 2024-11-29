Rust で CLI アプリケーションランチャーを作成しましょう。基本的な機能として、指定したアプリケーションを起動できるようにします。

```rust
use std::process::Command;
use std::env;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    // アプリケーションと実行パスのマッピング
    let mut apps = HashMap::new();

    // Windowsの場合
    #[cfg(target_os = "windows")]
    {
        apps.insert("chrome", r"C:\Program Files\Google\Chrome\Application\chrome.exe");
        apps.insert("notepad", r"C:\Windows\System32\notepad.exe");
        apps.insert("calc", r"C:\Windows\System32\calc.exe");
    }

    // macOSの場合
    #[cfg(target_os = "macos")]
    {
        apps.insert("chrome", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome");
        apps.insert("safari", "/Applications/Safari.app/Contents/MacOS/Safari");
        apps.insert("terminal", "/System/Applications/Utilities/Terminal.app/Contents/MacOS/Terminal");
    }

    // Linuxの場合
    #[cfg(target_os = "linux")]
    {
        apps.insert("chrome", "google-chrome");
        apps.insert("firefox", "firefox");
        apps.insert("terminal", "gnome-terminal");
    }

    match command.as_str() {
        "list" => {
            println!("Available applications:");
            for app in apps.keys() {
                println!("- {}", app);
            }
        },
        "run" => {
            if args.len() < 3 {
                println!("Please specify an application to run");
                return;
            }

            let app_name = &args[2];
            launch_application(app_name, &apps);
        },
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  app-launcher list              - List available applications");
    println!("  app-launcher run <app-name>    - Launch specified application");
}

fn launch_application(app_name: &str, apps: &HashMap<&str, &str>) {
    match apps.get(app_name) {
        Some(path) => {
            if Path::new(path).exists() || cfg!(target_os = "linux") {
                let status = Command::new(path)
                    .spawn()
                    .expect("Failed to launch application");
                println!("Launched {} (PID: {})", app_name, status.id());
            } else {
                println!("Application path not found: {}", path);
            }
        },
        None => println!("Unknown application: {}", app_name)
    }
}

```

この CLI ツールの主な機能を説明します：

1. `list` コマンド：利用可能なアプリケーションの一覧を表示

```bash
app-launcher list
```

2. `run` コマンド：指定したアプリケーションを起動

```bash
app-launcher run chrome
```

主な特徴：

- OS 別に適切なパスとコマンドを設定
- エラーハンドリング（存在しないアプリケーション、無効なコマンドなど）
- シンプルで使いやすいインターフェース

改善できる点：

1. 設定ファイルからアプリケーションの一覧を読み込む機能
2. アプリケーションへの引数渡し
3. 環境変数からパスを取得する機能
4. アプリケーションの追加・削除機能

これらの機能を追加しましょうか？また、特定のアプリケーションに対する対応が必要な場合は、お知らせください。

cargo new からアプリケーションの作成、ビルド、インストールまでの手順を説明します。

1. まず、新しいプロジェクトを作成します：

```bash
cargo new app-launcher
cd app-launcher
```

2. Cargo.toml を編集して必要な情報を追加します：

```toml
[package]
name = "app-launcher"
version = "0.1.0"
edition = "2021"
authors = ["あなたの名前 <メールアドレス>"]
description = "アプリケーションランチャーCLI"

[dependencies]
```

3. 先ほどのコードを `src/main.rs` に配置します。

4. ローカルでビルドしてテストを行います：

```bash
cargo build
cargo run -- list  # 利用可能なアプリケーション一覧を表示
cargo run -- run chrome  # Chromeを起動
```

5. リリースビルドを作成します：

```bash
cargo build --release
```

6. システムにインストールする方法は以下の 2 つがあります：

### 方法 1: cargo install を使用する

```bash
cargo install --path .
```

これにより、実行ファイルは以下の場所にインストールされます：

- Linux/macOS: `~/.cargo/bin/`
- Windows: `%USERPROFILE%\.cargo\bin\`

### 方法 2: 手動でパスを通す

リリースビルドされたバイナリをシステムの PATH が通っているディレクトリにコピーします：

Linux/macOS の場合：

```bash
sudo cp target/release/app-launcher /usr/local/bin/
```

Windows の場合：

```bash
copy target\release\app-launcher.exe %USERPROFILE%\bin\
```

※ Windows の場合は、予め`%USERPROFILE%\bin`などのディレクトリを作成し、PATH に追加しておく必要があります。

7. インストール後の確認：

```bash
app-launcher --version
app-launcher list
```

セキュリティに関する注意点：

- Windows では、実行時にセキュリティ警告が表示される場合があります
- Linux/macOS では、必要に応じて実行権限を付与します：

```bash
chmod +x /usr/local/bin/app-launcher
```

開発時の便利な機能：

```bash
# デバッグビルド＆実行（開発時）
cargo run -- list

# コードフォーマット
cargo fmt

# コードチェック
cargo clippy
```

アンインストール方法：

```bash
# cargo installでインストールした場合
cargo uninstall app-launcher

# 手動でインストールした場合
# Linux/macOS
sudo rm /usr/local/bin/app-launcher

# Windows
del %USERPROFILE%\bin\app-launcher.exe
```

これで基本的な開発環境からインストールまでの流れが整います。何か具体的な部分で質問や確認したい点はありますか？

docker-desktop、slack、chatwork を起動できるように、先ほどのコードを修正します。OS ごとに適切なパスを設定します。

```rust
use std::process::Command;
use std::env;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    // アプリケーションと実行パスのマッピング
    let mut apps = HashMap::new();

    // Windowsの場合
    #[cfg(target_os = "windows")]
    {
        apps.insert("docker-desktop", r"C:\Program Files\Docker\Docker\Docker Desktop.exe");
        apps.insert("slack", r"C:\Users\%USERNAME%\AppData\Local\slack\slack.exe");
        apps.insert("chatwork", "https://www.chatwork.com/");  // ブラウザで開く
        apps.insert("chrome", r"C:\Program Files\Google\Chrome\Application\chrome.exe");
    }

    // macOSの場合
    #[cfg(target_os = "macos")]
    {
        apps.insert("docker-desktop", "/Applications/Docker.app/Contents/MacOS/Docker Desktop");
        apps.insert("slack", "/Applications/Slack.app/Contents/MacOS/Slack");
        apps.insert("chatwork", "https://www.chatwork.com/");  // ブラウザで開く
        apps.insert("chrome", "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome");
    }

    // Linuxの場合
    #[cfg(target_os = "linux")]
    {
        apps.insert("docker-desktop", "docker-desktop");  // パッケージマネージャーでインストールした場合
        apps.insert("slack", "/usr/bin/slack");
        apps.insert("chatwork", "https://www.chatwork.com/");  // ブラウザで開く
        apps.insert("chrome", "google-chrome");
    }

    match command.as_str() {
        "list" => {
            println!("Available applications:");
            for app in apps.keys() {
                println!("- {}", app);
            }
        },
        "run" => {
            if args.len() < 3 {
                println!("Please specify an application to run");
                return;
            }

            let app_name = &args[2];
            launch_application(app_name, &apps);
        },
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  app-launcher list              - List available applications");
    println!("  app-launcher run <app-name>    - Launch specified application");
    println!("\nAvailable applications:");
    println!("  - docker-desktop   (Docker Desktop)");
    println!("  - slack           (Slack)");
    println!("  - chatwork        (Chatwork in browser)");
    println!("  - chrome          (Google Chrome)");
}

fn launch_application(app_name: &str, apps: &HashMap<&str, &str>) {
    match app_name {
        "chatwork" => {
            // ChatworkはブラウザでURLを開く
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(["/C", "start", apps.get("chatwork").unwrap()])
                    .spawn()
                    .expect("Failed to open Chatwork");
            }
            #[cfg(target_os = "macos")]
            {
                Command::new("open")
                    .arg(apps.get("chatwork").unwrap())
                    .spawn()
                    .expect("Failed to open Chatwork");
            }
            #[cfg(target_os = "linux")]
            {
                Command::new("xdg-open")
                    .arg(apps.get("chatwork").unwrap())
                    .spawn()
                    .expect("Failed to open Chatwork");
            }
            println!("Opened Chatwork in default browser");
        },
        _ => {
            // その他のアプリケーション
            match apps.get(app_name) {
                Some(path) => {
                    if Path::new(path).exists() || cfg!(target_os = "linux") {
                        let status = Command::new(path)
                            .spawn()
                            .expect("Failed to launch application");
                        println!("Launched {} (PID: {})", app_name, status.id());
                    } else {
                        println!("Application path not found: {}", path);
                    }
                },
                None => println!("Unknown application: {}", app_name)
            }
        }
    }
}

```

主な変更点：

1. Docker Desktop、Slack、Chatwork のパスを追加
2. Chatwork はブラウザベースなので、OS ごとに適切なコマンドでブラウザを開くように実装
3. 使用方法の説明を更新

使い方：

```bash
# アプリケーション一覧を表示
app-launcher list

# Docker Desktopを起動
app-launcher run docker-desktop

# Slackを起動
app-launcher run slack

# Chatworkをブラウザで開く
app-launcher run chatwork
```

注意点：

1. パスは実際のインストール場所に応じて調整が必要かもしれません
2. Windows の場合、`%USERNAME%`は実際のユーザー名に置き換える必要があります
3. インストールされていないアプリケーションは起動に失敗します

必要に応じて以下の機能を追加できます：

1. 設定ファイルでパスをカスタマイズ可能に
2. アプリケーションの起動オプション追加
3. 既に起動しているかのチェック機能
4. エラーメッセージの詳細化

これらの追加機能が必要な場合は、お知らせください。
