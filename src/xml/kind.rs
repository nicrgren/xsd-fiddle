use std::{fmt, str};

/// The Type of a field, renamed to Kind as to not conflict with
/// reserved named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Boolean,
    Int,
    Long,
    Double,
    String,
    Base64Binary,
    Guid,
    DateTime,
    Array(Box<Kind>),
    Object(String),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => f.write_str("boolean"),
            Self::Int => f.write_str("int"),
            Self::Long => f.write_str("long"),
            Self::Double => f.write_str("double"),
            Self::String => f.write_str("string"),
            Self::Base64Binary => f.write_str("Base64Binary"),
            Self::Guid => f.write_str("Guid"),
            Self::DateTime => f.write_str("DateTime"),
            Self::Array(inner_kind) => write!(f, "Array<{}>", &inner_kind),
            Self::Object(name) => f.write_str(&name),
        }
    }
}

impl str::FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("guid") => Ok(Self::Guid),
            _ if s.eq_ignore_ascii_case("xs:int") => Ok(Self::Int),
            _ if s.eq_ignore_ascii_case("xs:long") => Ok(Self::Long),
            _ if s.eq_ignore_ascii_case("xs:string") => Ok(Self::String),
            _ if s.eq_ignore_ascii_case("xs:double") => Ok(Self::Double),
            _ if s.eq_ignore_ascii_case("xs:base64Binary") => Ok(Self::Base64Binary),
            _ if s.eq_ignore_ascii_case("xs:dateTime") => Ok(Self::DateTime),
            _ if s.eq_ignore_ascii_case("xs:boolean") => Ok(Self::Boolean),
            s if s.starts_with("ArrayOf") => {
                let inner_kind = Self::from_str(s.trim_start_matches("ArrayOf"))
                    .map_err(|err| format!("Parsing Array element from `{}`: {}", s, err))?;
                Ok(Self::Array(Box::new(inner_kind)))
            }
            s if s.is_empty() => Err("Empty Object Kind".into()),
            s if !s.starts_with("xs") => {
                if s.chars().next().unwrap().is_lowercase() {
                    Err(format!("Non upcased Object: `{}`", s))
                } else {
                    Ok(Self::Object(s.into()))
                }
            }
            s => Err(format!("Unknown kind `{}`", s)),
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Kind;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a str")
    }
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.parse::<Kind>().map_err(E::custom)
    }
}

impl<'de> serde::de::Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Kind, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor)
    }
}

struct Visitor;
