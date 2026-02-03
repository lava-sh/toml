#![recursion_limit = "256"]
#![allow(clippy::dbg_macro)]
#![cfg(all(feature = "parse", feature = "display", feature = "serde"))]

macro_rules! parse_value {
    ($s:expr) => {{
        let v = $s.parse::<toml_v1::Value>();
        assert!(
            v.is_ok(),
            "Failed with `{}` when parsing:
```
{}
```
",
            v.unwrap_err(),
            $s
        );
        v.unwrap()
    }};
}

mod invalid;
mod parse;

use toml_v1::Table as RustDocument;
use toml_v1::Value as RustValue;
