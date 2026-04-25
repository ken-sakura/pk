use std::{env, fs, path::Path, process};
use arboard::Clipboard;
use pulldown_cmark::{html, Options, Parser as MarkdownParser};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 引数チェック: プログラム名を除いて1つ以上の引数が必要
    if args.len() < 2 {
        eprintln!("エラー: 引数が指定されていません。");
        eprintln!("使用法: peek [-hp] <ファイル名>");
        process::exit(1);
    }

    let mut hp_mode = false;
    let mut file_path_arg = None;

    // 簡易的な引数解析
    for arg in args.iter().skip(1) {
        if arg == "-hp" {
            hp_mode = true;
        } else if file_path_arg.is_none() {
            file_path_arg = Some(arg);
        }
    }

    // ファイルパスが指定されていない場合のエラー
    let file_path = match file_path_arg {
        Some(path) => path,
        None => {
            eprintln!("エラー: ファイル名が指定されていません。");
            process::exit(1);
        }
    };

    let path = Path::new(file_path);

    // ファイルの存在確認
    if !path.exists() {
        eprintln!("エラー: ファイル '{}' が見つかりません。", file_path);
        process::exit(1);
    }

    if hp_mode {
        // -hp オプション時の処理
        // 拡張子が .md かチェック
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            eprintln!("エラー: -hp オプションを指定する場合は、マークダウンファイル（.md）を指定してください。");
            process::exit(1);
        }

        match fs::read_to_string(path) {
            Ok(content) => {
                // マークダウンをHTMLに変換
                let parser = MarkdownParser::new_ext(&content, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                // 標準出力に表示
                print!("{}", html_output);

                // クリップボードにコピー
                if let Err(e) = copy_to_clipboard(&html_output) {
                    eprintln!("\n警告: クリップボードへのコピーに失敗しました: {}", e);
                }
            }
            Err(e) => {
                eprintln!("エラー: ファイルの読み込みに失敗しました: {}", e);
                process::exit(1);
            }
        }
    } else {
        // 通常の cat 処理
        match fs::read_to_string(path) {
            Ok(content) => {
                // 標準出力に表示
                print!("{}", content);

                // クリップボードにコピー
                if let Err(e) = copy_to_clipboard(&content) {
                    eprintln!("\n警告: クリップボードへのコピーに失敗しました: {}", e);
                }
            }
            Err(e) => {
                eprintln!("エラー: ファイルの読み込みに失敗しました: {}", e);
                process::exit(1);
            }
        }
    }
}

/// 指定されたテキストをクリップボードにコピーする
fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}
