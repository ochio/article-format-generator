use colored::Colorize;
use dotenv;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write}; // Writeをインポートしてflush()を使用可能にする
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Media {
    Qiita,
    Zenn,
}

impl FromStr for Media {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_lowercase().as_str() {
            "qiita" => Ok(Media::Qiita),
            "zenn" => Ok(Media::Zenn),
            _ => Err("invalid input".to_string()),
        }
    }
}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Media::Qiita => write!(f, "qiita"),
            Media::Zenn => write!(f, "zenn"),
        }
    }
}

struct Article {
    media: Media,
    title: String,
    dir: String,
    file_path: String,
}

impl Article {
    fn new(media: Media, title: &str) -> Result<Article, String> {
        let base_dir = dotenv::var("BASE_DIR").unwrap();
        let dir = format!("{}/{}/{}", base_dir, media, title);
        let file_path = format!("{}/content.md", dir);
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory: {}", e))?;

        Ok(Article {
            media,
            title: title.to_string(),
            dir,
            file_path,
        })
    }

    fn make_content(&self) -> Result<(), String> {
        File::create(&self.file_path)
            .map_err(|e| format!("Failed to create content file: {}", e))?;
        Ok(())
    }

    fn create_symbolic(self) -> Result<(), String> {
        let base_dir = dotenv::var("BASE_DIR").unwrap();
        let contents_dir = format!("{}/all", base_dir);
        let linked_file = format!("{}/{}.md", contents_dir, self.title);

        if !Path::new(&contents_dir).is_dir() {
            match fs::create_dir(contents_dir) {
                Ok(_) => println!("Success create directory"),
                Err(_) => eprintln!("Failed to create directory"),
            }
        }

        Command::new("ln")
            .args(["-s", &self.file_path, &linked_file]) // シンボリックリンクの対象とリンク名
            .status() // コマンドを実行し、終了ステータスを取得
            .map_err(|_| "Failed to execute command".to_string())
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err("Symbolic link creation failed".to_string())
                }
            })
    }
}

fn main() {
    let (media, title) = prompt_for_article_info();

    match Article::new(media, &title) {
        Ok(article) => match article
            .make_content()
            .and_then(|_| article.create_symbolic())
        {
            Ok(_) => {
                println!("{}", "Article created successfully".green())
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(e) => eprintln!("Error creating article: {}", e),
    }
}

fn prompt_for_article_info() -> (Media, String) {
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

fn get_media() -> Media {
    loop {
        let media = read_input("media");
        match media.parse::<Media>() {
            Ok(media) => return media,
            Err(_) => println!("{}", "invalid media".red()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_from_str() {
        assert_eq!(Media::from_str("qiita"), Ok(Media::Qiita));
        assert_eq!(Media::from_str("zenn"), Ok(Media::Zenn));
        assert!(Media::from_str("blog").is_err());
    }

    #[test]
    fn create_new_directory() {
        let media = Media::Qiita;
        let title = "Test Article";
        let article = Article::new(media, title);

        assert!(article.is_ok());
        let article = article.unwrap();
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.media, Media::Qiita);

        assert!(fs::metadata(&article.dir).is_ok());
        cleanup_directory(&"blog");
    }

    #[test]
    fn make_content() {
        let media = Media::Qiita;
        let title = "Content Creation Test";
        let article = Article::new(media, title).unwrap();
        let result = article.make_content();

        assert!(result.is_ok());
        let content_path = format!("{}/content.md", article.dir);
        assert!(fs::metadata(&content_path).is_ok());
        cleanup_directory(&"blog");
    }

    fn cleanup_directory(dir: &str) {
        fs::remove_dir_all(dir).expect("Failed to clean up the test directory");
    }
}
