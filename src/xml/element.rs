use super::{Kind, Occurence};

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub name: String,
    pub nillable: Option<String>,

    #[serde(default)]
    pub min_occurs: Occurence,
    #[serde(default)]
    pub max_occurs: Occurence,

    #[serde(rename = "type")]
    pub kind: Option<Kind>,
}
