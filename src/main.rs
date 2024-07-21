use std::{fs::File, io::Write};

use clap::Parser;
use yaml::UrlPair;
pub mod args;
pub mod fetch;
pub mod html;
pub mod yaml;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    let url_pairs = yaml::read_yaml_file(&args.config)?;

    // 出力ディレクトリが存在しない場合は作成
    std::fs::create_dir_all(&args.output_dir)?;

    for pair in url_pairs {
        let pair_name = pair.name.clone();
        let html_content = run_test(&pair).await;

        // HTMLファイルに差分を保存
        let filename = format!("{}_diff.html", pair_name.replace(' ', "_"));
        let file_path = args.output_dir.join(filename);
        let mut file = File::create(file_path)?;
        file.write_all(html_content.as_bytes())?;
    }

    Ok(())
}

async fn run_test(pair: &UrlPair) -> String {
    println!("Comparing: {}", pair.name);
    let old_result = fetch::fetch_url_content(&pair.old_url, &pair.old_headers).await;
    let new_result = fetch::fetch_url_content(&pair.new_url, &pair.new_headers).await;
    match (old_result, new_result) {
        (Ok(old_content), Ok(new_content)) => {
            html::generate_html_diff(old_content, new_content, &pair.name)
        }
        (Err(e1), Err(e2)) => html::generate_html_error(
            &pair.name,
            &format!("Both URLs failed. Old: {:?}, New: {:?}", e1, e2),
        ),
        (Err(e), _) => html::generate_html_error(&pair.name, &format!("Old URL failed: {:?}", e)),
        (_, Err(e)) => html::generate_html_error(&pair.name, &format!("New URL failed: {:?}", e)),
    }
}