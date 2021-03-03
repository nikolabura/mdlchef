use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct MdlMeme {
    /// Version should be MDL/1.1 for now
    pub version: String,
    /// Should always be "meme"
    pub r#type: String,
    /// Contains the base format
    #[serde(deserialize_with = "string_or_struct")]
    pub base: MdlBase,
    /// Either a string for single caption at default position,
    /// or an object with top text and bottom text
    #[serde(default = "empty_mdl_caption")]
    #[serde(deserialize_with = "string_or_struct")]
    pub caption: MdlCaption,
    /// Inserts object
    pub inserts: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct MdlBase {
    pub format: String,
}

impl FromStr for MdlBase {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MdlBase {
            format: s.to_string(),
        })
    }
    type Err = String;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MdlCaption {
    #[serde(alias = "bottom")]
    #[serde(alias = "south_text")]
    pub bottom_text: Option<String>,
    #[serde(alias = "top")]
    #[serde(alias = "north_text")]
    pub top_text: Option<String>,
    #[serde(alias = "middle")]
    pub center_text: Option<String>,
}

fn empty_mdl_caption() -> MdlCaption {
    MdlCaption {
        bottom_text: None,
        center_text: None,
        top_text: None,
    }
}

impl FromStr for MdlCaption {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MdlCaption {
            bottom_text: Some(s.to_string()),
            top_text: None,
            center_text: None,
        })
    }
    type Err = String;
}

/// source: https://serde.rs/string-or-struct.html
fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = String>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = String>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
