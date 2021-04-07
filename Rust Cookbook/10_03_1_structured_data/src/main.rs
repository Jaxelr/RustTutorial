use serde_json::json;
use serde_json::{Value, Error};
use toml::{Value as TomlValue, de::Error as TomlError};

fn main() -> Result<(), Box <dyn std::error::Error>> {
    parse_json()?;
    parse_toml()?;
    Ok(())
}

fn parse_json() -> Result<(), Error> {

    let j = r#"{
        "userid": 103609,
        "verified": true,
        "access_privileges": [
          "user",
          "admin"
        ]
      }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
    "userid": 103609,
    "verified": true,
    "access_privileges": [
    "user",
    "admin"
    ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}

fn parse_toml() -> Result<(), TomlError> {
    let toml_content = r#"
    [package]
    name = "your_package"
    version = "0.1.0"
    authors = ["You! <you@example.org>"]

    [dependencies]
    serde = "1.0"
    "#;

    let package_info: TomlValue = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(package_info["package"]["name"].as_str(),
            Some("your_package"));

    Ok(())
}