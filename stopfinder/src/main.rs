#![feature(iter_intersperse)]

use std::ascii::AsciiExt;
use common::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

const STANDARD_API: &'static str = "https://www.efa-bw.de/mobidata-bw/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    #[serde(rename = "outputFormat")]
    output_format: &'static str,
    type_sf: &'static str,

    #[serde(rename = "anyMaxSizeHitList")]
    pub matches: u32,

    #[serde(rename = "name_sf")]
    pub search: String,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            output_format: "JSON",
            type_sf: "any",
            matches: 50,
            search: "".to_string(),
        }
    }
}

impl SearchParams {
    pub fn matches(mut self, matches: u32) -> Self {
        self.matches = matches;
        return self;
    }

    pub fn search(mut self, search: impl AsRef<str>) -> Self {
        self.search = search.as_ref().to_owned();
        return self;
    }
}

nestify::nest! {
    #[derive(Debug, Clone, Serialize, Deserialize)]*
    pub struct StopResponse {
        #[serde(rename = "stopFinder")]
        stops: struct StopFinder {
            points: Vec<struct Point {
                name: String,
                object: String,
                #[serde(rename = "ref")]
                ids: struct IDs {
                    gid: String,
                    id: String,
                    place: String,
                }
            }>
        }
    }
}

#[tokio::main]
pub async fn main() {
    env_logger::init();

    println!(
        r#"
Willkommen zum Haltestellensuchassistentsprogramm der Azubitafel
Geben Sie den Namen einer Haltestelle ein.
"#
    );

    let api_url: reqwest::Url = std::env::var("BUSFAHRTEN_API")
        .unwrap_or(STANDARD_API.to_string())
        .parse()
        .expect("Ungültige API URL");

    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("HTTP Client konnte nicht erstellt werden.");

    'outer: loop {
        match prompt("Haltestelle: # ").await {
            Ok(search) => {
                let mut api_url = api_url.clone();
                api_url.set_path(
                    PathBuf::from(api_url.path())
                        .join("XML_STOPFINDER_REQUEST")
                        .to_str()
                        .expect("Ungültiger Pfad angegeben"),
                );

                let req = match client
                    .request(reqwest::Method::GET, api_url)
                    .query(&SearchParams::default().search(&search))
                    .send()
                    .await
                {
                    Ok(req) => req,
                    Err(err) => {
                        log::error!("Ein Fehler ist aufgetreten: {err:#?}");
                        continue;
                    }
                };

                if let Ok(stops) = req.json::<StopResponse>().await && stops.stops.points.len() > 0 {
                    let matches = stops
                        .stops
                        .points
                        .iter()
                        .enumerate()
                        .map(|(a, point)| format!("{id}: {name}", id = a + 1, name = &point.name))
                        .intersperse("\n  ".to_owned())
                        .collect::<String>();

                    println!("Es wurden folgende Treffer gefunden: \n  {matches}\nBitte geben Sie die Nummer der gewünschten Haltestelle an");

                    loop {
                        let index = prompt(format!("\n (1-{len}) # ", len=stops.stops.points.len())).await
                            .and_then(|str| str.parse::<usize>().map_err(Error::other));

                        if let Ok(Some(point)) = index.map(|index| stops.stops.points.get(index - 1)) {
                            save_point(point).await;

                            log::info!("Fertig");

                            std::process::exit(0);
                        }
                    }
                } else {
                    log::warn!("Keine Treffer für '{search}'")
                }
            }
            Err(err) => {
                log::error!("{err:#?}");
                continue;
            }
        }
    }
}

async fn prompt(msg: impl AsRef<str>) -> Result<String> {
    let mut read = tokio::io::BufReader::new(tokio::io::stdin());

    tokio::io::stderr()
        .write_all(msg.as_ref().as_bytes())
        .await?;

    let mut str = String::new();
    read.read_line(&mut str).await?;

    Ok(str.trim().to_string())
}

async fn save_point(point: &Point) {
    match prompt(format!("Es wurde {point} gewählt. Zur Konfigurationsdatei schreiben?\n (J/n) # ", point=point.name))
        .await
        .map(|i| i.to_ascii_lowercase()) {
        Ok(str) if str == "y" || str == "j" => {
            log::info!("Wird gespeichert");

            let mut config = match common::get_config().await {
                Err(err) => {
                    log::error!("Konfigurationsdatei konnte nicht gelesen werden. {err:?}");
                    match prompt("Möchten Sie stattdessen eine neue Datei anlegen?\n (J/n) # ")
                        .await
                        .map(|i| i.to_ascii_lowercase()) {
                        Ok(str) if str == "j" || str == "y" || str.is_empty() => Config::default(),
                        _ => return
                    }
                },
                Ok(config) => config
            };

            config.departure.point = Some(point.ids.gid.clone());

            if let Some(ref path) = config.source {
                match prompt(format!("Die Konfigurationsdatei zurück zu {path:?} schreiben oder anderen Pfad angeben?\n (Z/a) # ")).await
                    .map(|i| i.to_ascii_lowercase()) {
                    Ok(str) if str == "z" || str.is_empty() => {
                        if let Err(err) = write_config(&config, path).await {
                            log::error!("Fehler beim Schreiben der Datei: {err:?}");
                            return;
                        };
                        return;
                    },
                    _ => {}
                }
            }

            loop {
                if let Ok(path) = prompt("Bitte Pfad der angeben # ").await
                    .map(PathBuf::from) {
                    if let Err(err) = write_config(&config, path).await {
                        log::error!("Fehler beim Schreiben der Datei: {err:?}");
                        return;
                    };
                    return;
                }
            }
        },
        _ => return
    }
}

async fn write_config(config: &Config, path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::write(path, toml::to_string(config).map_err(Error::other)?).await
}