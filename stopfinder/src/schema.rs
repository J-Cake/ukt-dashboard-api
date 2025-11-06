use serde::{Deserialize, Serialize};

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
        pub stops: pub struct StopFinder {
            pub points: Vec<pub struct Point {
                pub name: String,
                pub object: String,
                #[serde(rename = "ref")]
                pub ids: pub struct IDs {
                    pub gid: String,
                    pub id: String,
                    pub place: String,
                }
            }>
        }
    }
}