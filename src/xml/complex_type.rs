use crate::{
    xml::{ComplexContent, Element, Sequence},
    Field, ObjectImpl, TypeName,
};
use anyhow::{anyhow, Result};

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

impl ComplexType {
    pub fn into_object_impl(self) -> Result<ObjectImpl> {
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
            .map(|el: Element| {
                let required = !el.is_optional();
                let name = el.name;
                Field {
                    name,
                    required,
                    type_name: TypeName::from(el.kind.unwrap()),
                }
            })
            .collect();

        let res = ObjectImpl { name, fields };

        Ok(res)
    }
}
