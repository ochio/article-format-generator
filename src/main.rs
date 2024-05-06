use colored::Colorize;
use std::fs::{self, File};
use std::io::{self, Write}; // Writeをインポートしてflush()を使用可能にする
use std::process::Command;

const MEDIA_LIST: [&str; 2] = ["qiita", "zenn"];

struct Article {
    media: String,
    title: String,
    dir: String,
}

impl Article {
    fn new(media: &str, title: &str) -> Article {
        let dir = format!("{}/{}", media, title);
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create directory: {}", e))
            .unwrap();

        Article {
            media: media.to_string(),
            title: title.to_string(),
            dir,
        }
    }

    fn make_content(&self) -> Result<(), String> {
        let path = format!("{}/content.md", self.dir);
        File::create(&path).map_err(|e| format!("Failed to create content file: {}", e))?;
        Ok(())
    }

    fn create_symbolic(self) {
        let target_path = format!("{}/content.md", self.dir); // 文字列連結をformat!で実行
        let linked_file = format!("{}.md", self.title);
        Command::new("ln")
            .args(["-s", &target_path, &linked_file]) // シンボリックリンクの対象とリンク名
            .status() // コマンドを実行し、終了ステータスを取得
            .expect("failed to execute command"); // エラー発生時にpanicを発生させる
    }
}

fn main() {
    let (media, title) = prompt_for_article_info();
    let article = Article::new(&media, &title);

    match article.make_content() {
        Ok(_) => {
            article.create_symbolic();
            println!("Article created successfully")
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

// TODO: mediaはenumで管理する
fn prompt_for_article_info() -> (String, String) {
    let media = get_media();
    let title = read_input("title");
    (media, title)
}

fn read_input(label: &str) -> String {
    // プロンプトを表示
    print!("{}?: ", label);
    io::stdout().flush().expect("failed to flush"); // 標準出力をフラッシュしてプロンプトをすぐに表示

    let mut input = String::new(); // 文字列を格納するための変数

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // 標準入力からの読み取り

    let output = input.trim_end().to_string();
    output
}

fn get_media() -> String {
    let mut media = String::new();
    loop {
        media = read_input("media");
        if validate_media(&media) {
            return media;
        } else {
            println!("{}", "invalid media".red());
            continue;
        }
    }
}

fn validate_media(media: &str) -> bool {
    MEDIA_LIST.contains(&media)
}
