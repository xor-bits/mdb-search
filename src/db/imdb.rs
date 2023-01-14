use super::MovieDatabase;
use crate::{movie::Movie, Style};
use serde::Deserialize;

//

pub struct InternetMovieDatabase;

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

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct TitleData {
//     // id: String,
//     year: String,
// }

//

impl MovieDatabase for InternetMovieDatabase {
    fn name(&self) -> &'static str {
        "IMDb"
    }

    fn full_name(&self) -> &'static str {
        "Internet Movie Database"
    }

    fn id_name(&self) -> &'static str {
        "imdbid"
    }

    fn query(&self, style: Style, api_key: &str, search: &str) -> Result<Vec<Movie>, String> {
        // url encode the search text
        let search = urlencoding::encode(search);

        // query IMDb
        let res = reqwest::blocking::get(format!(
            "https://imdb-api.com/en/API/SearchMovie/{api_key}/{search}",
        ))
        .and_then(|r| r.json::<SearchData>())
        .map_err(|err| err.to_string())?;

        // api error
        if !res.error_message.is_empty() {
            return Err(res.error_message);
        }

        Ok(res
            .results
            .into_iter()
            .flat_map(|i| i.into_iter())
            .filter(|s| s.result_type == "Movie")
            .map(|s| {
                let year = if let (Some("("), Some(year), Some(")")) = (
                    s.description.get(0..1),
                    s.description.get(1..5),
                    s.description.get(5..6),
                ) {
                    year.parse().ok()
                } else {
                    None
                };

                let year = if let Some(y) = s.description.get(0..4).and_then(|s| s.parse().ok()) {
                    Some(y)
                } else {
                    year
                };

                Movie {
                    id: s.id,
                    title: s.title,
                    desc: s.description,
                    year,

                    db: self.id_name(),
                    style,
                }
            })
            .collect::<Vec<_>>())
    }
}
