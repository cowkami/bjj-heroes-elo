use anyhow::Result;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    let html_content = cache("bjj_heros_raw.html", || {
        load_data("https://www.bjjheroes.com/a-z-bjj-fighters-list")
    })?;
    // parse data
    let document = scraper::Html::parse_document(&html_content);
    let html_table_row_selector = scraper::Selector::parse("tr").unwrap();
    let html_table_rows = document.select(&html_table_row_selector);
    for row in html_table_rows {
        println!("{row:?}");
        break;
    }
    Ok(())
}

fn load_data(url: &str) -> String {
    let response = reqwest::blocking::get(url);
    response
        .expect(&format!("Failed to get response from {url}"))
        .text()
        .expect("Failed to convert as text")
}

fn cache<F>(cache_name: &str, f: F) -> Result<String>
where
    F: Fn() -> String,
{
    let dir_path = Path::new("./artifacts");
    let cache_path = dir_path.join(Path::new(cache_name));
    create_dir(&dir_path);
    let output = if cache_path.exists() {
        fs::read_to_string(&cache_path)?
    } else {
        f()
    };
    dump(&cache_path, &output)?;
    Ok(output)
}

fn create_dir(dir_path: &Path) {
    match std::fs::create_dir(dir_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }
}

fn dump(file_path: &Path, data: &str) -> Result<()> {
    let mut file = fs::File::create(file_path)?;
    let bin_data = data.as_bytes();
    file.write_all(bin_data)?;
    Ok(())
}
