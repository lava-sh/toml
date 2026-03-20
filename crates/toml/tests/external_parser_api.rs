use toml::de::parser::array::on_array;
use toml::de::parser::document::document;
use toml::de::parser::inline_table::on_inline_table;
use toml::de::parser::Input;
use toml_parser::parser::parse_document;
use toml_parser::parser::parse_value;
use toml_parser::parser::Event;
use toml_parser::parser::EventKind;
use toml_parser::parser::EventReceiver;
use toml_parser::parser::ValidateWhitespace;
use toml_parser::Source;
use toml_parser::Span;
use winnow::stream::Stream as _;

#[test]
fn external_parser_api_is_usable() {
    let _: Span = Span::new_unchecked(0, 0);

    let source = Source::new("key = { nested = 1 }\narr = [1, 2]\n");
    let tokens = source.lex().into_vec();
    let mut events = Vec::<Event>::with_capacity(tokens.len());
    let _: &mut dyn EventReceiver = &mut events;
    let mut errors = Vec::new();
    let mut receiver = ValidateWhitespace::new(&mut events, source);
    parse_document(&tokens, &mut receiver, &mut errors);

    let mut input = Input::new(&events);
    let parsed = document(&mut input, source, &mut errors);
    assert!(parsed.get_ref().get("key").is_some());
    assert!(parsed.get_ref().get("arr").is_some());

    let source = Source::new("{ nested = 1 }");
    let tokens = source.lex().into_vec();
    let mut events = Vec::<Event>::with_capacity(tokens.len());
    let mut receiver = ValidateWhitespace::new(&mut events, source);
    parse_value(&tokens, &mut receiver, &mut errors);
    let mut input = Input::new(&events);
    let open = input.next_token().expect("inline table open event");
    assert_eq!(open.kind(), EventKind::InlineTableOpen);
    let parsed = on_inline_table(open, &mut input, source, &mut errors);
    assert!(parsed.get_ref().is_table());

    let source = Source::new("[1, 2]");
    let tokens = source.lex().into_vec();
    let mut events = Vec::<Event>::with_capacity(tokens.len());
    let mut receiver = ValidateWhitespace::new(&mut events, source);
    parse_value(&tokens, &mut receiver, &mut errors);
    let mut input = Input::new(&events);
    let open = input.next_token().expect("array open event");
    assert_eq!(open.kind(), EventKind::ArrayOpen);
    let parsed = on_array(open, &mut input, source, &mut errors);
    assert!(parsed.get_ref().is_array());
}
