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
    Object(String),
}

impl std::str::FromStr for Kind {
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
