use crate::{write_str_variable, write_variable};
use std::{fs, io, path};

fn get_build_deps(manifest_location: &path::Path) -> io::Result<Vec<(String, String)>> {
    use io::Read;

    let mut lock_buf = String::new();
    fs::File::open(manifest_location.join("Cargo.lock"))?.read_to_string(&mut lock_buf)?;
    Ok(parse_dependencies(&lock_buf))
}

fn parse_dependencies(lock_toml_buf: &str) -> Vec<(String, String)> {
    let lockfile: cargo_lock::Lockfile = lock_toml_buf.parse().expect("Failed to parse lockfile");
    let mut deps = Vec::new();

    for package in lockfile.packages {
        deps.push((package.name.to_string(), package.version.to_string()));
    }
    deps.sort_unstable();
    deps
}

pub fn write_dependencies(manifest_location: &path::Path, mut w: &fs::File) -> io::Result<()> {
    use io::Write;

    let deps = get_build_deps(manifest_location)?;
    write_variable!(
        w,
        "DEPENDENCIES",
        format!("[(&str, &str); {}]", deps.len()),
        format!("{deps:?}"),
        "An array of effective dependencies as documented by `Cargo.lock`."
    );
    write_str_variable!(
        w,
        "DEPENDENCIES_STR",
        deps.iter()
            .map(|(n, v)| format!("{n} {v}"))
            .collect::<Vec<_>>()
            .join(", "),
        "The effective dependencies as a comma-separated string."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_deps() {
        let lock_toml_buf = r#"
            [root]
            name = "foobar"
            version = "1.0.0"
            dependencies = [
                "normal_dep 1.2.3",
                "local_dep 4.5.6",
            ]

            [[package]]
            name = "normal_dep"
            version = "1.2.3"
            dependencies = [
                "dep_of_dep 7.8.9",
            ]

            [[package]]
            name = "local_dep"
            version = "4.5.6"

            [[package]]
            name = "dep_of_dep"
            version = "7.8.9""#;
        let deps = super::parse_dependencies(lock_toml_buf);
        assert_eq!(
            deps,
            [
                ("dep_of_dep".to_owned(), "7.8.9".to_owned()),
                ("local_dep".to_owned(), "4.5.6".to_owned()),
                ("normal_dep".to_owned(), "1.2.3".to_owned()),
            ]
        );
    }
}
