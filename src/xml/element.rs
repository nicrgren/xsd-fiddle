use super::{Kind, Occurence};

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub name: String,

    /// This is only present in 2 fields from billecta.
    /// Disregard and focus on min_occurs & max_occurs
    nillable: Option<String>,

    #[serde(default)]
    pub min_occurs: i8,
    #[serde(default)]
    pub max_occurs: Occurence,

    #[serde(rename = "type")]
    pub kind: Option<Kind>,
}

impl Element {
    pub fn is_optional(&self) -> bool {
        self.min_occurs == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_xml() {
        let s = r#"
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
   <xs:element minOccurs="0" maxOccurs="unbounded" name="string" nillable="true" type="xs:string" />
</xs:schema>
"#;
        let schema: crate::xml::Schema = crate::xml::de(s.as_bytes()).expect("Deserializing");
        let el = schema.all_elements().next().expect("One element");

        assert_eq!(el.min_occurs, 0);
        assert_eq!(el.max_occurs, Occurence::Unbounded);
        assert_eq!(el.name, "string");
        assert_eq!(el.kind, Some(Kind::String));
        assert!(el.is_optional());
    }
}
