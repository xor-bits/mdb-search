use super::MovieDatabase;
use crate::{movie::Movie, Style};
use serde::Deserialize;

//

pub struct TheMovieDatabase;

//

#[derive(Debug, Deserialize)]
struct SearchData {
    results: Option<Vec<SearchResult>>,
    status_message: Option<String>,
    status_code: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    id: Option<i32>,
    //poster_path: String,
    title: Option<String>,
    overview: Option<String>,
    release_date: Option<String>,
}

//

impl MovieDatabase for TheMovieDatabase {
    fn name(&self) -> &'static str {
        "TMDB"
    }

    fn full_name(&self) -> &'static str {
        "The Movie Database"
    }

    fn id_name(&self) -> &'static str {
        "tmdbid"
    }

    fn query(&self, style: Style, api_key: &str, search: &str) -> Result<Vec<Movie>, String> {
        // url encode the search text
        let search = urlencoding::encode(search);

        // query IMDb
        let res = reqwest::blocking::get(format!(
            "https://api.themoviedb.org/3/search/movie?api_key={api_key}&query={search}",
        ))
        .and_then(|r| r.json::<SearchData>())
        .map_err(|err| err.to_string())?;

        // api error
        if let Some(err) = res.status_message {
            return Err(err);
        }
        if let Some(err) = res.status_code {
            return Err(err.to_string());
        }

        Ok(res
            .results
            .into_iter()
            .flat_map(|i| i.into_iter())
            .filter_map(|s| Some((s.id?.to_string(), s.title?, s.overview?, s.release_date)))
            .map(|(id, title, desc, year)| {
                let year = year
                    .as_ref()
                    .and_then(|y| y.split_once('-'))
                    .and_then(|y| y.0.parse().ok());

                Movie {
                    id,
                    title,
                    desc,
                    year,

                    db: self.id_name(),
                    style,
                }
            })
            .collect::<Vec<_>>())
    }
}
