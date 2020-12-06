use keyvalues::KeyValues;

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

// #[test]
// fn parse_nested_keyvalues() {

// }

#[test]
fn parse_vmt_file() {
    let kv = KeyValues::from_file("tests/resources/spray.vmt").unwrap();


    let mut kv2 = KeyValues::new();

    let mut subkey = KeyValues::new();
    subkey.add_value("$basetexture", "vgui\\logos\\spray_headshot");
    subkey.add_value("$translucent", "1");
    subkey.add_value("$decal", "1");
    subkey.add_value("$decalscale", "0.250");

    kv2.add_subkey(&"LightmappedGeneric".to_string(), &subkey);

    assert_eq!(kv, kv2);
}