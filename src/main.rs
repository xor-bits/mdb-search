use clipboard::ClipboardProvider;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use style::Style;

//

mod cb;
mod db;
mod movie;
mod style;

//

//

fn main() -> ! {
    let theme = ColorfulTheme::default();

    let db = db::select();

    let style = style::select(&theme);

    let mut cb = cb::init(&theme);

    let api_key = db.api_key(&theme);

    loop {
        let search: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your query")
            .interact_text()
            .unwrap();

        // progress bar
        let bp = ProgressBar::new_spinner();
        bp.enable_steady_tick(Duration::from_millis(120));
        bp.set_style(ProgressStyle::default_spinner());
        bp.set_message("Reqwesting...");

        let res: Vec<_> = match db.query(style, &api_key, &search) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("DB API error: {err}");
                continue;
            }
        };

        bp.finish();

        // and save title + year to the clipboard
        if let Some(cb) = cb.as_mut() {
            // pick a movie
            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick the movie")
                .default(0)
                .items(&res)
                .interact()
                .unwrap();

            let movie = &res[selection];

            if let Err(err) = cb.set_contents(format!("{movie:#}")) {
                eprintln!("Failed to set clipboard: {err}");
            }
        }

        println!("\nResults:");
        for res in res {
            println!("{res}");
        }

        println!();
    }
}
