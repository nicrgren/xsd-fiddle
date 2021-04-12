use crate::{xml, TypeName};
use anyhow::anyhow;

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct SimpleType {
    pub name: String,

    #[serde(rename = "restriction", default)]
    pub restrictions: Vec<Restriction>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub base: xml::Kind,

    #[serde(rename = "enumeration", default)]
    pub enumerations: Vec<Enumeration>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct Enumeration {
    pub value: String,
}

impl SimpleType {
    pub fn into_enum_impl(self) -> anyhow::Result<crate::EnumImpl> {
        let name = self.name;
        let restriction = self
            .restrictions
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("`{}` has no restrictions", &name))?;

        let base = TypeName::from(restriction.base);
        let variants = restriction
            .enumerations
            .into_iter()
            .map(|en| en.value)
            .collect();
        Ok(crate::EnumImpl {
            name,
            base,
            variants,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_into_enum_impl() {
        let s = r#"
<xs:schema elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="AccountingExportDateSelectionType">
    <xs:restriction base="xs:string">
      <xs:enumeration value="Unknown" />
      <xs:enumeration value="EventDate" />
      <xs:enumeration value="TransactionDate" />
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"#;

        let schema: xml::Schema = xml::de(s.as_bytes()).expect("Deserializing");
        let simple_type = schema
            .simple_types
            .into_iter()
            .next()
            .expect("1 SimpleType");

        let ei: crate::EnumImpl = simple_type
            .into_enum_impl()
            .expect("Converting SimpleType to EnumImpl");

        assert_eq!(ei.name, "AccountingExportDateSelectionType");
        assert_eq!(
            ei.base,
            crate::TypeName::Primitive(crate::Primitive::String)
        );

        assert_eq!(ei.variants, vec!["Unknown", "EventDate", "TransactionDate"]);
    }
}
