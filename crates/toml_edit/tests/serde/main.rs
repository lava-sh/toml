#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display"))]

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

mod de_enum;
mod de_errors;
mod de_key;
mod general;
mod ser_enum;
mod ser_key;
mod ser_tables_last;
mod ser_to_string;
mod ser_to_string_pretty;
mod spanned;

use serde_spanned::Spanned;
use toml_edit_v1::de::from_str;
use toml_edit_v1::ser::to_string;
use toml_edit_v1::ser::to_string_pretty;
use toml_edit_v1::Date;
use toml_edit_v1::Datetime;
use toml_edit_v1::Time;

use toml_types::Table as SerdeDocument;
use toml_types::Table as SerdeTable;
use toml_types::Value as SerdeValue;

fn value_from_str<T>(s: &'_ str) -> Result<T, toml_edit_v1::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(s.parse::<toml_edit_v1::de::ValueDeserializer>().unwrap())
}

fn to_string_value<T>(value: &T) -> Result<String, toml_edit_v1::ser::Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let serializer = toml_edit_v1::ser::ValueSerializer::new();
    let value = value.serialize(serializer)?;
    let output = value.to_string();
    Ok(output)
}
