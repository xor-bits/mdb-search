use self::{imdb::InternetMovieDatabase, tmdb::TheMovieDatabase};
use crate::{movie::Movie, Style};
use dialoguer::{
    theme::{ColorfulTheme, Theme},
    Confirm, FuzzySelect, Password,
};
use std::fmt;

//

pub mod imdb;
pub mod tmdb;

//

pub fn select() -> &'static dyn MovieDatabase {
    let databases: &[&dyn MovieDatabase] = &[&InternetMovieDatabase, &TheMovieDatabase];
    let database = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select movie database")
        .items(databases)
        .interact()
        .unwrap();
    databases[database]
}

//

pub trait MovieDatabase {
    fn name(&self) -> &'static str;

    fn full_name(&self) -> &'static str;

    fn id_name(&self) -> &'static str;

    fn query(&self, style: Style, api_key: &str, search: &str) -> Result<Vec<Movie>, String>;
}

impl dyn MovieDatabase {
    pub fn api_key(&self, theme: &dyn Theme) -> String {
        // collect the IMDB api key
        let entry = keyring::Entry::new(&format!("mdb-search-{}", self.name()), "none");

        // get the saved API key
        let api_key = entry.get_password().unwrap_or_else(|_| {
            // read from stdin
            let key = Password::with_theme(theme)
                .with_prompt(format!("{} API key", self.name()))
                .interact()
                .unwrap();

            // ask to save
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Save it to the keyring?")
                .interact()
                .unwrap()
            {
                // save
                if let Err(err) = entry.set_password(&key) {
                    eprintln!("Failed to write: {err}");
                }
            }

            key
        });

        // url encode it, because it will be used in requests
        urlencoding::encode(&api_key).to_string()
    }
}

impl fmt::Display for dyn MovieDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.name(), self.full_name())
    }
}
