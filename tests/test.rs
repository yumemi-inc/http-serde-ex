#[test]
fn roundtrip() {
    use http::{HeaderMap, HeaderValue};
    use http::{Method, StatusCode, Uri};
    use std::io;

    let mut map = HeaderMap::new();
    map.insert("hey", HeaderValue::from_static("ho"));
    map.insert("foo", HeaderValue::from_static("bar"));
    map.append("multi-value", HeaderValue::from_static("multi"));
    map.append("multi-value", HeaderValue::from_static("valued"));

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::header_map")] HeaderMap,
        #[serde(with = "http_serde::uri")] Uri,
        #[serde(with = "http_serde::method")] Method,
        #[serde(with = "http_serde::status_code")] StatusCode,
    );

    let wrapped = Wrap(
        map,
        "http://example.com/".parse().unwrap(),
        Method::PUT,
        StatusCode::NOT_MODIFIED,
    );
    let json = serde_json::to_string(&wrapped).unwrap();
    let yaml = serde_yaml::to_string(&wrapped).unwrap();
    let bin = bincode::serialize(&wrapped).unwrap();
    assert_eq!(
        "[{\"hey\":\"ho\",\"foo\":\"bar\",\"multi-value\":[\"multi\",\"valued\"]},\"http://example.com/\",\"PUT\",304]",
        &json
    );
    assert_eq!(
        "---\n- hey: ho\n  foo: bar\n  multi-value:\n    - multi\n    - valued\n- \"http://example.com/\"\n- PUT\n- 304",
        &yaml
    );
    let back_js_str: Wrap = serde_json::from_str(&json).unwrap();
    let back_js_reader: Wrap = serde_json::from_reader(io::Cursor::new(json.as_bytes())).unwrap();
    let back_yaml_str: Wrap = serde_yaml::from_str(&yaml).unwrap();
    let back_yaml_reader: Wrap = serde_yaml::from_reader(io::Cursor::new(yaml.as_bytes())).unwrap();
    let back_bin: Wrap = bincode::deserialize(&bin).unwrap();

    for back in [
        back_js_str,
        back_js_reader,
        back_yaml_str,
        back_yaml_reader,
        back_bin,
    ]
    .iter()
    {
        assert_eq!(back.0.get("hey").map(|s| s.as_bytes()).unwrap(), b"ho");
        assert_eq!(back.0.get("foo").map(|s| s.as_bytes()).unwrap(), b"bar");
        assert_eq!(
            back.0
                .get_all("multi-value")
                .iter()
                .map(|v| v.to_str().unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
            &["multi", "valued"][..]
        );

        assert_eq!(&back.1.to_string(), "http://example.com/");
        assert_eq!(back.2, Method::PUT);
        assert_eq!(back.3, StatusCode::NOT_MODIFIED);
    }
}
