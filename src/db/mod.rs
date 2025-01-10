use self::{imdb::InternetMovieDatabase, tmdb::TheMovieDatabase};
use crate::{movie::Movie, Style};
use dialoguer::{
    theme::{ColorfulTheme, Theme},
    Confirm, FuzzySelect, Password,
};
use std::{env, fmt};

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
    fn key_prompt(name: &'static str, theme: &dyn Theme) -> String {
        Password::with_theme(theme)
            .with_prompt(format!("{name} API key"))
            .interact()
            .unwrap()
    }

    pub fn try_api_key(&self, theme: &dyn Theme) -> keyring::Result<String> {
        if let Ok(api_key_from_env) = env::var(format!("{}_API_KEY", self.name())) {
            return Ok(api_key_from_env);
        }

        // collect the (T|I)MDB api key
        let entry = keyring::Entry::new_with_target(
            "default",
            &format!("mdb-search-{}", self.name()),
            "none",
        )?;

        match entry.get_password() {
            Err(keyring::Error::NoEntry) => {}
            other => return other,
        };

        let key = Self::key_prompt(self.name(), theme);

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Save it to the keyring?")
            .interact()
            .unwrap_or(false)
        {
            return Ok(key);
        }

        if let Err(err) = entry.set_password(&key) {
            eprintln!("Failed to use keyring: {err}");
        }

        Ok(key)
    }

    pub fn api_key(&self, theme: &dyn Theme) -> String {
        let key = match self.try_api_key(theme) {
            Ok(key) => key,
            Err(err) => {
                eprintln!("Failed to use keyring: {err}");
                Self::key_prompt(self.name(), theme)
            }
        };

        // url encode it, because it will be used in requests
        urlencoding::encode(&key).to_string()
    }
}

impl fmt::Display for dyn MovieDatabase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.name(), self.full_name())
    }
}
