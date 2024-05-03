use std::io::{self, Write}; // Writeをインポートしてflush()を使用可能にする

fn main() {
    let media = read_input("media");
    println!("{}", media)
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
