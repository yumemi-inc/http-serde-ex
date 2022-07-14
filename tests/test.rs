#[test]
fn roundtrip() {
    use http::{uri::Authority, Method, StatusCode, Uri, Version};
    use http::{HeaderMap, HeaderValue};
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
        #[serde(with = "http_serde::authority")] Authority,
        #[serde(with = "http_serde::version")] Version,
    );

    let wrapped = Wrap(
        map,
        "http://example.com/".parse().unwrap(),
        Method::PUT,
        StatusCode::NOT_MODIFIED,
        "example.com:8080".parse().unwrap(),
        Version::HTTP_2,
    );
    let json = serde_json::to_string(&wrapped).unwrap();
    let yaml = serde_yaml::to_string(&wrapped).unwrap();
    let cbor = serde_cbor::to_vec(&wrapped).unwrap();
    let bin = bincode::serialize(&wrapped).unwrap();
    assert_eq!(
        "[{\"hey\":\"ho\",\"foo\":\"bar\",\"multi-value\":[\"multi\",\"valued\"]},\"http://example.com/\",\"PUT\",304,\"example.com:8080\",\"HTTP/2.0\"]",
        &json
    );
    assert_eq!(
        "---\n- hey: ho\n  foo: bar\n  multi-value:\n    - multi\n    - valued\n- \"http://example.com/\"\n- PUT\n- 304\n- \"example.com:8080\"\n- HTTP/2.0\n",
        &yaml
    );
    let back_js_str: Wrap = serde_json::from_str(&json).unwrap();
    let back_js_reader: Wrap = serde_json::from_reader(io::Cursor::new(json.as_bytes())).unwrap();
    let back_yaml_str: Wrap = serde_yaml::from_str(&yaml).unwrap();
    let back_yaml_reader: Wrap = serde_yaml::from_reader(io::Cursor::new(yaml.as_bytes())).unwrap();
    let back_cbor: Wrap = serde_cbor::from_slice(&cbor).unwrap();
    let back_bin: Wrap = bincode::deserialize(&bin).unwrap();

    for back in [
        back_js_str,
        back_js_reader,
        back_yaml_str,
        back_yaml_reader,
        back_cbor,
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
        assert_eq!(&back.4.to_string(), "example.com:8080");
        assert_eq!(format!("{:?}", back.5), "HTTP/2.0");
    }
}

#[test]
fn option_header_map() {
    use http::{HeaderMap, HeaderValue};

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::option::header_map")]
        Option<HeaderMap>,
    );

    let mut map = HeaderMap::new();
    map.insert("Authorization", HeaderValue::from_str("Bearer").unwrap());

    let wrap = Wrap(Some(map));
    assert_eq!(r#"{"authorization":"Bearer"}"#.to_owned(), serde_json::to_string(&wrap).unwrap());

    let wrap = Wrap(None);
    assert_eq!("null".to_owned(), serde_json::to_string(&wrap).unwrap());
}

#[test]
fn option_status_code() {
    use http::StatusCode;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::option::status_code")]
        Option<StatusCode>,
    );

    let wrap = Wrap(Some(StatusCode::OK));
    assert_eq!("200".to_owned(), serde_json::to_string(&wrap).unwrap());

    let wrap = Wrap(None);
    assert_eq!("null".to_owned(), serde_json::to_string(&wrap).unwrap());
}

#[test]
fn option_method() {
    use http::Method;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::option::method")]
        Option<Method>,
    );

    let wrap = Wrap(Some(Method::POST));
    assert_eq!(r#""POST""#.to_owned(), serde_json::to_string(&wrap).unwrap());

    let wrap = Wrap(None);
    assert_eq!("null".to_owned(), serde_json::to_string(&wrap).unwrap());
}

#[test]
fn option_uri() {
    use http::Uri;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::option::uri")]
        Option<Uri>,
    );

    let wrap = Wrap(Some("https://example.com/".parse().unwrap()));
    assert_eq!(r#""https://example.com/""#.to_owned(), serde_json::to_string(&wrap).unwrap());

    let wrap = Wrap(None);
    assert_eq!("null".to_owned(), serde_json::to_string(&wrap).unwrap());
}

#[test]
fn option_authority() {
    use std::str::FromStr;
    use http::uri::Authority;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::option::authority")]
        Option<Authority>,
    );

    let wrap = Wrap(Some(Authority::from_str("example.com").unwrap()));
    assert_eq!(r#""example.com""#.to_owned(), serde_json::to_string(&wrap).unwrap());

    let wrap = Wrap(None);
    assert_eq!("null".to_owned(), serde_json::to_string(&wrap).unwrap());
}

#[test]
fn option_version() {
    use http::Version;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wrap(
        #[serde(with = "http_serde::option::version")]
        Option<Version>,
    );

    let wrap = Wrap(Some(Version::HTTP_11));
    assert_eq!(r#""HTTP/1.1""#.to_owned(), serde_json::to_string(&wrap).unwrap());

    let wrap = Wrap(None);
    assert_eq!("null".to_owned(), serde_json::to_string(&wrap).unwrap());
}
