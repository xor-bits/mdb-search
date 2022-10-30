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
    results: Option<Vec<SearchResult>>,
    error_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    id: String,
    result_type: String,
    //image: String,
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TitleData {
    // id: String,
    year: String,
}

struct Movie {
    id: String,
    title: String,
    desc: String,
}

//

fn main() -> ! {
    // collect the IMDB api key
    // read from fs
    let api_key = std::fs::read_to_string(API_KEY_FILE)
        .ok()
        .unwrap_or_else(|| {
            // read from stdin
            let key = Password::with_theme(&ColorfulTheme::default())
                .with_prompt("IMDB API key")
                .interact()
                .unwrap();

            // ask to save
            if dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Save it to {}?", API_KEY_FILE))
                .interact()
                .unwrap()
            {
                // save
                if let Err(err) = std::fs::write(API_KEY_FILE, &key) {
                    eprintln!("Failed to write: {err}");
                }
            }

            key
        });
    // url encode it, because it will be used in requests
    let api_key = urlencoding::encode(&api_key);

    // clipboard
    let mut ctx: Option<ClipboardContext> = ClipboardProvider::new().ok();

    loop {
        // IMDB search text
        let q: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your query")
            .interact_text()
            .unwrap();
        // url encode the search text
        let url_q = urlencoding::encode(&q);

        // progress bar
        let bp = ProgressBar::new_spinner();
        bp.enable_steady_tick(Duration::from_millis(120));
        bp.set_style(ProgressStyle::default_spinner());
        bp.set_message("Reqwesting...");

        // query IMDB
        let res = reqwest::blocking::get(format!(
            "https://imdb-api.com/en/API/SearchMovie/{api_key}/{url_q}",
        ))
        .and_then(|r| r.json::<SearchData>())
        .map(|data| {
            (
                data.error_message,
                data.results
                    .into_iter()
                    .flat_map(|i| i.into_iter())
                    .filter(|s| s.result_type == "Movie")
                    .map(|s| Movie {
                        id: s.id,
                        title: s.title,
                        desc: s.description,
                    })
                    .collect::<Vec<_>>(),
            )
        });

        match res {
            Ok((error_message, res)) => {
                bp.finish_with_message("Done");

                // res.sort_by(|a, b| a.title.cmp(&b.title));

                // api error
                if !error_message.is_empty() {
                    eprintln!("IMDB API error: {error_message}");
                    continue;
                }

                // convert results to a list
                let results: Vec<_> = res
                    .iter()
                    .map(|m| {
                        format!(
                            "{}\n - {}...",
                            m.title,
                            m.desc.chars().take(100).collect::<String>()
                        )
                    })
                    .collect();

                // pick a movie
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pick the movie")
                    .default(0)
                    .items(&results[..])
                    .interact()
                    .unwrap();

                // and save title + year to the clipboard
                if let Some(ctx) = ctx.as_mut() {
                    let Movie { id, title, .. } = &res[selection];

                    let year = match reqwest::blocking::get(format!(
                        "https://imdb-api.com/en/API/Title/{api_key}/{}",
                        urlencoding::encode(id)
                    ))
                    .and_then(|r| r.json::<TitleData>())
                    {
                        Ok(TitleData { year }) => format!(" ({year})"),
                        Err(_) => "".to_string(),
                    };

                    if let Err(err) = ctx.set_contents(format!("{title}{year}")) {
                        eprintln!("Failed to set clipboard: {err}");
                    }

                    println!("Copied to clipboard");
                }

                println!();
            }
            Err(err) => {
                // json parse error or request error
                bp.finish_with_message("Err");
                eprintln!("{err}");
            }
        }
    }
}
