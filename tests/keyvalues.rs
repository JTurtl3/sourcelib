use sourcelib::keyvalues::*;

#[test]
fn parse_keyvalues() {
    let kv = KeyValues::from_str(
"
key value
\"k e y\" valyoo!
"
    ).unwrap();

    let mut kv2 = KeyValues::new();
    kv2.add_value("key", "value");
    kv2.add_value("k e y", "valyoo!");

    assert_eq!(kv, kv2);
}

#[test]
fn parse_vmt_file() {
    let kv = KeyValues::from_file("tests/resources/spray.vmt").unwrap();


    let mut kv2 = KeyValues::new();

    let mut subkey = KeyValues::new();
    subkey.add_value("$basetexture", "vgui\\logos\\spray_headshot");
    subkey.add_value("$translucent", "1");
    subkey.add_value("$decal", "1");
    subkey.add_value("$decalscale", "0.250");

    kv2.add_subkey("LightmappedGeneric", &subkey);

    assert_eq!(kv, kv2);
}

#[test]
fn parse_lopsided_keyvalues() {
    let res = KeyValues::from_str("key value\nkeey vaalue\nkey?");
    assert_eq!(
        res,
        Err(Error { kind: ErrorKind::UnexpectedEOF, line: 3 })
    );
}

#[test]
fn parse_keyvalues_unclosed_brace() {
    let res = KeyValues::from_str("key value\nsubkey\n{\nkey value");
    assert_eq!(
        res,
        Err(Error { kind: ErrorKind::NoMatchingRightBrace, line: 3 })
    );
}

#[test]
fn parse_keyvalues_unexpected_brace() {
    let res = KeyValues::from_str("key value\n}\nkeey vaalue");
    assert_eq!(
        res,
        Err(Error {
            kind: ErrorKind::UnexpectedToken(TokenType::RightBrace),
            line: 2
        })
    );
}
