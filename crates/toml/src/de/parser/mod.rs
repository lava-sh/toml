#![allow(clippy::type_complexity)]

use serde_spanned::Spanned;
#[cfg(not(feature = "unbounded"))]
use toml_parser::parser::RecursionGuard as InternalRecursionGuard;
use toml_parser::parser::ValidateWhitespace as InternalValidateWhitespace;

pub use array::on_array;
pub use dearray::DeArray;
pub use detable::DeTable;
pub use devalue::DeFloat;
pub use devalue::DeInteger;
pub use devalue::DeString;
pub use devalue::DeValue;
pub use document::descend_path;
pub use document::document;
pub use document::get_key_span;
pub use inline_table::on_inline_table;
pub use key::on_key;
pub use toml_parser::parser::Event;
pub use toml_parser::parser::EventKind;
pub use toml_parser::parser::EventReceiver;
#[cfg(not(feature = "unbounded"))]
pub use toml_parser::parser::RecursionGuard;
pub use toml_parser::parser::ValidateWhitespace;
pub use toml_parser::ErrorSink;
pub use toml_parser::ParseError;
pub use toml_parser::Source;
pub use toml_parser::Span;
pub use value::on_scalar;
pub use value::value;

use crate::alloc_prelude::*;

pub mod array;
pub(crate) mod dearray;
#[cfg(feature = "debug")]
pub(crate) mod debug;
pub(crate) mod detable;
pub(crate) mod devalue;
pub mod document;
pub mod inline_table;
pub mod key;
pub mod value;

pub type Input<'i> = winnow::stream::TokenSlice<'i, Event>;

pub fn parse_document<'i>(source: Source<'i>, errors: &mut dyn ErrorSink) -> Spanned<DeTable<'i>> {
    let tokens = source.lex().into_vec();

    let mut events = Vec::with_capacity(tokens.len());
    let mut receiver = InternalValidateWhitespace::new(&mut events, source);
    #[cfg(not(feature = "unbounded"))]
    let mut receiver = InternalRecursionGuard::new(&mut receiver, LIMIT);
    #[cfg(not(feature = "unbounded"))]
    let receiver = &mut receiver;
    #[cfg(feature = "unbounded")]
    let receiver = &mut receiver;
    toml_parser::parser::parse_document(&tokens, receiver, errors);

    let mut input = Input::new(&events);
    let doc = document(&mut input, source, errors);
    doc
}

pub fn parse_value<'i>(source: Source<'i>, errors: &mut dyn ErrorSink) -> Spanned<DeValue<'i>> {
    let tokens = source.lex().into_vec();

    let mut events = Vec::with_capacity(tokens.len());
    let mut receiver = InternalValidateWhitespace::new(&mut events, source);
    #[cfg(not(feature = "unbounded"))]
    let mut receiver = InternalRecursionGuard::new(&mut receiver, LIMIT);
    #[cfg(not(feature = "unbounded"))]
    let receiver = &mut receiver;
    #[cfg(feature = "unbounded")]
    let receiver = &mut receiver;
    toml_parser::parser::parse_value(&tokens, receiver, errors);

    let mut input = Input::new(&events);
    let value = value(&mut input, source, errors);
    value
}

#[cfg(not(feature = "unbounded"))]
const LIMIT: u32 = 80;

pub(crate) mod prelude {
    pub(crate) use toml_parser::parser::EventKind;
    pub(crate) use toml_parser::ErrorSink;
    pub(crate) use toml_parser::ParseError;
    pub(crate) use winnow::stream::Stream as _;

    pub(crate) type Input<'i> = super::Input<'i>;

    #[cfg(feature = "debug")]
    pub(crate) use super::debug::trace;
    #[cfg(feature = "debug")]
    pub(crate) use super::debug::TraceScope;
}
