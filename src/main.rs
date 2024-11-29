use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Command;

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
        apps.insert(
            "docker-desktop",
            r"C:\Program Files\Docker\Docker\Docker Desktop.exe",
        );
        apps.insert(
            "slack",
            r"C:\Users\%USERNAME%\AppData\Local\slack\slack.exe",
        );
        apps.insert("chatwork", "https://www.chatwork.com/"); // ブラウザで開く
        apps.insert(
            "chrome",
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        );
    }

    // macOSの場合
    #[cfg(target_os = "macos")]
    {
        apps.insert(
            "docker-desktop",
            "/Applications/Docker.app/Contents/MacOS/Docker Desktop",
        );
        apps.insert("slack", "/Applications/Slack.app/Contents/MacOS/Slack");
        apps.insert("chatwork", "https://www.chatwork.com/"); // ブラウザで開く
        apps.insert(
            "chrome",
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        );
    }

    // Linuxの場合
    #[cfg(target_os = "linux")]
    {
        apps.insert("docker-desktop", "docker-desktop"); // パッケージマネージャーでインストールした場合
        apps.insert("slack", "/usr/bin/slack");
        apps.insert("chatwork", "https://www.chatwork.com/"); // ブラウザで開く
        apps.insert("chrome", "google-chrome");
    }

    match command.as_str() {
        "list" => {
            println!("Available applications:");
            for app in apps.keys() {
                println!("- {}", app);
            }
        }
        "run" => {
            if args.len() < 3 {
                println!("Please specify an application to run");
                return;
            }

            let app_name = &args[2];
            launch_application(app_name, &apps);
        }
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
        }
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
                }
                None => println!("Unknown application: {}", app_name),
            }
        }
    }
}
