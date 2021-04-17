use core::fmt;

mod code_formatter;
mod rust_fmt;
pub mod xml;

pub use code_formatter::CodeFormatter;

pub struct ModelSchema {
    pub implementations: Vec<Implementation>,

    // Tempory
    pub elements: Vec<xml::Element>,
}

impl ModelSchema {
    pub fn create_from_xml(xml: xml::Schema) -> Result<Self, anyhow::Error> {
        let mut implementations = Vec::new();

        let complex_types = xml.complex_types;
        let simple_types = xml.simple_types;
        let elements = xml.elements;

        for ct in complex_types
            .into_iter()
            .filter(|t| !t.sequences.is_empty())
        {
            match ct.into_object_impl() {
                Ok(object_impl) => implementations.push(Implementation::Object(object_impl)),
                Err(err) => eprintln!("Failed to create ObjectImpl: {}", err),
            }
        }

        for st in simple_types {
            match st.into_enum_impl() {
                Ok(enum_impl) => implementations.push(Implementation::Enum(enum_impl)),
                Err(err) => eprintln!("Failed to create EnumImpl: {}", err),
            }
        }

        implementations.sort();

        Ok(Self {
            implementations,
            elements,
        })
    }
}

/// Represents a type.
/// The base from which code is generated.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeName {
    Primitive(Primitive),
    Array(Box<TypeName>),
    Object(String),
}

impl TypeName {
    pub fn import_statement(&self) -> Option<&str> {
        match self {
            TypeName::Primitive(_) => None,
            TypeName::Array(ref inner) => inner.import_statement(),
            TypeName::Object(ref name) => Some(&name),
        }
    }
}

impl TypeName {
    pub fn object(s: impl Into<String>) -> Self {
        Self::Object(s.into())
    }

    pub fn array<T>(inner: T) -> Self
    where
        TypeName: From<T>,
    {
        Self::Array(Box::new(TypeName::from(inner)))
    }
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

impl From<Primitive> for TypeName {
    fn from(p: Primitive) -> Self {
        Self::Primitive(p)
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeName::Primitive(p) => write!(f, "{}", p),
            TypeName::Array(inner) => write!(f, "Array<{}>", inner),
            TypeName::Object(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, PartialEq, Eq)]
pub enum Implementation {
    Enum(EnumImpl),
    Object(ObjectImpl),
}

impl Implementation {
    pub fn name(&self) -> &str {
        match self {
            Self::Enum(inner) => inner.name.as_str(),
            Self::Object(inner) => inner.name.as_str(),
        }
    }
}

impl Ord for Implementation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name().cmp(other.name())
    }
}

impl PartialOrd for Implementation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

/// The current EnumImpl currently only supports Strings.
///
/// To implement other enums, such as ints or similar the kind would
/// have to be contained in this struct and the variants written out
/// in the serialize function of the enum impl.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumImpl {
    pub name: String,
    pub base: TypeName,
    pub variants: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectImpl {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    pub name: String,
    pub required: bool,
    pub type_name: TypeName,
}

impl Field {
    pub fn new(name: impl Into<String>, type_name: impl Into<TypeName>) -> Self {
        Self {
            name: name.into(),
            required: true,
            type_name: type_name.into(),
        }
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

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

        models
            .implementations
            .iter()
            .for_each(|m| println!("{}", m.name()));

        let mut fmt = super::rust_fmt::RustFmt;

        println!("\n\n\n\n=====");

        models
            .implementations
            .iter()
            .find(|m| m.name() == "CommonActionEvent")
            .iter()
            .for_each(|m| match m {
                Implementation::Object(obj) => {
                    let mut buf = String::new();
                    fmt.write_impl_file(&mut buf, &obj)
                        .expect("writing fmt file");
                    println!("{}", buf);
                }

                _ => println!("NOT AN OBJECT wTF"),
            });

        println!("Created {} models", models.implementations.len());
    }
}
