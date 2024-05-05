use std::fs::{self, File};
use std::io::{self, Write}; // Writeをインポートしてflush()を使用可能にする

const MEDIA_LIST: [&str; 2] = ["qiita", "zenn"];

fn main() {
    let media = get_media();
    let title = read_input("title");
    let dir = media + "/" + &title;
    fs::create_dir_all(&dir).unwrap();

    make_content(&dir);
}

fn read_input(label: &str) -> String {
    // プロンプトを表示
    print!("{}?: ", label);
    io::stdout().flush().unwrap(); // 標準出力をフラッシュしてプロンプトをすぐに表示

    let mut input = String::new(); // 文字列を格納するための変数

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // 標準入力からの読み取り

    let output = input.trim_end().to_string();
    output
}

fn make_content(dir: &str) {
    let path = dir.to_string() + "/content.md";
    File::create(path).unwrap();
}

fn get_media() -> String {
    let mut media = String::new();
    loop {
        media = read_input("media");
        if validate_media(&media) {
            return media;
        } else {
            continue;
        }
    }
}

fn validate_media(media: &str) -> bool {
    MEDIA_LIST.contains(&media)
}
