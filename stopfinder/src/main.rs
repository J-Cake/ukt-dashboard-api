#![feature(iter_intersperse)]

mod io;
mod schema;

use common::prelude::*;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

mod prelude {
    pub use crate::io::*;
    pub use crate::schema::*;
}

const STANDARD_API: &'static str = "https://www.efa-bw.de/mobidata-bw/";

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
        match prelude::prompt("Haltestelle: # ").await {
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
                    .query(&prelude::SearchParams::default().search(&search))
                    .send()
                    .await
                {
                    Ok(req) => req,
                    Err(err) => {
                        log::error!("Ein Fehler ist aufgetreten: {err:#?}");
                        continue;
                    }
                };

                if let Ok(stops) = req.json::<prelude::StopResponse>().await && stops.stops.points.len() > 0 {
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
                        let index = prelude::prompt(format!(" (1-{len}) # ", len=stops.stops.points.len())).await
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

async fn save_point(point: &prelude::Point) -> Result<()> {
    if !prelude::confirm(format!("Es wurde {point} gewählt. Speichern?\n (y/n) # ", point=point.name)).await {
        return Ok(());
    }

    log::warn!("Wird gespeichert");

    let path = loop {
        let path = PathBuf::from(prelude::prompt("Dateipfad angeben # ").await?);

        if let Ok(true) = tokio::fs::try_exists(&path).await {
            break path;
        }
    };

    let mut config: Config = tokio::fs::read_to_string(&path)
        .await
        .and_then(|str| toml::from_str(&str).map_err(Error::other))?;

    if !config.departure.is_empty() && prelude::confirm("Es sind bereits Haltestellen eingetragen. Wollen Sie die Liste leeren?\n (y/n) # ").await {
        config.departure.clear();
    }

    config.departure.push(DepartureConfig {
        point: point.ids.gid.clone(),
        show_next: Default::default(),
    });

    tokio::fs::write(&path, toml::to_string_pretty(&config).map_err(Error::other)?).await?;

    Ok(())
}

async fn write_config(config: &Config, path: impl AsRef<Path>) -> Result<()> {
    tokio::fs::write(path, toml::to_string(config).map_err(Error::other)?).await
}