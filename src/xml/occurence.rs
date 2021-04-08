#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Occurence {
    Bound(i64),
    Unbounded,
}

impl Default for Occurence {
    fn default() -> Self {
        Self::Unbounded
    }
}

impl std::str::FromStr for Occurence {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("unbounded") {
            Ok(Self::Unbounded)
        } else {
            s.parse::<i64>().map(Self::Bound).map_err(|_| {
                format!(
                    "Invalid Occurence `{}` must be a disctete int or `unbounded`",
                    s
                )
            })
        }
    }
}

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Occurence;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer or str `unbounded`")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Occurence::Bound(i64::from(value)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Occurence::Bound(i64::from(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Occurence::Bound(value))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        s.parse::<Occurence>().map_err(E::custom)
    }
}

impl<'de> serde::de::Deserialize<'de> for Occurence {
    fn deserialize<D>(deserializer: D) -> Result<Occurence, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor)
    }
}

struct Visitor;
