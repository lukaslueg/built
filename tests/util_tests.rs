extern crate built;
extern crate semver;

const DEPS: [(&'static str, &'static str); 2] = [("Foo", "1.2.3-alpha"), ("Bar", "0.1.3-456")];


#[test]
fn parse_versions() {
    let v = DEPS;
    let parsed = built::util::parse_versions(&v).collect::<Vec<_>>();
    assert_eq!(parsed,
               vec![("Foo",
                     semver::Version {
                         major: 1,
                         minor: 2,
                         patch: 3,
                         pre: vec![semver::Identifier::AlphaNumeric("alpha".to_owned())],
                         build: vec![],
                     }),
                    ("Bar",
                     semver::Version {
                         major: 0,
                         minor: 1,
                         patch: 3,
                         pre: vec![semver::Identifier::Numeric(456)],
                         build: vec![],
                     })]);

    assert!(built::util::parse_versions(&v)
        .any(|(name, ver)| name == "Bar" && ver < semver::Version::parse("0.1.4").unwrap()));
}
