use core::fmt;
use std::convert::TryInto;

pub mod xml;

pub struct ModelSchema {
    pub models: Vec<ObjectImpl>,
}

impl ModelSchema {
    pub fn create_from_xml(xml: xml::Schema) -> Result<Self, anyhow::Error> {
        let mut models = Vec::with_capacity(200);

        let complex_types = xml.complex_types;

        for t in complex_types
            .into_iter()
            .filter(|t| !t.sequences.is_empty())
        {
            match t.try_into() {
                Ok(object_impl) => models.push(object_impl),
                Err(err) => println!("Failed to create ObjectImpl: {}", err),
            }
        }

        models.sort_by(|m1: &ObjectImpl, m2: &ObjectImpl| m1.name.cmp(&m2.name));

        Ok(Self { models })
    }
}

/// Represents a type parsed from XML spec.
/// The base from which code is generated.
pub enum TypeName {
    Primitive(Primitive),
    Array(Box<TypeName>),
    Object(String),
}

impl From<xml::Kind> for TypeName {
    fn from(kind: xml::Kind) -> Self {
        match kind {
            xml::Kind::Boolean => Self::Primitive(Primitive::Bool),
            xml::Kind::Int => Self::Primitive(Primitive::Int),
            xml::Kind::Long => Self::Primitive(Primitive::Long),
            xml::Kind::Double => Self::Primitive(Primitive::Double),
            xml::Kind::String => Self::Primitive(Primitive::String),
            xml::Kind::Base64Binary => Self::Object("Base64Binary".into()),
            xml::Kind::Guid => Self::Object("Guid".into()),
            xml::Kind::DateTime => Self::Object("DateTime".into()),
            xml::Kind::Array(kind) => Self::Array(Box::new(Self::from(*kind))),
            xml::Kind::Object(name) => Self::Object(name),
        }
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeName::Primitive(p) => write!(f, "{}", p),
            TypeName::Array(inner) => write!(f, "Vec<{}>", inner),
            TypeName::Object(name) => write!(f, "{}", name),
        }
    }
}

pub enum Primitive {
    Bool,
    Int,
    Long,
    Double,
    String,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool => f.write_str("boolean"),
            Self::Int => f.write_str("int"),
            Self::Long => f.write_str("long"),
            Self::Double => f.write_str("double"),
            Self::String => f.write_str("string"),
        }
    }
}

pub struct ObjectImpl {
    pub name: String,
    pub fields: Vec<Field>,
}

impl ObjectImpl {
    pub fn create_impl(&self) -> String {
        use std::io::Write;
        let mut buf = Vec::new();

        buf.write_fmt(format_args!("pub struct {} {{\n", self.name));
        for f in &self.fields {
            buf.write_fmt(format_args!("\t{}: {},\n", f.name, f.type_name));
        }

        buf.push(b'}');

        String::from_utf8_lossy(&buf).into_owned()
    }
}

pub struct Field {
    pub name: String,
    pub type_name: TypeName,
}

pub struct Enumeration {}

#[cfg(test)]
static BILLECTA_XSD: &str = include_str!("../api.xsd");

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_objects_from_billecta_xsd() {
        let xml_schema: xml::Schema =
            xml::de(BILLECTA_XSD.as_bytes()).expect("Parsing Billecta XSD");

        let models = ModelSchema::create_from_xml(xml_schema).expect("Creating ModelSchema");

        models.models.iter().for_each(|m| println!("{}", m.name));

        models
            .models
            .iter()
            .find(|m| m.name == "InvoiceAction")
            .iter()
            .for_each(|m| println!("{}", m.create_impl()));

        println!("Created {} models", models.models.len());
    }
}
