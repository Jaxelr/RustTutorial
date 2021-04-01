use semver::{Identifier, Version, VersionReq, SemVerError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed_version = Version::parse("0.2.6")?;

    parse_version(parsed_version)?;

    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str)?;

    parse_complex_version(&version_str, parsed_version)?;

    parse_prerelease()?;

    test_matching_max_version()?;

    Ok(())
}

fn parse_version(mut version: Version) -> Result<(), SemVerError> {
    assert_eq!(
        version,
        Version {
            major: 0,
            minor: 2,
            patch: 6,
            pre: vec![],
            build: vec![],
        }
    );

    version.increment_patch();
    assert_eq!(version.to_string(), "0.2.7");
    println!("New patch release: v{}", version);

    version.increment_minor();
    assert_eq!(version.to_string(), "0.3.0");
    println!("New minor release: v{}", version);

    version.increment_major();
    assert_eq!(version.to_string(), "1.0.0");
    println!("New major release: v{}", version);

    Ok(())
}

fn parse_complex_version(version_str : &str, parsed_version: Version) -> Result<(), SemVerError> {

    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: vec![Identifier::Numeric(125)],
            build: vec![],
        }
    );
    assert_eq!(
        parsed_version.build,
        vec![Identifier::AlphaNumeric(String::from("g72ee7853"))]
    );

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}

fn parse_prerelease() -> Result<(), SemVerError> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert!(version_1.is_prerelease());
    assert!(!version_2.is_prerelease());

    Ok(())
}

fn find_max_matching_version<'a, I>(version_req_str: &str, iterable: I) -> Result<Option<Version>, Box<dyn std::error::Error>> where I: IntoIterator<Item = &'a str> {
    let vreq = VersionReq::parse(version_req_str)?;

    Ok(
        iterable
            .into_iter()
            .filter_map(|s| Version::parse(s).ok())
            .filter(|s| vreq.matches(s))
            .max(),
    )
}

fn test_matching_max_version() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
        Some(Version::parse("1.0.0")?)
    );

    assert_eq!(
        find_max_matching_version(
            ">1.2.3-alpha.3",
            vec![
                "1.2.3-alpha.3",
                "1.2.3-alpha.4",
                "1.2.3-alpha.10",
                "1.2.3-beta.4",
                "3.4.5-alpha.9",
            ]
        )?,
        Some(Version::parse("1.2.3-beta.4")?)
    );

    Ok(())
}
