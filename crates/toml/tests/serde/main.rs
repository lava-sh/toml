#![recursion_limit = "256"]
#![cfg(all(feature = "parse", feature = "display", feature = "serde"))]

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

use toml_v1::from_str;
use toml_v1::to_string;
use toml_v1::to_string_pretty;
use toml_v1::value::Date;
use toml_v1::value::Datetime;
use toml_v1::value::Time;
use toml_v1::Spanned;

use toml_v1::Table as SerdeDocument;
use toml_v1::Table as SerdeTable;
use toml_v1::Value as SerdeValue;

fn value_from_str<T>(s: &'_ str) -> Result<T, toml_v1::de::Error>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(toml_v1::de::ValueDeserializer::parse(s)?)
}

fn to_string_value<T>(value: &T) -> Result<String, toml_v1::ser::Error>
where
    T: serde::ser::Serialize + ?Sized,
{
    let mut output = String::new();
    let serializer = toml_v1::ser::ValueSerializer::new(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}
