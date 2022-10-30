use clipboard::{ClipboardContext, ClipboardProvider};
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::time::Duration;

//

const API_KEY_FILE: &str = ".imdb-key";

//

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchData {
    // search_type: String,
    // expression: String,
    results: Vec<SearchResult>,
    error_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    id: String,
    //result_type: String,
    //image: String,
    title: String,
    //description: String,
    year: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TitleData {
    // id: String,
    year: String,
}

//

fn main() {
    let api_key = std::fs::read_to_string(API_KEY_FILE)
        .ok()
        .unwrap_or_else(|| {
            let key = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("IMDB API key")
                .interact()
                .unwrap();

            if dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Save it to {}?", API_KEY_FILE))
                .interact()
                .unwrap()
            {
                if let Err(err) = std::fs::write(API_KEY_FILE, &key) {
                    eprintln!("Failed to write: {err}");
                }
            }

            key
        });

    let api_key = urlencoding::encode(&api_key);

    loop {
        let q: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your query")
            .interact_text()
            .unwrap();

        let bp = ProgressBar::new_spinner();
        bp.enable_steady_tick(Duration::from_millis(120));
        bp.set_style(ProgressStyle::default_spinner());
        bp.set_message("Reqwesting...");

        let url_q = urlencoding::encode(&q);

        let res = reqwest::blocking::get(format!(
            "https://imdb-api.com/en/API/SearchMovie/{api_key}/{url_q}",
        ))
        .and_then(|r| r.json::<SearchData>())
        .map(|mut data| {
            for result in data.results.iter_mut() {
                let id_url = urlencoding::encode(&result.id);
                result.year = match reqwest::blocking::get(format!(
                    "https://imdb-api.com/en/API/Title/{api_key}/{id_url}"
                ))
                .and_then(|r| r.json::<TitleData>())
                {
                    Ok(TitleData { year }) => Some(year),
                    Err(_) => None,
                };
            }
            data
        });

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

        match res {
            Ok(SearchData {
                results,
                error_message,
            }) => {
                bp.finish_with_message("Done");

                if !error_message.is_empty() {
                    eprintln!("IMDB API error: {error_message}");
                }

                let mut results: Vec<String> = results
                    .into_iter()
                    .map(|SearchResult { title, year, .. }| {
                        format!(
                            "{title}{}",
                            if let Some(year) = year {
                                format!(" ({year})")
                            } else {
                                "".to_string()
                            }
                        )
                    })
                    .collect();

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pick the movie")
                    .default(0)
                    .items(&results[..])
                    .interact()
                    .unwrap();

                ctx.set_contents(results.remove(selection)).unwrap();

                println!("Copied to clipboard");
            }
            Err(err) => {
                bp.finish_with_message("Err");
                eprintln!("{err}");
            }
        }
    }
}
