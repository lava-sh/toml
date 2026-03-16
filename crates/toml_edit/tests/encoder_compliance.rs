mod decoder;
mod encoder;

fn main() {
    let valid_ext = {
        let path =
            std::path::Path::new("tests/fixtures/valid/ext/table/append-with-dotted-keys-1.toml");
        let name = path
            .strip_prefix("tests/fixtures")
            .unwrap()
            .to_owned()
            .into();
        let fixture = std::fs::read(path).unwrap().into();
        let expected = std::fs::read(path.with_extension("json")).unwrap().into();
        vec![toml_test_data::Valid {
            name,
            fixture,
            expected,
        }]
    };

    let encoder = encoder::Encoder;
    let decoder = decoder::Decoder;
    let mut harness = toml_test_harness::EncoderHarness::new(encoder, decoder);
    harness.version("1.0.0");
    harness.extend_valid(valid_ext);
    harness.test();
}
