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
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let heroes = document
        .select(&tr_selector)
        .filter_map(get_hero_link)
        .filter_map(get_hero_detail)
        .collect::<Vec<_>>();
    println!("{:?}", heroes);
    Ok(())
}

fn get_hero_link(row: scraper::ElementRef) -> Option<String> {
    row.select(&scraper::Selector::parse("a").unwrap())
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(|link| format!("https://www.bjjheroes.com{}", link))
}

fn get_hero_detail(link: String) -> Option<String> {
    // match ?p=123 to get id 123
    let id = match link.split("?p=").last() {
        Some(id) => id,
        None => return None,
    };

    let html_content = cache(&format!("{}.html", id), || load_data(&link)).ok()?;
    let document = scraper::Html::parse_document(&html_content);
    let p_selector = scraper::Selector::parse("p").unwrap();
    let name = document
        .select(&p_selector)
        .next()?
        .text()
        .collect::<String>();
    Some(name)
}

// fn get_hero_name(row: scraper::ElementRef) -> Option<String> {
//     let names = row
//         .select(&scraper::Selector::parse("td").unwrap())
//         .map(get_names)
//         .collect::<Vec<_>>();
//     let first = names.get(0);
//     let last = names.get(1);
//     if first.is_none() || last.is_none() {
//         return None;
//     }
//     Some(format!("{} {}", first.unwrap(), last.unwrap()))
// }

// fn get_names(column: scraper::ElementRef) -> String {
//     column
//         .select(&scraper::Selector::parse("a").unwrap())
//         .map(|ele| ele.text().collect::<String>())
//         .collect::<String>()
// }

#[derive(Debug)]
struct BjjHero {
    name: String,
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
    if !dir_path.exists() {
        create_dir(&dir_path);
    }
    let cache_path = dir_path.join(Path::new(cache_name));
    let output = if cache_path.exists() {
        println!("Loading cache: {:?}", cache_path);
        fs::read_to_string(&cache_path)?
    } else {
        println!("Fetching data from the web to {:?}", cache_path);
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
