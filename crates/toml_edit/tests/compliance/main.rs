#![recursion_limit = "256"]
#![allow(clippy::dbg_macro)]

macro_rules! parse_value {
    ($s:expr) => {{
        let v = $s.parse::<crate::RustValue>();
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

use toml_edit_v1::DocumentMut as RustDocument;
use toml_edit_v1::Value as RustValue;
