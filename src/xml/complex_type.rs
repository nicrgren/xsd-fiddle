use super::{ComplexContent, Sequence};
use crate::{Field, ObjectImpl, TypeName};
use anyhow::anyhow;

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct ComplexType {
    pub name: String,

    #[serde(default)]
    pub r#abstract: bool,

    #[serde(rename = "sequence", default)]
    pub sequences: Vec<Sequence>,

    /// This is just bs data, can usually be ignored.
    #[serde(rename = "complexContent", default)]
    pub complex_contents: Vec<ComplexContent>,
}

impl std::convert::TryInto<ObjectImpl> for ComplexType {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<ObjectImpl, Self::Error> {
        if self.sequences.is_empty() {
            anyhow::bail!("`{}` has no sequences. Cannot create ObjectImpl", self.name);
        }

        let name = self.name;
        let (has_types, lacks_types): (Vec<_>, Vec<_>) = self
            .sequences
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("`{}` has no sequences. Cannot create ObjectImpl", &name))?
            .elements
            .into_iter()
            .partition(|el| el.kind.is_some());

        lacks_types
            .iter()
            .for_each(|el| eprintln!("Dropping typeless field `{}` in `{}`", el.name, &name));

        let fields = has_types
            .into_iter()
            .map(|el| Field {
                name: el.name,
                type_name: TypeName::from(el.kind.unwrap()),
            })
            .collect();

        let res = ObjectImpl { name, fields };

        Ok(res)
    }
}
